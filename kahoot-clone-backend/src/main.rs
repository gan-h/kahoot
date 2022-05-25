mod ext;
/// Module for handling the websocket api.
mod ws;

use std::net::SocketAddr;

// `axum` is a Rust web server framework
use axum::Router;

/**
 * Note: You may notice that some functions end with a naked expression without
 * and no return statement.
 *
 * When a block of code (ie. code surrounded by curly braces `{}`) ends with
 * an expression without a semicolon, it is implicitly "resolved" to that value.
 *
 * Example:
 * ```
 * let x = {
 *     let y = 8;
 *     y * y
 * };
 * ```
 * is equivalent to:
 * ```
 * let x = 8 * 8;
 * ```
 *
 * Relevant: https://doc.rust-lang.org/reference/expressions/block-expr.html
 */

/// The main function, where the application starts
//
// `tokio::main` is a macro for defining async main functions.
//
// `tokio` is an async/await runtime library for Rust.
//
// You need to use a library for async in Rust because it doesn't provide
// an "official" one out of the box which can be a pro or a con depending on
// how you look at it.
#[tokio::main]
async fn main() {
    // Set the host address to `localhost:3000`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

/// The server router
fn app() -> Router {
    Router::new()
        // GET /ws
        .nest("/ws", ws::router())
}
