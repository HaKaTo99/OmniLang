# OmniLang Specification: Security & Capabilities

**Status**: Stable (v2.1.0 Security)
**Version**: v2.1.0

OmniLang enforces a capability-based security model. Access to sensitive resources (files, network, hardware) or remote execution via `@mesh` must be explicitly authorized. 

In version **v2.1.0**, this is implemented via **X-Capability Tokens** for Mesh RPC communications.

### Capability Definition
To define a new capability type:

```omni
capability FileAccess {
    path: string;
    mode: "read" | "write";
}
```

### Requiring Capabilities (`@requires`)
Functions that perform sensitive operations must be annotated with `@requires`.

```omni
@requires(FileAccess)
fn readFile(path: string) -> string {
    // Verified by compiler: caller must possess a valid FileAccess token
    return sys.read(path);
}
```

### Passing Capabilities
To call a restricted function, a capability token must be passed.

```omni
fn main() {
    // In a real scenario, tokens are granted by the kernel/loader
    let token = acquireTokenFor("/etc/config");
    readFile("/etc/config") using token; 
}
```

### Implementation: X-Capability Tokens (v2.1.0)
The current implementation focuses on Mesh RPC security:

1. **Worker Enforcement**: The node server rejects any request that does not contain a matching `capability_token`.
2. **Implicit Shadowing**: The client engine automatically looks for a global constant `X_CAPABILITY_TOKEN` to inject into outcoming `@mesh` calls.

```omni
// Example: Sensor Node with Capability
const X_CAPABILITY_TOKEN: String = "secure-fabric-key";

@mesh(target: "127.0.0.1:8081")
fn trigger_alarm();

fn main() {
    trigger_alarm(); // Token injected automatically
}
```

## 2. Post-Quantum Cryptography (`@pqc`)

OmniLang integrates PQC algorithms directly into the language via the `@pqc` annotation.

```omni
@pqc(algorithm: "kyber-1024")
fn secureChannel(peer: Device) -> Channel {
    // Automatically uses Kyber-1024 for key exchange
    return connection.handshake(peer);
}
```

## 3. Zero-Trust Architecture

By default, code has **zero capabilities**.
- No file access.
- No network access.
- No memory access outside its own stack/heap.

This is enforced by the compiler and the xAetherOS runtime.
