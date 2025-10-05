pub fn setup_observability() {
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .install()
        .expect("failed to install Prometheus recorder");

    tracing_subscriber::fmt::init();
}