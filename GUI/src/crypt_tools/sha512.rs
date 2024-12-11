
use sha1::Digest as Sha1Digest;
use sha2::Sha512;
use std::io::{BufReader, Read};


pub fn hash_sha512<R: Read>(reader: R) -> String {
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