use bitcoin::{Address, Network, XOnlyPublicKey};
use bitcoin::secp256k1::{rand, Secp256k1, Keypair};

pub fn generate_wallet() {
    let elliptic_curve_fn = Secp256k1::new();
    let network = Network::Testnet;
    
    let (secret_key, public_key) = elliptic_curve_fn.generate_keypair(&mut rand::thread_rng());
    let keypair = Keypair::from_secret_key(&elliptic_curve_fn, &secret_key);
    let (x_only_public_key, _parity) = XOnlyPublicKey::from_keypair(&keypair);
    let tap_address = Address::p2tr(&elliptic_curve_fn, x_only_public_key, None, network);
    let p2wpkh_address = Address::p2wpkh(&bitcoin::CompressedPublicKey(public_key), network);

    println!("pay to witness public key hash {}", p2wpkh_address.to_qr_uri().to_ascii_lowercase());
    println!("pay to taproot address {}", tap_address.to_qr_uri().to_ascii_lowercase());
    println!("network: {}", network);
}