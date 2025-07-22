use crate::connection;
use connection::send_message_to_renewables;
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

        let message = format!("0 {} {}", num1, num2);
        
        match send_message_to_renewables(&message).await {
            Ok(response) => println!("Server responded: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

pub async fn get_current_production() -> Result<(), Box<dyn Error>> {
    let message = format!("1");
        
    match send_message_to_renewables(&message).await {
        Ok(response) => println!("Server responded: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

pub async fn select_renewables() -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let op = loop {
        println!("Choose operation (1 for change number of renewables, 2 for current power generation): ");
        match lines.next_line().await? {
            Some(input) => match input.trim().parse::<i32>() {
                Ok(num) if num == 1 || num == 2 => break num,
                Ok(_) => eprintln!("Please enter either 1 or 2!"),
                Err(_) => eprintln!("Bad input, please enter a valid number!"),
            },
            None => return Ok(()),
        }
    };

    if op == 1 {
        change_number_of_generators().await?;
    } else if op == 2 {
        get_current_production().await?;
    }

    Ok(())
}