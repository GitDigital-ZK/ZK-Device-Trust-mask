sdk/rust/src/types.rs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    pub raw_hash: String,
    pub platform: String,
}
```
