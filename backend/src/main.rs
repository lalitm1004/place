use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use std::process::ExitCode;

use backend::AppState;

#[tokio::main]
async fn main() -> ExitCode {
    let app_state = match AppState::new().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Error initializing AppState: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let app = Route::new().data(app_state).with(Cors::new());

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    ExitCode::FAILURE
}
