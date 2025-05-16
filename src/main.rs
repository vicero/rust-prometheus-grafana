//! Run with
//!
//! ```not_rust
//! cargo run
//! ```

mod metrics_middleware;
mod metrics_router;

use axum::{middleware, response::Html, routing::get, Router};
use axum::extract::Path;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/delay/{milliseconds}", get(delay))
        // introduce the metrics middleware before adding metrics, since we
        // want to exclude the /metrics route from prometheus
        .route_layer(middleware::from_fn(metrics_middleware::track_metrics))
        // merge the new router with the main router, introducing the /metrics endpoint
        .merge(metrics_router::create_router());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    println!("prometheus metrics at http://{}/metrics", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn delay(Path(milliseconds): Path<i16>) -> Html <String> {
    {
        // execute the delay specified by the path parameter
        tokio::time::sleep(std::time::Duration::from_millis(milliseconds as u64)).await;
        // return a simple HTML response
        Html(format!("<p>Delayed for <em>{milliseconds}</em> milliseconds</p>"))
    }
}