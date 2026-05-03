risk_engine/src/signals.rs

```rust
use serde::Deserialize;

/// Collection of device‑specific signals gathered by the SDK.
/// All fields are optional; the risk engine handles missing data gracefully.
#[derive(Debug, Clone, Deserialize)]
pub struct DeviceSignals {
    pub os_version: String,
    pub secure_boot_enabled: bool,
    pub is_emulator: bool,
    pub is_rooted_jailbroken: bool,
    pub days_since_last_patch: Option<u32>,
    pub current_ip_risk: Option<u8>,      // 0–100
    pub hardware_id: Option<String>,
}
```
