use std::io::stdin;
use crate::commands::new_command;

pub async fn command_loop() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let split = input.trim().split_whitespace().collect::<Vec<&str>>();
        if split.is_empty() {
            continue;
        }
        let command = split[0];
        let args = if split.len() > 1 {
            Some(split[1..].to_vec())
        } else {
            None
        };
        new_command(command, args).await;
    }
}