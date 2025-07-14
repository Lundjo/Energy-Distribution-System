mod connection;
use connection::send_message;
use std::error::Error;
use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut input = String::new();

    loop {
        input.clear();
        
        stdin.read_line(&mut input).await?;
        let message = input.trim();
        
        match send_message(message).await {
            Ok(_) => println!("Message sent successfully: '{}'", message),
            Err(e) => eprintln!("Message could not be sent: {}", e),
        }
    }
}
