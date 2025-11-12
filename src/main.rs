use fpl_rs::pkce::pkce_init;
use fpl_rs::urls;
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let (verifier, challenge, state) = pkce_init();

    let auth_url = format!("{}{}", urls.BASE_ACCOUNT_URL, urls.AUTH_PATH);

    println!("all done!")
}
