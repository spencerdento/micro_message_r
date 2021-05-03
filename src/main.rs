//Spencer Denton

//UNDEFINED BEHAVIOR if LaunchPad Unplugged while UART0 is being listend to

use serial::windows::COMPort;
use uart0::{uart0_init_port, uart0_listen, uart0_write_one_message};

mod commands;
mod uart0;

const STX: u8 = 0x02;
const ETX: u8 = 0x03;
const OK_UART_MESSAGE: [u8; 4] = [STX, b'O', b'K', ETX];

enum Operand {
    Addr,
    Subject,
    Text,
    None,
}
enum Command {
    CheckMail,
    Fetch { num: u32, operand: Operand },
    Username(String),
    At(String),
    Password(String),
    SmtpAddr(String),
    ImapAddr(String),
    To(String),
    None,
}

fn main() {
    let mut com_5 = uart0_init_port().expect("Couldn't Open COM5");

    //MAKE A DEFAULT COMMAND FOR ERRORS, SO IT CAN REPEAT TASK

    loop {
        let command = match uart0_listen(&mut com_5) {
            Ok(message) => message,
            Err(_) => String::from("?"),
        };

        //handle command
        let mut recieved_command = Command::None;
        let mut recieved_operand = Operand::None;
        if command == String::from("CHECK MAIL") {
            recieved_command = Command::CheckMail;
        } else if command.contains("FETCH ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            //SEND "OK"
            match uart0_write_one_message(&mut com_5, &OK_UART_MESSAGE[..]) {
                Ok(_) => println!("Sent OK"),
                Err(error) => println!("{:?}", error),
            };
            //GET OPERAND
            let command = match uart0_listen(&mut com_5) {
                Ok(message) => message,
                Err(_) => String::from("?"),
            };
            if command == String::from("ADDR") {
                recieved_operand = Operand::Addr;
            } else if command == String::from("SUBJECT") {
                recieved_operand = Operand::Subject;
            } else if command == String::from("TEXT") {
                recieved_operand = Operand::Text;
            }
            recieved_command = Command::Fetch {
                num: recieved_number,
                operand: recieved_operand,
            };
        } else if command.contains("USERNAME ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let username = match read_more(&mut com_5, recieved_number) {
                Ok(string) => string,
                Err(_) => String::from("?"),
            };
            recieved_command = Command::Username(username);
        } else if command.contains("@ ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let mut at = String::from("@");
            match read_more(&mut com_5, recieved_number) {
                Ok(string) => at.push_str(&string[..]),
                Err(_) => at = String::from("?"),
            };
            recieved_command = Command::At(at);
        } else if command.contains("PASSWORD ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let password = match read_more(&mut com_5, recieved_number) {
                Ok(string) => string,
                Err(_) => String::from("?"),
            };
            recieved_command = Command::Password(password);
        } else if command.contains("SMTP ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let smtp_addr = match read_more(&mut com_5, recieved_number) {
                Ok(string) => string,
                Err(_) => String::from("?"),
            };
            recieved_command = Command::SmtpAddr(smtp_addr);
        } else if command.contains("IMAP ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let imap_addr = match read_more(&mut com_5, recieved_number) {
                Ok(string) => string,
                Err(_) => String::from("?"),
            };
            recieved_command = Command::ImapAddr(imap_addr);
        } else if command.contains("TO ") {
            let recieved_number =
                (command[(command.len() - 1)..].as_bytes().get(0).unwrap() - 48) as u32;
            let to = match read_more(&mut com_5, recieved_number) {
                Ok(string) => string,
                Err(_) => String::from("?"),
            };
            recieved_command = Command::To(to);
        }

        execute_command(recieved_command);
    }
}

fn execute_command(command: Command) {
    match command {
        Command::CheckMail => {
            println!("Check Mail");
        }
        Command::Fetch { num, operand } => {
            let operand_name = match operand {
                Operand::Addr => String::from("Addr"),
                Operand::Subject => String::from("Subject"),
                Operand::Text => String::from("Text"),
                Operand::None => String::from("None"),
            };
            println!("Fetch, num: {}, operand: {}", num, operand_name);
        }
        Command::Username(username) => {
            println!("Username: {}", username);
            commands::set_username(username);
        }
        Command::At(at) => {
            println!("At: {}", at);
            commands::set_at(at);
        }
        Command::Password(password) => {
            println!("Password: {}", password);
            commands::set_password(password);
        }
        Command::SmtpAddr(smtp_addr) => {
            println!("SMTPAddr: {}", smtp_addr);
            commands::set_smtp_addr(smtp_addr);
        }
        Command::ImapAddr(imap_addr) => {
            println!("IMAPAddr: {}", imap_addr);
            commands::set_imap_addr(imap_addr);
        }
        Command::To(to) => {
            println!("To: {}", to);
        }
        Command::None => {
            println!("None");
        }
    };
}

fn read_more(port: &mut COMPort, num_more_reads: u32) -> anyhow::Result<String> {
    let mut full_message = String::new();
    let mut num_more_reads = num_more_reads;

    while num_more_reads > 0 {
        //SEND "OK"
        match uart0_write_one_message(port, &OK_UART_MESSAGE[..]) {
            Ok(_) => println!("Sent OK"),
            Err(error) => return Err(error),
        };

        //LISTEN FOR NEXT PART
        let command = match uart0_listen(port) {
            Ok(message) => message,
            Err(_) => String::from("?"),
        };
        full_message.push_str(&command[..]);
        num_more_reads -= 1;
    }

    Ok(full_message)
}
