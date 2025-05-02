use axum::body::Body;
use axum::{extract::MatchedPath, http::Request, middleware::Next, response::IntoResponse};
use metrics::{counter, histogram};
use std::time::Instant;

/// Middleware to track HTTP request metrics.
///
/// This middleware records metrics for each HTTP request, including:
/// - Total number of requests (`http_requests_total`)
/// - Request duration in seconds (`http_request_duration_seconds`)
///
/// The metrics are labeled with the HTTP method, request path, and response status code.
///
/// # Arguments
/// * `req` - The incoming HTTP request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// The HTTP response after processing the request.
pub async fn track_metrics(req: Request<Body>, next: Next) -> impl IntoResponse {
    // Record the start time of the request.
    let start = Instant::now();

    // Extract the matched path from the request extensions, or fall back to the URI path.
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };

    // Clone the HTTP method for labeling.
    let method = req.method().clone();

    // Pass the request to the next middleware or handler and await the response.
    let response = next.run(req).await;

    // Calculate the request latency in seconds.
    let latency = start.elapsed().as_secs_f64();

    // Get the response status code as a string.
    let status = response.status().as_u16().to_string();

    // Define labels for the metrics.
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    // Increment the counter for total HTTP requests.
    counter!("http_requests_total", &labels).increment(1);

    // Record the request duration in the histogram.
    histogram!("http_request_duration_seconds", &labels).record(latency);

    // Return the response.
    response
}