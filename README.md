Currency-Converter&ensp;💰
==========================
This CLI program allows you to view real-time currency exchange rates and convert between them.
## API

- Uses [Exchange-RateAPI](https://www.exchangerate-api.com/) to get currency rates that are updated daily.

- Requires an active API key in order to work.
### How to get the API Key?
- Go to the [Exchange-RateAPI](https://www.exchangerate-api.com/) website
- Input your e-mail address and click "Get Free Key!"
- Input your password and click "Accept Terms & Create API Key!"
- Confirm your e-mail address by clicking link that was delivered to your e-mail
- You will now be redirected to the dashboard
- Your API Key will be displayed under "API Access" section

### Set the API Key
- Run the program
- Use command ```key set <API_KEY>``` and replace ```<API_KEY>``` with the key from ExchangeRate-API dashboard
## Installation
- Install [Rust](https://www.rust-lang.org/tools/install)
- Clone the repository using ```git clone https://github.com/exceedxo/currency-converter```
- Go into the created directory and run the command: ```cargo run``` or ```cargo run --release``` for the release build
## Usage
### Available commands
- help - Get a list of commands
- all [BASE_CURRENCY] - Get all exchange rates for base currency (default is USD)
- rate [CURRENCY_1] [CURRENCY_2] - Get the exchange rate between two currencies
- convert [CURRENCY_FROM] [CURRENCY_TO] [AMOUNT] - Convert an amount from one currency to another
- key [view/set/remove] [API_KEY] - View, set, or remove the API key
- exit - Exit the program
