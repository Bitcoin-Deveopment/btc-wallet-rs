pub mod bitcoin_client;
pub mod bitcoin_keys;
fn main() {
    bitcoin_client::generate_wallet();
    println!("Hello, world!");
}
