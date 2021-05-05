//This file is responsible for the handling of a recived command
//Author: Spencer Denton

use std::fs;

use anyhow::Error;

use crate::Operand;

use self::{check_mail::{email_login, get_email, write_flags}, send_mail::send_email};

mod send_mail {
    use lettre::{SmtpTransport, Transport, smtp::response::Response};

    pub fn send_email(to: &str, smtp_addr: &str, username: &str, password: &str, body: &str) -> anyhow::Result<Response> {
        let email = lettre_email::EmailBuilder::new()
        .to(to)
        .from(username)
        .subject("Sent from my uMessage-r")
        .text(body)
        .build()?;

        let mut mailer = make_smtp_transport(smtp_addr, username, password)?;
    
        let result = mailer.send(email.into())?;
        Ok(result)
    }

    fn make_smtp_transport(smtp_addr: &str, username: &str, password: &str) -> anyhow::Result<SmtpTransport> {
        let mailer = lettre::SmtpClient::new_simple(smtp_addr)?;
        let mailer = mailer.credentials(lettre::smtp::authentication::Credentials::new(username.into(), password.into())).transport();

        Ok(mailer)
    }
}
//NOTE: this doesn't log out
mod check_mail {
    use std::{fs, net::TcpStream};
    use anyhow::Error;
    use imap::Session;
    use native_tls::{TlsConnector, TlsStream};

    pub fn email_login(domain: &str, username: &str, password: &str) -> anyhow::Result<Session<TlsStream<TcpStream>>> {
        let tls = TlsConnector::builder().build()?;
        //make a new client at the address of the domain and port, double check with domain, and give it a TLS connector
        let client = imap::connect((domain, 993), domain, &tls)?;
    
        //now i start my session
        match client.login(username, password) {
            Ok(x) => Ok(x),
            Err((e, _)) => Err(Error::new(e))
        }
    }

    pub fn get_email(my_session: &mut Session<TlsStream<TcpStream>>, email_number: u32) -> anyhow::Result<&str> {

        //select my inbox and get the number of messages
        let inbox_len = my_session.select("INBOX")?.exists;

        let my_fetch = my_session.fetch((inbox_len-email_number+1).to_string(), "RFC822")?;

        for mail in my_fetch.iter() {
            match mail.body() {
                Some(body) => {
                    fs::write("email.txt", body)?;
                },
                None => {
                    return Err(Error::msg("Message was unreadable."));
                }
            };
        }
        Ok("email.txt")
    }

    pub fn write_flags(my_session: &mut Session<TlsStream<TcpStream>>) -> anyhow::Result<&str> {

        //examine (READ ONLY) my inbox and get the number of messages
        let inbox_len = my_session.examine("INBOX")?.exists;

        let my_fetch = my_session.fetch(inbox_len.to_string(), "FLAGS")?;

        for mail in my_fetch.iter() {
            let mut flags_string = String::new();
            for flags in mail.flags() {
                flags_string.push_str(&flags.to_string()[..]);
            }
            fs::write("flags.txt", flags_string).unwrap();
        }
        Ok("flags.txt")
    }
}

pub fn set_password(password: String) {
    //1) I need to read the client_info file, storing the info somewhere
    let mut info_string = fs::read_to_string("client_info.txt").unwrap();
    //2) Find "Password: ", and move the index to the first element of the password
    let index_password = info_string.find("Password: ").unwrap() + 10;

    // remove everything until \n
    while info_string
        .get(index_password..index_password + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        info_string.remove(index_password);
    }

    //3) Edit the stored info with "<password>\n"
    info_string.insert_str(index_password, &password[..]);

    //4) Write the stored info into the file
    fs::write("client_info.txt", &info_string[..]).unwrap();
}

pub fn set_imap_addr(imap_addr: String) {
    let mut info_string = fs::read_to_string("client_info.txt").unwrap();

    let index_imap_addr = info_string.find("IMAP: ").unwrap() + 6;

    while info_string
        .get(index_imap_addr..index_imap_addr + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        info_string.remove(index_imap_addr);
    }

    info_string.insert_str(index_imap_addr, &imap_addr[..]);
    fs::write("client_info.txt", &info_string[..]).unwrap();
}

pub fn set_smtp_addr(smtp_addr: String) {
    let mut info_string = fs::read_to_string("client_info.txt").unwrap();

    let index_smtp_addr = info_string.find("SMTP: ").unwrap() + 6;

    while info_string
        .get(index_smtp_addr..index_smtp_addr + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        info_string.remove(index_smtp_addr);
    }

    info_string.insert_str(index_smtp_addr, &smtp_addr[..]);
    fs::write("client_info.txt", &info_string[..]).unwrap();
}

pub fn set_username(username: String) {
    let mut info_string = fs::read_to_string("client_info.txt").unwrap();

    let index_username = info_string.find("Email: ").unwrap() + 7;

    while info_string
        .get(index_username..index_username + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'@'
    {
        info_string.remove(index_username);
    }

    info_string.insert_str(index_username, &username[..]);
    fs::write("client_info.txt", &info_string[..]).unwrap();
}

pub fn set_at(at: String) {
    let mut info_string = fs::read_to_string("client_info.txt").unwrap();

    let index_at = info_string.find("@").unwrap();

    while info_string
        .get(index_at..index_at + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        info_string.remove(index_at);
    }

    info_string.insert_str(index_at, &at[..]);
    fs::write("client_info.txt", &info_string[..]).unwrap();
}

pub fn set_to(to: String) {
    let mut draft_string = fs::read_to_string("email_draft.txt").unwrap();

    let index_to = draft_string.find("To: ").unwrap() + 4;

    while draft_string
        .get(index_to..index_to + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        draft_string.remove(index_to);
    }

    draft_string.insert_str(index_to, &to[..]);
    fs::write("email_draft.txt", &draft_string[..]).unwrap();
}

pub fn set_body(body: String) {
    let mut draft_string = fs::read_to_string("email_draft.txt").unwrap();

    let index_body = draft_string.find("Body: ").unwrap() + 6;

    while draft_string
        .get(index_body..index_body + 1)
        .unwrap()
        .as_bytes()
        .get(0)
        .unwrap()
        .to_owned()
        != b'\r'
    {
        draft_string.remove(index_body);
    }

    draft_string.insert_str(index_body, &body[..]);
    fs::write("email_draft.txt", &draft_string[..]).unwrap();
}

pub fn submit() -> anyhow::Result<String> {
    //construct the credentials
    //1) get email
    let email = &get_email_username()[..];
    //2) get password
    let password = &get_password()[..];
    //3) get SMTP server address
    let smtp_addr = &get_smtp_addr()[..];

    //construct the email
    //1) get TO
    let to = &get_to()[..];
    //2) get body
    let body = &get_body_to_send()[..];

    match send_email(to, smtp_addr, email, password, body) {
        Ok(_) => Ok(String::from("SENT")),
        Err(_) => Err(Error::msg("BAD")),
    }
}

pub fn fetch(num: u32, operand: Operand) -> anyhow::Result<String> {
    let mut my_session;
    match email_login(&get_imap_addr()[..], &get_email_username()[..], &get_password()) {
        Ok(x) => {my_session = x},
        Err(_) => return Err(Error::msg("couldn't login")),
    };

    match get_email(&mut my_session, num) {
        Ok(message) => println!("{}", message),
        Err(_) => return Err(Error::msg("couldn't get email")),
    };

    match operand {
        Operand::Addr => {
            return Ok(get_from_email_addr())
        },
        Operand::Subject => {
            return Ok(get_from_subject())
        },
        Operand::Text => {
            return Ok(get_from_text())
        },
        Operand::None => {
            return Err(Error::msg("No operand"))
        },
    };
}

pub fn checkmail() -> anyhow::Result<String> {
    //1) LOGIN TO IMAP
    let mut my_session;
    match email_login(&get_imap_addr()[..], &get_email_username()[..], &get_password()) {
        Ok(x) => {my_session = x},
        Err(_) => return Err(Error::msg("couldn't login")),
    };

    //2) Write flags to file
    match write_flags(&mut my_session) {
        Ok(msg) => println!("{}", msg),
        Err(error) => return Err(error),
    };

    let flags_string = fs::read_to_string("flags.txt").unwrap();
    if !flags_string.contains("Seen") {
        Ok(String::from("NEW"))
    } else {
        Ok(String::from("OLD"))
    }
}

fn get_from_email_addr() -> String {
    let mut email_string = fs::read_to_string("email.txt").unwrap();
    let index_from = email_string.find("From: ").unwrap() + 6;
    let truncated_email = &mut email_string[index_from..];
    let index_email = truncated_email.find("<").unwrap() + 1;
    let index_end_email = truncated_email.find(">").unwrap();
    truncated_email[index_email..index_end_email].to_string()
}

fn get_from_subject() -> String {
    let mut email_string = fs::read_to_string("email.txt").unwrap();
    let index_subject = email_string.find("Subject: ").unwrap() + 9;
    let truncated_email = &mut email_string[index_subject..];
    let index_newline = truncated_email.find('\r').unwrap();
    let split_slices = truncated_email.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_from_text() -> String {
    let mut email_string = fs::read_to_string("email.txt").unwrap();
    let index_charset = email_string.find("charset").unwrap() + 7;
    let truncated_text = &mut email_string[index_charset..];
    let index_start_text = truncated_text.find('\r').unwrap() + 1;
    let truncated_text = &truncated_text[index_start_text..];
    let index_end_text = truncated_text.find("-").unwrap();
    truncated_text[..index_end_text].trim().to_string()
}

fn get_imap_addr() -> String {
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_imap = credentials_string.find("IMAP: ").unwrap() + 6;
    let truncated_credentials = &mut credentials_string[index_imap..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_email_username() -> String {
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_email = credentials_string.find("Email: ").unwrap() + 7;
    let truncated_credentials = &mut credentials_string[index_email..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_password() -> String {
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_password = credentials_string.find("Password: ").unwrap() + 10;
    let truncated_credentials = &mut credentials_string[index_password..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_smtp_addr() -> String {
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_smtp = credentials_string.find("SMTP: ").unwrap() + 6;
    let truncated_credentials = &mut credentials_string[index_smtp..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_to() -> String {
    let mut email_string = fs::read_to_string("email_draft.txt").unwrap();
    let index_to = email_string.find("To: ").unwrap() + 4;
    let truncated_email = &mut email_string[index_to..];
    let index_newline = truncated_email.find('\r').unwrap();
    let split_slices = truncated_email.split_at(index_newline);
    split_slices.0.to_string()
}

fn get_body_to_send() -> String {
    let mut email_string = fs::read_to_string("email_draft.txt").unwrap();
    let index_body = email_string.find("Body: ").unwrap() + 6;
    let truncated_email = &mut email_string[index_body..];
    let index_newline = truncated_email.find('\r').unwrap();
    let split_slices = truncated_email.split_at(index_newline);
    split_slices.0.to_string()
}

