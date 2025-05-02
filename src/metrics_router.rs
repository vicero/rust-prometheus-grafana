use axum::{Router, routing::get};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::future::ready;

/// Sets up and configures a Prometheus metrics recorder.
///
/// This function initializes a Prometheus metrics recorder with custom bucket
/// configurations for the `http_requests_duration_seconds` metric. The buckets
/// are defined as exponential intervals in seconds.
///
/// # Returns
/// A `PrometheusHandle` that can be used to render the metrics in a Prometheus-compatible format.
fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[0.005, 0.01, 0.025, 0.05, 0.1];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

/// Creates an Axum router with a `/metrics` endpoint.
///
/// The `/metrics` endpoint serves Prometheus metrics in a text format. The
/// metrics are collected using the Prometheus recorder configured by
/// `setup_metrics_recorder`.
///
/// # Returns
/// An `axum::Router` instance with the `/metrics` route configured.
pub fn create_router() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}