use std::str::FromStr;
use bitcoin::{bip32::Xpriv, Network};
use bitcoin::secp256k1::{SecretKey, rand::rngs::OsRng};

pub struct BitconKeys {
    pub master_key: String,
    pub network: u32
}

impl BitconKeys {
    pub fn new (secret_seed:Option<String>) -> BitconKeys {
        let network = Network::Bitcoin;
        let seed = match secret_seed {
            Some(secret) => SecretKey::from_str(&secret).unwrap(),_=>SecretKey::new(&mut OsRng)
        };
        let master_key = Xpriv::new_master(network, &seed.secret_bytes()).unwrap();
        
        return  BitconKeys{master_key:master_key.to_string(), network:network.magic()};
    }
}