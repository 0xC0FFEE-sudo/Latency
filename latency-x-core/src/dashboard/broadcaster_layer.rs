use crate::dashboard::events::{DashboardEvent, LogEntry};
use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

pub struct DashboardBroadcastLayer {
    tx: broadcast::Sender<DashboardEvent>,
}

impl DashboardBroadcastLayer {
    pub fn new(tx: broadcast::Sender<DashboardEvent>) -> Self {
        Self { tx }
    }
}

impl<S> Layer<S> for DashboardBroadcastLayer
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut fields = std::collections::HashMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        let log_entry = LogEntry {
            level: event.metadata().level().to_string(),
            target: event.metadata().target().to_string(),
            message: fields
                .get("message")
                .map(|v| v.to_string())
                .unwrap_or_default().trim_matches('"').to_string(),
            timestamp: Utc::now(),
        };

        let _ = self.tx.send(DashboardEvent::Log(log_entry));
    }
}

struct JsonVisitor<'a>(&'a mut std::collections::HashMap<String, serde_json::Value>);

impl tracing::field::Visit for JsonVisitor<'_> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
} 