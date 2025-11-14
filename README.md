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
DEBUG=1 cargo run
```

to diagnose. All requests should return a [200] status code except the [access request](https://github.com/RobWHickman/fpl-rs/blob/master/src/main.rs#L31) which should return a [302].

## API inegration



## Required Environment Variables

Only needs two environment variables specific to the user:

- `EMAIL`: the email address used to set up the account. Can be updated in [account management](https://www.premierleague.com/en/settings/email-address).
- `PASSWORD`: the password on the account. Used to login at the homepage. Can be changed using [account management](https://www.premierleague.com/en/settings/account-security).