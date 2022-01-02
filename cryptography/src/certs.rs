use openssl::rsa::Rsa;
use std;
use std::fs::OpenOptions;
use std::io::prelude::*;

// Recommend using static environment variables for cert generation
// const BITES: u32 = 4096;
// const PRIVATE_KEY_LOCATION: &str = "certs/private.pem";
// const PUBLIC_KEY_LOCATION: &str = "certs/public.pem";

struct Key {
    location: String,
    data: Vec<u8>,
}

pub struct RSA {
    public: Key,
    private: Key,
}

pub trait FileIO {
    fn write_public_key_to_file(self, key_location: String) -> Self;
    fn write_private_key_to_file(self, key_location: String) -> Self;
    fn read_public_key_from_file(self, key_location: String) -> Self;
    fn read_private_key_from_file(self, key_location: String) -> Self;
}

impl RSA {
    pub fn new(bites: u32) -> Self {
        let rsa = Rsa::generate(bites).unwrap();
        let public = Key {
            data: rsa.public_key_to_pem().unwrap(),
            location: "".to_string(),
        };
        let private = Key {
            data: rsa.private_key_to_pem().unwrap(),
            location: "".to_string(),
        };
        RSA {
            public: public,
            private: private,
        }
    }
}
impl FileIO for RSA {
    fn write_public_key_to_file(mut self, key_location: String) -> Self {
        let mut options = OpenOptions::new();
        self.public.location = key_location;
        let mut file = options
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.public.location)
            .unwrap();
        let mut pos = 0;
        while pos < self.public.data.len() {
            let bytes_written = file.write(&self.public.data[pos..]);
            match bytes_written {
                Ok(x) => pos += x,
                Err(x) => println!("{}", x),
            }
        }
        self
    }

    fn write_private_key_to_file(mut self, key_location: String) -> Self {
        let mut options = OpenOptions::new();
        self.private.location = key_location;
        let mut file = options
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.private.location)
            .unwrap();
        let mut pos = 0;
        while pos < self.private.data.len() {
            let bytes_written = file.write(&self.private.data[pos..]);
            match bytes_written {
                Ok(x) => pos += x,
                Err(x) => println!("{}", x),
            }
        }
        self
    }

    fn read_public_key_from_file(mut self, key_location: String) -> Self {
        let mut options = OpenOptions::new();
        let file = options.read(true).open(&key_location);
        match file {
            Ok(mut data) => {
                self.public.location = key_location;
                match data.read_to_end(&mut self.public.data) {
                    Ok(x) => println!("Bytes Written: {}", x),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
        self
    }

    fn read_private_key_from_file(mut self, key_location: String) -> Self {
        let mut options = OpenOptions::new();
        let file = options.read(true).open(&key_location);
        match file {
            Ok(mut data) => {
                self.private.location = key_location;
                match data.read_to_end(&mut self.private.data) {
                    Ok(x) => println!("Bytes Written: {}", x),
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
        self
    }
}
