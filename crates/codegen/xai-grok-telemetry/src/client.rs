//! Privacy-build telemetry client stubs.
//!
//! Typed telemetry events remain available as internal instrumentation
//! contracts, but this module has no storage, HTTP emitter, or analytics SDK.

use crate::config::{TelemetryConfig, TelemetryMode};
use crate::http::OriginClientInfo;

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug, Default)]
pub struct TelemetryClient;

#[derive(Clone, Debug, Default)]
pub struct UserContext {
    pub country: String,
    pub language: String,
    pub timestamp: String,
}

impl UserContext {
    pub fn collect() -> Self {
        Self::default()
    }
}

pub fn is_enabled() -> bool {
    false
}

pub fn is_session_metrics_enabled() -> bool {
    false
}

pub async fn track(event_name: &str, request_id: &str, ctx: &UserContext, metadata: Metadata) {
    let _ = (event_name, request_id, ctx, metadata);
}

pub fn sync_profile() {}

#[allow(clippy::too_many_arguments)]
pub fn init(
    config: TelemetryConfig,
    mode: TelemetryMode,
    user_id: Option<String>,
    team_id: Option<String>,
    deployment_key: Option<String>,
    origin_client: Option<OriginClientInfo>,
    shell_version: String,
    subscription_tier: Option<String>,
    http_client: reqwest::Client,
) {
    let _ = (
        config,
        mode,
        user_id,
        team_id,
        deployment_key,
        origin_client,
        shell_version,
        subscription_tier,
        http_client,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn init_if_needed(
    config: TelemetryConfig,
    mode: TelemetryMode,
    user_id: Option<String>,
    team_id: Option<String>,
    deployment_key: Option<String>,
    origin_client: Option<OriginClientInfo>,
    shell_version: String,
    subscription_tier: Option<String>,
    http_client: reqwest::Client,
) {
    let _ = (
        config,
        mode,
        user_id,
        team_id,
        deployment_key,
        origin_client,
        shell_version,
        subscription_tier,
        http_client,
    );
}
