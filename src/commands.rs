//This file is responsible for the handling of a recived command
//Author: Spencer Denton

use std::fs;

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
