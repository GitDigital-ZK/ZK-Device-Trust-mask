sdk/es/device-trust.js

```javascript
// Lightweight ES module wrapper that lazily loads the WASM module.
let clientPromise = null;

async function getClient() {
  if (!clientPromise) {
    const { DeviceTrustClient } = await import('@gitdigital/zk-device-trust-mask');
    clientPromise = new DeviceTrustClient();
  }
  return clientPromise;
}

export async function attestDevice() {
  const client = await getClient();
  return client.attest();
}
```
