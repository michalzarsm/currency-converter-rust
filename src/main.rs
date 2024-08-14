use tokio::{task::spawn, signal::ctrl_c};
use user_input::command_loop;
use std::process::exit;

mod api;
mod user_input;
mod commands;
mod config;

#[tokio::main]
async fn main() {
    println!("Welcome to the Currency Converter!");
    println!("This program uses www.exchangerate-api.com to get the latest exchange rates.");
    println!("Type help for a list of commands.");

    spawn(command_loop());

    match ctrl_c().await {
        Ok(_) => {
            println!("Exiting the program...");
            exit(0);
        }
        Err(e) => {
            println!("Error exiting program: {}", e);
            exit(1);
        }
    }
}