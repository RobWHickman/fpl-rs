use fpl_rs::pkce::{pkce_challenge, pkce_verifier};

fn main() {
    let verifier = pkce_verifier();
    let challenge = pkce_challenge(&verifier);

    println!("all done!")
}
