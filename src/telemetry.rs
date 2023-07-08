use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn init_tracing<Sink>(
    name: String,
    env_filter: String,
    json: bool,
    sink: Sink,
) where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    if !json {
        let fmt_layer = fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_target(false);

        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer);

        set_global_default(subscriber).expect("failed to set subscriber");
        return;
    }

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    LogTracer::init().expect("failed to set logger");
    set_global_default(subscriber).expect("failed to set subscriber");
}
