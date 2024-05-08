use std::{fs::File, io::Read};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
    };

    println!("{}", encode);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(&buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buf)?,
    };
    let decode = String::from_utf8(decode)?;
    println!("{}", decode);
    Ok(())
}
