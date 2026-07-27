# WP01 CSP Baseline

The release configuration uses local assets only and declares:

```text
default-src 'self';
script-src 'self';
style-src 'self';
img-src 'self' data: asset:;
font-src 'self';
connect-src 'self' ipc: http://ipc.localhost;
object-src 'none';
frame-ancestors 'none';
base-uri 'self';
form-action 'none';
```

No remote origin or broad `unsafe-inline` relaxation is configured. Tauri's
release asset pipeline is expected to inject the CSP nonces/hashes required by
bundled scripts and runtime Fluent/Griffel styles.

The Vite release build exercises the actual Fluent components and Griffel
bundle. Runtime CSP rendering remains `Blocked` on the Work Session host because
Rust, WebKitGTK development libraries, and a display server are unavailable.
WP01 therefore makes no visible-window or CSP-runtime pass claim.
