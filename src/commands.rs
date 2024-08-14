use crate::{api::{get_all_exchange_rates, get_exchange_rate, convert}, config::{write_config, remove_config, read_config}};
use std::process::exit;  

enum Command {
    Help,
    GetAllRates,
    GetRate,
    Convert,
    Key,
    Exit,
}

fn match_command(input: &str) -> Option<Command> {
    match input {
        "help" => Some(Command::Help), // "help" is the command to get a list of commands
        "all" => Some(Command::GetAllRates), // "all" is the command to get all exchange rates
        "rates" => Some(Command::GetAllRates), // "rates" is an alias for "all"
        "list" => Some(Command::GetAllRates), // "list" is an alias for "all"
        "rate" => Some(Command::GetRate), // "rate" is the command to get a specific exchange rate
        "convert" => Some(Command::Convert), // "convert" is the command to convert an amount from one currency to another
        "key" => Some(Command::Key), // "key" is the command to set the API key
        "exit" => Some(Command::Exit), // "exit" is the command to exit the program
        _ => None,
    }
}

pub async fn new_command(command: &str, args: Option<Vec<&str>>) {
    let command = match_command(command);
    let args = args.unwrap_or_default();
    match command {
        Some(Command::Help) => {
            println!("==== Help ====");
            println!("Available commands:");
            println!("help - Get a list of commands");
            println!("all [BASE_CURRENCY] - Get all exchange rates for base currency (default is USD)");
            println!("rate [CURRENCY_1] [CURRENCY_2] - Get the exchange rate between two currencies");
            println!("convert [CURRENCY_FROM] [CURRENCY_TO] [AMOUNT] - Convert an amount from one currency to another");
            println!("key [view/set/remove] [API_KEY] - View, set, or remove the API key");
            println!("exit - Exit the program");
            println!("==============");
        }
        Some(Command::GetAllRates) => {
            let base_currency = if args.is_empty() {
                println!("Base currency not provided. Using USD as the base currency.");
                "USD"
            } else {
                args[0]
            };
            println!("Getting all exchange rates for {}...", base_currency);
            let exchange_rate_response = get_all_exchange_rates(base_currency).await;
            match exchange_rate_response {
                Ok(response) => {
                    println!("Exchange rates for {}:", response.base_code);
                    for (currency, rate) in response.conversion_rates.as_object().unwrap() {
                        println!("{}: {}", currency, rate);
                    }
                }
                Err(e) => {
                    println!("Error getting exchange rates: {}", e);
                }
            }
        }
        Some(Command::GetRate) => {
            if args.len() != 2 {
                println!("Please provide two currencies to get the exchange rate between.");
                println!("[Example: rate USD EUR]");
            } else {
                println!("Getting the exchange rate between {} and {}...", args[0], args[1]);
                let exchange_rate_response = get_exchange_rate(args[0], args[1]).await;
                match exchange_rate_response {
                    Ok(response) => {
                        println!("Exchange rate from {} to {}: {}", response.base_code, response.target_code, response.conversion_rate);
                    }
                    Err(e) => {
                        println!("Error getting exchange rate: {}", e);
                    }
                }
            }
        }
        Some(Command::Convert) => {
            if args.len() < 3 {
                println!("Please provide a currency to convert from, a currency to convert to, and an amount to convert.");
                println!("[Example: convert USD EUR 100]");
            } else {
                let from_currency = args[0];
                let to_currency = args[1];
                let amount = match args[2].parse::<f64>() {
                    Ok(amount) => amount,
                    Err(_) => {
                        println!("Invalid amount provided. Please provide a valid number.");
                        return;
                    }
                };
                println!("Converting {} {} to {}...", amount, from_currency, to_currency);
                let conversion_response = convert(from_currency, to_currency, amount).await;
                match conversion_response {
                    Ok(response) => {
                        println!("{} {} is equal to {} {}.", amount, from_currency, response.conversion_result, to_currency);
                        println!("Exchange rate used: {}", response.conversion_rate);
                    }
                    Err(e) => {
                        println!("Error converting currency: {}", e);
                    }
                }

            }
        }

        Some(Command::Key) => {
            if args.is_empty() {
                println!("Please provide a command to view, set, or remove the API key.");
                println!("[Example: key view]");
            } else {
                match args[0] {
                    "view" => {
                        match read_config() {
                            Ok(config) => {
                                println!("API key: {}", config.api_key);
                            }
                            Err(e) => {
                                println!("Error reading API key: {}", e);
                            }
                        }
                    }
                    "set" => {
                        if args.len() < 2 {
                            println!("Please provide an API key to set.");
                            println!("[Example: key set YOUR_API_KEY]");
                        } else {
                            let api_key = args[1];
                            match write_config(api_key.to_string()) {
                                Ok(_) => {
                                    println!("API key set.");
                                }
                                Err(e) => {
                                    println!("Error setting API key: {}", e);
                                }
                            }
                        }
                    }
                    "remove" => {
                        match remove_config() {
                            Ok(_) => {
                                println!("API key removed.");
                            }
                            Err(e) => {
                                println!("Error removing API key: {}", e);
                            }
                        }
                    }
                    _ => {
                        println!("Command not recognized. Please provide a command to view, set, or remove the API key.");
                        println!("[Example: key view]");
                    }
                }
            }
        }
        Some(Command::Exit) => {
            println!("Exiting the program...");
            exit(0);
        }
        None => {
            println!("Command not recognized. Type help for a list of commands.");
        }
    }
}