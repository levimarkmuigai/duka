use tracing_subscriber::{
    EnvFilter, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};
pub fn init_telemetry() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_span_events(FmtSpan::CLOSE);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}
