risk_engine/src/scorer.rs

```rust
use crate::signals::DeviceSignals;
use serde::Serialize;
use std::cmp;

/// Final risk score: 0–100, higher is more trusted.
#[derive(Debug, Clone, Serialize)]
pub struct RiskScore {
    pub score: u8,
    pub details: ScoreDetails,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScoreDetails {
    pub os_integrity: u8,
    pub patch_level: u8,
    pub secure_boot: u8,
    pub environment_consistency: u8,
    pub network_trust: u8,
}

/// Compute a holistic score from individual signal assessments.
/// Weights are derived from policy and can be customized.
pub fn compute_score(signals: &DeviceSignals) -> RiskScore {
    let os_integrity = check_os_integrity(signals);
    let patch_level = check_patch_level(signals);
    let secure_boot = check_secure_boot(signals);
    let env_consistency = check_environment(signals);
    let net_trust = check_network(signals);

    let weights = Weights::default();
    let raw = (os_integrity as u16 * weights.os_integrity
        + patch_level as u16 * weights.patch_level
        + secure_boot as u16 * weights.secure_boot
        + env_consistency as u16 * weights.environment
        + net_trust as u16 * weights.network) as u16;

    // Normalize to 0–100
    let total_weight: u16 = weights.os_integrity
        + weights.patch_level
        + weights.secure_boot
        + weights.environment
        + weights.network;
    let score = cmp::min(100, (raw * 100) / (total_weight * 100)) as u8;

    RiskScore {
        score,
        details: ScoreDetails {
            os_integrity,
            patch_level,
            secure_boot,
            environment_consistency: env_consistency,
            network_trust: net_trust,
        },
    }
}

fn check_os_integrity(signals: &DeviceSignals) -> u8 {
    // Example: verify OS build fingerprint matches known good values
    if signals.os_version.contains("verified") {
        100
    } else {
        30
    }
}

fn check_patch_level(signals: &DeviceSignals) -> u8 {
    let days_since = signals.days_since_last_patch.unwrap_or(365);
    if days_since < 30 {
        100
    } else if days_since < 90 {
        70
    } else {
        20
    }
}

fn check_secure_boot(signals: &DeviceSignals) -> u8 {
    if signals.secure_boot_enabled { 100 } else { 0 }
}

fn check_environment(signals: &DeviceSignals) -> u8 {
    if signals.is_emulator || signals.is_rooted_jailbroken {
        0
    } else {
        90
    }
}

fn check_network(signals: &DeviceSignals) -> u8 {
    if signals.current_ip_risk.unwrap_or(0) > 70 {
        10
    } else {
        80
    }
}

struct Weights {
    os_integrity: u16,
    patch_level: u16,
    secure_boot: u16,
    environment: u16,
    network: u16,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            os_integrity: 30,
            patch_level: 25,
            secure_boot: 25,
            environment: 15,
            network: 5,
        }
    }
}
```
