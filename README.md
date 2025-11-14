# Rust FPL Authorisation

Simple utilty to return a Fantasy Premier League profile with an access token attached that can be used for API access. 

## To run

Simply build and run using

```
cargo run
```

from the root directory. Expected return of a [Profile](https://github.com/RobWHickman/fpl-rs/blob/master/src/profile.rs) struct verifying user information and an attached `access_code` for that user which can be used to access the FPL API.

If any of the requests are failing try

```
RUST_LOG=debug cargo run
```

to diagnose. This can also be set in the `.env`. All requests should return a [200] status code except the [access request](https://github.com/RobWHickman/fpl-rs/blob/master/src/main.rs#L31) which should return a [302].

## API integration

Use the returned access token to retrieve information from endpoints of the FPL API. Some API documentation can be found [here](https://www.oliverlooney.com/blogs/FPL-APIs-Explained). For example, the retrieve the user's team, this boilerplate for a given `TEAM_ID` (see the URL of the team page) and `ACCESS_TOKEN` will return a JSON of the players selected.

```
use reqwest::blocking::Client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let team_id = env::var("TEAM_ID")?;
    let access_token = env::var("ACCESS_TOKEN")?;
    
    let url = format!("https://fantasy.premierleague.com/api/my-team/{}", team_id);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("X-API-Authorization", format!("Bearer {}", access_token))
        .send()?
        .json::<serde_json::Value>()?;
    
    println!("{:#?}", response);
    
    Ok(())
}
```

```
{'picks': [{'element': 670, 'position': 1, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': True, 'element_type': 1, 'selling_price': 46, 'purchase_price': 45}, {'element': 291, 'position': 2, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 2, 'selling_price': 54, 'purchase_price': 55}, {'element': 506, 'position': 3, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 2, 'selling_price': 53, 'purchase_price': 55}, {'element': 107, 'position': 4, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 2, 'selling_price': 49, 'purchase_price': 50}, {'element': 324, 'position': 5, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 3, 'selling_price': 65, 'purchase_price': 65}, {'element': 267, 'position': 6, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 3, 'selling_price': 66, 'purchase_price': 65}, {'element': 485, 'position': 7, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 3, 'selling_price': 74, 'purchase_price': 75}, {'element': 157, 'position': 8, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 3, 'selling_price': 62, 'purchase_price': 65}, {'element': 64, 'position': 9, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 4, 'selling_price': 85, 'purchase_price': 90}, {'element': 624, 'position': 10, 'multiplier': 2, 'is_captain': True, 'is_vice_captain': False, 'element_type': 4, 'selling_price': 77, 'purchase_price': 80}, {'element': 654, 'position': 11, 'multiplier': 1, 'is_captain': False, 'is_vice_captain': False, 'element_type': 4, 'selling_price': 63, 'purchase_price': 65}, {'element': 183, 'position': 12, 'multiplier': 0, 'is_captain': False, 'is_vice_captain': False, 'element_type': 1, 'selling_price': 44, 'purchase_price': 45}, {'element': 186, 'position': 13, 'multiplier': 0, 'is_captain': False, 'is_vice_captain': False, 'element_type': 2, 'selling_price': 44, 'purchase_price': 45}, {'element': 82, 'position': 14, 'multiplier': 0, 'is_captain': False, 'is_vice_captain': False, 'element_type': 3, 'selling_price': 75, 'purchase_price': 70}, {'element': 609, 'position': 15, 'multiplier': 0, 'is_captain': False, 'is_vice_captain': False, 'element_type': 2, 'selling_price': 43, 'purchase_price': 45}], 'picks_last_updated': '2025-08-22T09:08:22.805900Z', 'chips': [{'id': 4, 'status_for_entry': 'available', 'played_by_entry': [], 'name': 'bboost', 'number': 1, 'start_event': 1, 'stop_event': 19, 'chip_type': 'team', 'is_pending': False}, {'id': 5, 'status_for_entry': 'available', 'played_by_entry': [], 'name': '3xc', 'number': 1, 'start_event': 1, 'stop_event': 19, 'chip_type': 'team', 'is_pending': False}, {'id': 1, 'status_for_entry': 'available', 'played_by_entry': [], 'name': 'wildcard', 'number': 1, 'start_event': 2, 'stop_event': 19, 'chip_type': 'transfer', 'is_pending': False}, {'id': 3, 'status_for_entry': 'available', 'played_by_entry': [], 'name': 'freehit', 'number': 1, 'start_event': 2, 'stop_event': 19, 'chip_type': 'transfer', 'is_pending': False}], 'transfers': {'cost': 4, 'status': 'cost', 'limit': 5, 'made': 0, 'bank': 85, 'value': 908}}
```

## Required Environment Variables

Only needs two environment variables specific to the user:

- `EMAIL`: the email address used to set up the account. Can be updated in [account management](https://www.premierleague.com/en/settings/email-address).
- `PASSWORD`: the password on the account. Used to login at the homepage. Can be changed using [account management](https://www.premierleague.com/en/settings/account-security).

These are expected to be found in the environment- the easiest way by storing as strings in a .gitignored `.env` file.

Not strictly necessary, but can also control the level of debugging by using
-`RUST_LOG=debug` or whatever the desired logging level 
in the `.env`. N.b. that the access token for the account is printed at the `INFO` level.