use sha256;
use std::fs;

fn main() {
    let mut result = true;

    match fs::read_to_string("sha256.txt") {
        Ok(hash_strings) => {
            for hash_string in hash_strings.split("\n") {
                let s = hash_string.split("  ").collect::<Vec<_>>();;
                if s.len() != 2 {
                    continue; // for empty line or invalid stuff
                }
                match std::fs::read(s[1]) {
                    Ok(bytes) => {
                        let hash = sha256::digest_bytes(&bytes);
                        if hash != s[0] {
                            println!("[-] NG: {}", s[1]);
                            result = false;
                        } else {
                            println!("[*] OK: {}", s[1]);
                        }
                    },
                    Err(e) => {
                        println!("[-] Failed to read {}", s[1]);
                        println!("[-] {}", e);
                        result = false;
                    }
                }
            }
        },
        Err(e) => {
            println!("[-] Failed to read sha256.txt");
            println!("[-] {}", e);
            result = false;
        }
    }

    if result {
        println!("[+] Hash check has successfully completed!")
    } else {
        println!("[-] Hash check failed.")
    }
}
