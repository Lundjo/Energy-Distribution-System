use crate::connection;
use connection::{send_message1, send_message2};
use std::error::Error;
use tokio::io::AsyncBufReadExt;

pub async fn input() -> Result<(), Box<dyn Error>> {
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();
        
        stdin.read_line(&mut input).await?;
        let message = input.trim();
        
        match send_message1(message).await {
            Ok(_) => println!("Message sent successfully: '{}'", message),
            Err(e) => eprintln!("Message could not be sent: {}", e),
        }
        match send_message2(message).await {
            Ok(_) => println!("Message sent successfully: '{}'", message),
            Err(e) => eprintln!("Message could not be sent: {}", e),
        }
    }
}