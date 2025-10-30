use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_observability() {
    // // Metrics
    // metrics_exporter_prometheus::PrometheusBuilder::new()
    //     .install()
    //     .expect("failed to install Prometheus recorder");

    // Tracing/Logging
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug,reqwest=info,hyper=warn,h2=warn")) // debug by default, but quieter for HTTP libraries
        )
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .init();
}