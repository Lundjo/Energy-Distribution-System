use crate::connection;
use connection::{send_message_to_renewables, send_message2};
use std::error::Error;
use tokio::io::AsyncBufReadExt;

pub async fn change_number_of_generators() -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    loop {
        let num1 = loop {
            println!("Enter a number of wind turbines to change:");
            match lines.next_line().await? {
                Some(input) => match input.trim().parse::<i32>() {
                    Ok(num) => break num,
                    Err(_) => eprintln!("Bad input, please enter a valid number!"),
                },
                None => return Ok(()),
            }
        };

        let num2 = loop {
            println!("Enter a number of solar panels to change:");
            match lines.next_line().await? {
                Some(input) => match input.trim().parse::<i32>() {
                    Ok(num) => break num,
                    Err(_) => eprintln!("Bad input, please enter a valid number!"),
                },
                None => return Ok(()),
            }
        };

        let message = format!("{} {}", num1, num2);
        
        match send_message_to_renewables(&message).await {
            Ok(response) => println!("Server responded: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}