use crate::connection::{send_message_to_hydro, send_message_to_renewables};
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub async fn change_number_of_generators() -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

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
        Ok(response) => {
            println!("Server responded: {}", response);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e.into())
        },
    }
}

pub async fn get_current_production_renewables() -> Result<f64, Box<dyn Error>> {
    let message = format!("1");
        
    let response = send_message_to_renewables(&message).await?;

    let parts: Vec<&str> = response.split_whitespace().collect();
    let wind = parts[0].parse::<f64>()?;
    let solar = parts[1].parse::<f64>()?;
    
    Ok(wind+solar)
}

pub async fn change_hydro_usage() -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let num = loop {
        println!("Change hydro plant power output:");
        match lines.next_line().await? {
            Some(input) => match input.trim().parse::<f64>() {
                Ok(num) => break num,
                Err(_) => eprintln!("Bad input, please enter a valid number!"),
            },
            None => return Ok(()),
        }
    };

    let message = format!("0 {}", num);
        
    match send_message_to_hydro(&message).await {
        Ok(response) => {
            println!("Server responded: {}", response);
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e.into())
        },
    }
}

pub async fn get_current_production_hydro() -> Result<f64, Box<dyn Error>> {
    let message = format!("1");
        
    let response = send_message_to_hydro(&message).await?;
    
    let value = response.parse::<f64>()?;
    Ok(value)
}

pub async fn select_operation(client: &mut Option<(String, tokio::net::TcpStream)>) -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let op = loop {
        println!("Choose operation - 1: Change hydro plant power output, 2: Change number of renewables, 3: Show client request");
        match lines.next_line().await? {
            Some(input) => match input.trim().parse::<i32>() {
                Ok(num) if num == 1 || num == 2 || num == 3 => break num,
                Ok(_) => eprintln!("Please enter either 1, 2 or 3!"),
                Err(_) => eprintln!("Bad input, please enter a valid number!"),
            },
            None => return Ok(()),
        }
    };

    if op == 1 {
        change_hydro_usage().await?;
    } else if op == 2{
        change_number_of_generators().await?;
    } else if op == 3 {
        if let Err(e) = user(client).await {
            eprintln!("Error in user function: {}", e);
        }
    }

    Ok(())
}

pub async fn user(client: &mut Option<(String, tokio::net::TcpStream)>) -> Result<(), Box<dyn Error>> {
    if let Some((msg, mut stream)) = client.take() {
        let required =  msg.parse::<f64>()?;
        let h = get_current_production_hydro().await?;
        let r = get_current_production_renewables().await?;
        let total = required -  h - r;

        let stdin = tokio::io::BufReader::new(tokio::io::stdin());
        let mut lines = stdin.lines();

        if total <= 0.0 {
            let op = loop {
                println!("Enough power can be supplied. Press 1 to allow user to turn on devices, 2 to deny request, 3 for main menu: ");
                match lines.next_line().await? {
                    Some(input) => match input.trim().parse::<i32>() {
                        Ok(num) if num == 1 || num == 2 || num == 3 => break num,
                        Ok(_) => eprintln!("Please enter either 1, 2 or 3!"),
                        Err(_) => eprintln!("Bad input, please enter a valid number!"),
                    },
                    None => return Ok(()),
                }
            };

            let message;

            if op == 1 {
                message = format!("1");
                if let Err(e) = stream.write_all(message.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }
            } else if op == 2{
                message = format!("0");
                if let Err(e) = stream.write_all(message.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }
            } else if op == 3 {
                *client = Some((msg, stream));
            }
        } else {
            let op = loop {
                println!("Additional required power: {}W. Press 1 to deny request or 2 to return to main menu: ", total);
                match lines.next_line().await? {
                    Some(input) => match input.trim().parse::<i32>() {
                        Ok(num) if num == 1 || num == 2 => break num,
                        Ok(_) => eprintln!("Please enter either 1 or 2!"),
                        Err(_) => eprintln!("Bad input, please enter a valid number!"),
                    },
                    None => return Ok(()),
                }
            };

            let message;

            if op == 1{
                message = format!("0");
                if let Err(e) = stream.write_all(message.as_bytes()).await {
                    eprintln!("Failed to send response: {}", e);
                }
            } else if op == 2 {
                *client = Some((msg, stream));
            }
        }

        
    } else {
        println!("No client connected");
    }

    Ok(())
}