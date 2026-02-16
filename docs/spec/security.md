# OmniLang Specification: Security & Capabilities

**Status**: Draft
**Version**: v0.1

## 1. Capability-Based Security

OmniLang enforces a capability-based security model. Access to sensitive resources (files, network, hardware) must be explicitly granted via capability tokens.

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
