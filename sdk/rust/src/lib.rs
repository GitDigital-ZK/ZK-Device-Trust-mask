sdk/rust/src/lib.rs

```rust
pub mod types;
pub mod proof;

use risk_engine::{evaluate_device, DeviceSignals, RiskScore};
use types::DeviceFingerprint;

/// Main entrypoint for the device‑trust SDK.
pub struct DeviceTrustClient;

impl DeviceTrustClient {
    /// Gather device signals from the environment.
    /// On native platforms this probes the OS; on WASM it uses browser APIs.
    pub fn collect_signals() -> Result<DeviceSignals, Box<dyn std::error::Error>> {
        #[cfg(target_arch = "wasm32")]
        {
            // Use web APIs (example placeholder)
            Ok(DeviceSignals {
                os_version: "web-unknown".into(),
                secure_boot_enabled: false,
                is_emulator: false,
                is_rooted_jailbroken: false,
                days_since_last_patch: None,
                current_ip_risk: None,
                hardware_id: None,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use native system calls (Linux, macOS, Windows)
            Ok(DeviceSignals {
                os_version: std::env::consts::OS.to_string(),
                secure_boot_enabled: false, // implement detection
                is_emulator: false,
                is_rooted_jailbroken: false,
                days_since_last_patch: None,
                current_ip_risk: None,
                hardware_id: Some("machine-id".into()),
            })
        }
    }

    /// Compute risk score from collected signals.
    pub fn evaluate_trust(signals: &DeviceSignals) -> RiskScore {
        evaluate_device(signals)
    }

    /// Generate a device fingerprint (deterministic ID from hardware).
    pub fn generate_fingerprint(signals: &DeviceSignals) -> DeviceFingerprint {
        let id = signals.hardware_id.clone().unwrap_or("unknown".into());
        DeviceFingerprint {
            raw_hash: blake3::hash(id.as_bytes()).to_hex().to_string(),
            platform: std::env::consts::OS.to_string(),
        }
    }
}

// Simple blake3 helper
mod utils {
    pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}
```

sdk/rust/src/types.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    pub raw_hash: String,
    pub platform: String,
}
```
