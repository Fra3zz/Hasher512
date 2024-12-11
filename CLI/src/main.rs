use sha1::{Sha1, Digest as Sha1Digest};
use sha2::{Sha256, Sha512};
use clap::{Arg, Command};
use colored::*;
use std::fs::File;
use std::io::{BufReader, Read};

fn hash_sha1<R: Read>(reader: R) -> String {
    let mut hasher = Sha1::new();
    let mut buffer = [0; 4096];
    let mut reader = BufReader::new(reader);

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read data");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    format!("{:x}", hasher.finalize())
}

fn hash_sha256<R: Read>(reader: R) -> String {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];
    let mut reader = BufReader::new(reader);

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read data");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    format!("{:x}", hasher.finalize())
}

fn hash_sha512<R: Read>(reader: R) -> String {
    let mut hasher = Sha512::new();
    let mut buffer = [0; 4096];
    let mut reader = BufReader::new(reader);

    loop {
        let bytes_read = reader.read(&mut buffer).expect("Failed to read data");
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    format!("{:x}", hasher.finalize())
}

fn main() {
    let matches = Command::new("SHA Calculator")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Computes SHA-1, SHA-256, and SHA-512 hashes")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("File to hash")
            .conflicts_with("text"))
        .arg(Arg::new("text")
            .short('t')
            .long("text")
            .value_name("TEXT")
            .help("Text to hash")
            .conflicts_with("file"))
        .get_matches();

    if let Some(file_path) = matches.get_one::<String>("file") {
        println!("{}", "Hashes for file:".bold().underline());

        let file = File::open(file_path).expect("Failed to open file");
        println!("{}   {}", "SHA-1:".green(), hash_sha1(BufReader::new(file)));

        let file = File::open(file_path).expect("Failed to open file");
        println!("{} {}", "SHA-256:".yellow(), hash_sha256(BufReader::new(file)));

        let file = File::open(file_path).expect("Failed to open file");
        println!("{} {}", "SHA-512:".blue(), hash_sha512(BufReader::new(file)));
    } else if let Some(text) = matches.get_one::<String>("text") {
        println!("{}", "Hashes for text:".bold().underline());
        println!("{}   {}", "SHA-1:".green(), hash_sha1(text.as_bytes()));
        println!("{} {}", "SHA-256:".yellow(), hash_sha256(text.as_bytes()));
        println!("{} {}", "SHA-512:".blue(), hash_sha512(text.as_bytes()));
    } else {
        eprintln!("{}", "No input provided. Use --file <path> or --text <string>.".red());
    }
}
