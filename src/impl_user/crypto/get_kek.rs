use zeroize::Zeroizing;
use std::{fs, path::Path};
use anyhow::{Error, anyhow};

const BASE_DIR: &str = "/etc/broker";
const VERSION_FILE: &str = "master.key.v";
const KEY_PREFIX: &str = "master.key.";

pub fn get_kek_version() -> Result<String, Error> {
    
    let version_raw = fs::read_to_string(Path::new(BASE_DIR).join(VERSION_FILE))?;
    let version = version_raw.trim();
    
    if !version.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow!("Invalid KEK version"));
    }
    Ok(version.to_string())
}


pub fn get_kek() -> Result<Zeroizing<Vec<u8>>, Error> {
    
    let version = get_kek_version()?;
   
    let key_path = Path::new(BASE_DIR).join(format!("{}{}", KEY_PREFIX, &version));
    let key_bytes = Zeroizing::new(fs::read(&key_path)?);
    let kek = Zeroizing::new(hex::decode(&*key_bytes)?);
        
    if kek.len() != 32 {
        return Err(anyhow!("Invalid KEK length"));
    }

    Ok(kek)

}

