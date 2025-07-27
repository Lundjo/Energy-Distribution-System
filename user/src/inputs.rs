use std::error::Error;
use tokio::io::AsyncBufReadExt;
use crate::connection::{send_message};
use crate::models::{Devices};

pub async fn select_device(dev: &mut Devices) -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let d = loop {
        dev.list_devices();
        println!("Enter number from 1 to 5 to choose which device to change the number of:");
        match lines.next_line().await? {
            Some(input) => match input.trim().parse::<i32>() {
                Ok(num) if (num >= 1 && num <= 5)  => break num,
                Ok(_) => eprintln!("Please enter a valid device number!"),
                Err(_) => eprintln!("Bad input, please enter a valid number!"),
            },
            None => return Ok(()),
        }
    };

    change_number_of_devices(dev, d).await?;

    Ok(())
}

pub async fn change_number_of_devices(dev: &mut Devices, d: i32) -> Result<(), Box<dyn Error>> {
    let stdin = tokio::io::BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let num = loop {
        println!("Enter number of devices to change:");
        match lines.next_line().await? {
            Some(input) => match input.trim().parse::<i32>() {
                Ok(num) => break num,
                Err(_) => eprintln!("Bad input, please enter a valid number!"),
            },
            None => return Ok(()),
        }
    };

    send_wattage(dev, d, num).await?;

    Ok(())
}

pub async fn send_wattage(dev: &mut Devices, d: i32, num: i32) -> Result<(), Box<dyn Error>> {
    if num > 0 {
        let mut message = String::from("");
        match num {
            1 => message = ((dev.d1 as f64 + num as f64) * 0.1).to_string(),
            2 => message = ((dev.d2 as f64 + num as f64) * 0.5).to_string(),
            3 => message = ((dev.d3 as f64 + num as f64) * 1.5).to_string(),
            4 => message = ((dev.d4 as f64 + num as f64) * 3.0).to_string(),
            5 => message = ((dev.d5 as f64 + num as f64) * 5.0).to_string(),
            _ => (),
        }

        match send_message(&message).await {
            Ok(response) => {
                match response.parse::<i32>() {
                    Ok(1) => {
                        dev.change_active_device_number(d, num);
                        println!("Successfully changed number of active devices");
                    },
                    Ok(_) => {
                        println!("Can't turn on devices");
                    },
                    Err(e) => {
                        eprintln!("Failed to parse server response: {}", e);
                    }
                }
            },
            Err(e) => eprintln!("Message could not be sent: {}", e),
        }
    } else {
        dev.change_active_device_number(d, num);
        println!("Succesfully changed number of active devices'");
    }
     
    Ok(())
}