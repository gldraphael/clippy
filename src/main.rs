use axum::{response::Result, Router};
use std::net::SocketAddr;
use std::env;

const CLIPPY:&str = r#"

         __
        /  \        _____________
        |  |       /             \
        @  @       | It looks    |
        || ||      | like you    |
        || ||   <--| are very    |
        |\_/|      | bored.      |
        \___/      \_____________/

"#;
const PORT_ENV_VAR:&str = "APP_PORT";

#[tokio::main]
async fn main() {

    // build our application with a fallback route
    let app = Router::new().fallback(handler);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], get_port()));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Result<&'static str> {
    Ok(CLIPPY)
}

fn get_port() -> u16 {
    env::var(PORT_ENV_VAR)
        .unwrap_or("NOT_SET".to_string())
        .parse::<u16>()
        .unwrap_or(3000)
}
