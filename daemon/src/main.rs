daemon/src/main.rs

```rust
use axum::{routing::post, Json, Router};
use risk_engine::{evaluate_device, DeviceSignals, RiskScore};
use sdk_rust::proof::verify_attestation_proof;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::init();

    let cache = Arc::new(Mutex::new(std::collections::HashMap::<String, RiskScore>::new()));

    let app = Router::new()
        .route("/attest", post(attest_handler))
        .with_state(cache);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Daemon listening on :3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct AttestRequest {
    device_signals: DeviceSignals,
    proof: String,  // base64‑encoded proof data
}

#[derive(Debug, Serialize)]
struct AttestResponse {
    device_id: String,      // UUID‑v5 derived from hardware_id
    trust_score: u8,
    verdict: String,
}

async fn attest_handler(
    state: axum::extract::State<Arc<Mutex<std::collections::HashMap<String, RiskScore>>>>,
    Json(payload): Json<AttestRequest>,
) -> Json<AttestResponse> {
    // 1. Evaluate risk
    let score = evaluate_device(&payload.device_signals);
    let device_id = uuid::Uuid::new_v4().to_string(); // deterministic derivation can be used

    // 2. Verify ZK proof (placeholder — in production, decode and verify)
    let proof_valid = verify_attestation_proof(&payload.proof).unwrap_or(false);
    if !proof_valid {
        return Json(AttestResponse {
            device_id,
            trust_score: 0,
            verdict: "proof_invalid".into(),
        });
    }

    // 3. Cache result
    {
        let mut cache = state.lock().await;
        cache.insert(device_id.clone(), score.clone());
    }

    let verdict = if score.score >= 70 {
        "trusted"
    } else {
        "untrusted"
    };

    Json(AttestResponse {
        device_id,
        trust_score: score.score,
        verdict: verdict.into(),
    })
}
```
