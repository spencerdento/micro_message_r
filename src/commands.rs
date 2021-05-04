//This file is responsible for the handling of a recived command
//Author: Spencer Denton

use std::fs;

use anyhow::Error;

use self::send_mail::send_email;

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
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    //1) get email
    let index_email = credentials_string.find("Email: ").unwrap() + 7;
    let truncated_credentials = &mut credentials_string[index_email..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    let email = split_slices.0;
    //2) get password
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_password = credentials_string.find("Password: ").unwrap() + 10;
    let truncated_credentials = &mut credentials_string[index_password..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    let password = split_slices.0;
    //3) get SMTP server address
    let mut credentials_string = fs::read_to_string("client_info.txt").unwrap();
    let index_smtp = credentials_string.find("SMTP: ").unwrap() + 6;
    let truncated_credentials = &mut credentials_string[index_smtp..];
    let index_newline = truncated_credentials.find('\r').unwrap();
    let split_slices = truncated_credentials.split_at(index_newline);
    let smtp_addr = split_slices.0;

    //construct the email
    let mut email_string = fs::read_to_string("email_draft.txt").unwrap();
    //1) get TO
    let index_to = email_string.find("To: ").unwrap() + 4;
    let truncated_email = &mut email_string[index_to..];
    let index_newline = truncated_email.find('\r').unwrap();
    let split_slices = truncated_email.split_at(index_newline);
    let to = split_slices.0;
    //2) get body
    let mut email_string = fs::read_to_string("email_draft.txt").unwrap();
    let index_body = email_string.find("Body: ").unwrap() + 6;
    let truncated_email = &mut email_string[index_body..];
    let index_newline = truncated_email.find('\r').unwrap();
    let split_slices = truncated_email.split_at(index_newline);
    let body = split_slices.0;

    match send_email(to, smtp_addr, email, password, body) {
        Ok(_) => Ok(String::from("SENT")),
        Err(_) => Err(Error::msg("BAD")),
    }
    
}




