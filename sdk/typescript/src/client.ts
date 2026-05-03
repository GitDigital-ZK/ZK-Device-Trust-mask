sdk/typescript/src/client.ts

```typescript
import init, { collect_signals, evaluate_trust, generate_fingerprint } from '@gitdigital/device-trust-wasm';

// Type definitions matching the Rust structs
export interface DeviceSignals {
  os_version: string;
  secure_boot_enabled: boolean;
  is_emulator: boolean;
  is_rooted_jailbroken: boolean;
  days_since_last_patch?: number;
  current_ip_risk?: number;
  hardware_id?: string;
}

export interface ScoreDetails {
  os_integrity: number;
  patch_level: number;
  secure_boot: number;
  environment_consistency: number;
  network_trust: number;
}

export interface RiskScore {
  score: number;
  details: ScoreDetails;
}

export interface DeviceFingerprint {
  raw_hash: string;
  platform: string;
}

export class DeviceTrustClient {
  private wasmReady: Promise<void>;

  constructor(wasmPath?: string) {
    this.wasmReady = init(wasmPath); // Load WASM module
  }

  async collectSignals(): Promise<DeviceSignals> {
    await this.wasmReady;
    return collect_signals() as any;
  }

  async evaluateTrust(signals: DeviceSignals): Promise<RiskScore> {
    await this.wasmReady;
    return evaluate_trust(signals) as any;
  }

  async generateFingerprint(signals: DeviceSignals): Promise<DeviceFingerprint> {
    await this.wasmReady;
    return generate_fingerprint(signals) as any;
  }

  /** Full attestation flow: signals -> risk -> fingerprint -> (optionally) proof */
  async attest() {
    const signals = await this.collectSignals();
    const score = await this.evaluateTrust(signals);
    const fingerprint = await this.generateFingerprint(signals);
    return { signals, score, fingerprint };
  }
}
```
