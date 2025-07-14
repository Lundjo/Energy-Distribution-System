mod connection;
use connection::send_message1;
use connection::send_message2;
use connection::start_server;
use std::thread;

fn main() {
    thread::spawn(start_server);

    loop {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let message = input.trim();
                match send_message1(message) {
                    Ok(_) => println!("Message sent successfully: '{}'", message),
                    Err(e) => eprintln!("Message could not be sent: {}", e),
                }

                match send_message2(message) {
                    Ok(_) => println!("Message sent successfully: '{}'", message),
                    Err(e) => eprintln!("Message could not be sent: {}", e),
                }
            },
            Err(e) => eprintln!("Input could no be read: {}", e),
        }
    }
}
