risk_engine/src/lib.rs

```rust
pub mod scorer;
pub mod signals;

pub use scorer::RiskScore;
pub use signals::DeviceSignals;

/// Evaluate the trustworthiness of a device given a set of signals.
/// Returns a risk score between 0 (highly risky) and 100 (fully trusted).
pub fn evaluate_device(signals: &DeviceSignals) -> RiskScore {
    scorer::compute_score(signals)
}
```
