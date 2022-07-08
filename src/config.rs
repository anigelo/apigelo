use std::env;

pub fn db_connection_string() -> String {
    env::var("APIGELO_CONNECTION_STRING").expect("APIGELO_CONNECTION_STRING not present")
}