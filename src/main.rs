use fpl_rs::pkce::pkce_init;
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let (verifier, challenge, state) = pkce_init();
    

    println!("all done!")
}
