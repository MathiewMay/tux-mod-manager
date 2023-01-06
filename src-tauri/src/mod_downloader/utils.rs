use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

use failure::Fallible;

use url::{ParseError, Url};

pub fn parse_url(url_as_string: &str) -> Result<Url, ParseError> {
    match Url::parse(url_as_string) {
        Ok(url) => Ok(url),
        Err(error) if error == ParseError::RelativeUrlWithoutBase => {
            let url_with_base = format!("{}{}", "http://", url_as_string);
            parse_url(url_with_base.as_str())
        }
        Err(error) => Err(error),
    }
}

pub fn decode_percent_coded_string(data: &str) -> Fallible<String> {
    let mut decoded_bytes: Vec<u8> = Vec::new();
    let mut bytes = data.bytes();
    while let Some(b) = bytes.next() {
        match b as char {
            '%' => {
                let bytes_to_decode = &[bytes.next().unwrap(), bytes.next().unwrap()];
                let hex_str = std::str::from_utf8(bytes_to_decode).unwrap();
                decoded_bytes.push(u8::from_str_radix(hex_str, 16).unwrap());
            }
            _ => {
                decoded_bytes.push(b);
            }
        }
    }
    Ok(String::from_utf8(decoded_bytes)?)
}

pub fn get_file_handle(
    filename: &str,
    save_path: &str,
    resume_download: &bool,
    append: &bool,
) -> io::Result<File> {
    let path = format!("{}/{}", save_path, filename);
    // println!("Save Path: {}", path);
    if *resume_download && Path::new(&path).exists() {
        if *append {
            match OpenOptions::new().append(true).open(&path) {
                Ok(file) => Ok(file),
                Err(error) => Err(error),
            }
        } else {
            match OpenOptions::new().write(true).open(&path) {
                Ok(file) => Ok(file),
                Err(error) => Err(error),
            }
        }
    } else {
        match OpenOptions::new().write(true).create(true).open(&path) {
            Ok(file) => Ok(file),
            Err(error) => Err(error),
        }
    }
}
