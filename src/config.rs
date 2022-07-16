use std::env;

pub fn db_connection_string() -> String {
    env::var("APIGELO_CONNECTION_STRING").expect("APIGELO_CONNECTION_STRING not present")
}

pub fn media_path() -> String {
    env::var("APIGELO_MEDIA_PATH").unwrap_or("/media".to_string())
}

pub fn http_port() -> u16 {
    env::var("APIGELO_PORT").unwrap_or("8088".to_string())
        .parse().unwrap()
}

pub fn cors_origin() -> String {
    env::var("APIGELO_CORS_ORIGIN").unwrap_or("http://localhost:8080".to_string())
}