use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub struct ZtpTelemetry;

impl ZtpTelemetry {
    pub fn get_subscriber(
        name: impl Into<String>,
        env_filter: impl Into<String>,
    ) -> impl Subscriber + Send + Sync {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter.into()));
        let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);

        let subscriber = Registry::default()
            .with(env_filter)
            .with(JsonStorageLayer)
            .with(formatting_layer);

        subscriber
    }

    pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
        LogTracer::init().expect("Failed to set logger.");
        set_global_default(subscriber).expect("Failed to set subscriber.");
    }
}
