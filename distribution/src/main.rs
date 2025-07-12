mod connection;
use connection::send_message;

fn main() {
    loop {
        let mut input = String::new();

        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let message = input.trim();
                match send_message(message) {
                    Ok(_) => println!("Message sent successfully: '{}'", message),
                    Err(e) => eprintln!("Message could not be sent: {}", e),
                }
            },
            Err(e) => eprintln!("Input could no be read: {}", e),
        }
    }
}
