sdk/typescript/src/mobile/DeviceTrustModule.ts

```typescript
// React Native module that adds device-specific signals unavailable in browser.
import { NativeModules } from 'react-native';
import { DeviceTrustClient, DeviceSignals } from '../client';

const { RNDeviceInfo } = NativeModules;

export class MobileDeviceTrustClient extends DeviceTrustClient {
  async collectSignals(): Promise<DeviceSignals> {
    const baseSignals = await super.collectSignals();
    // Augment with native mobile signals
    const isEmulator = await RNDeviceInfo.isEmulator();
    const isJailbroken = await RNDeviceInfo.isJailbroken();
    return {
      ...baseSignals,
      is_emulator: isEmulator,
      is_rooted_jailbroken: isJailbroken,
      // Add more Android/iOS specific fields
    };
  }
}
```
