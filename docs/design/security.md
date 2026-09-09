# Security Design

## Threat Model

Waydri runs as a system compositor with elevated privileges. The plugin system and IPC interface are potential attack surfaces. The security model focuses on containing damage from compromised components.

## Capability-Based Access Control

Plugins and external tools operate under a capability-based permission model. Each capability grants access to a specific system resource.

### Capability Types

| Capability | Grants Access |
|-----------|---------------|
| `network` | Outbound network connections to specified hosts |
| `fs_read` | Read access to specified filesystem paths |
| `fs_write` | Write access to specified filesystem paths |
| `compositor` | Read/write access to compositor state |
| `input` | Ability to generate synthetic input events |
| `rpc` | IPC server and client communication |
| `animation` | Access to animation and effects pipeline |

### Capability Enforcement

Capabilities are declared in plugin manifests and enforced at runtime:

```json
{
    "capabilities": ["compositor", "rpc"],
    "sandbox": {
        "policy": "strict",
        "network_hosts": [],
        "filesystem_read": [],
        "filesystem_write": []
    }
}
```

In strict mode, any capability violation is denied and logged. The violating plugin is not terminated but has the operation blocked.

## Sandbox Policies

### Strict (Default)

Only explicitly declared capabilities are permitted. All other operations are denied. This is the recommended policy for third-party plugins.

### Permissive

Operations beyond the declared capabilities are allowed but logged as warnings. Useful during plugin development.

### Disabled

No sandbox enforcement. Only for trusted first-party plugins that ship with Waydri.

## Plugin Isolation

Plugins run in the same process as the compositor but are isolated through:

1. **Capability checks**: Every API call is validated against the plugin's declared capabilities.
2. **Resource limits**: Plugins are subject to memory and CPU time limits.
3. **Error containment**: Plugin panics are caught and the plugin is unloaded without crashing the compositor.
4. **Sandbox directories**: Each plugin has an isolated working directory for temporary files.

## IPC Security

### Socket Permissions

The IPC Unix socket is created with mode 0600 (owner read/write only). Only the compositor process and the user running it can access the socket.

### Connection Validation

Incoming IPC connections are validated:

1. Check peer UID matches the compositor owner.
2. Check peer PID is a valid process.
3. Rate-limit connections to prevent denial of service.
4. Timeout inactive connections after 30 seconds.

### Input Validation

All IPC command arguments are validated before execution. Malformed commands are rejected with an error response and logged.

## Memory Safety

Rust's ownership system prevents most memory safety bugs. Additional measures:

- DMA-BUF handles are validated before use.
- Shared memory pools use bounds-checked access.
- No unsafe code blocks without documented justification.
- Fuzz testing covers IPC parsing and protocol handling.

## Secure Defaults

- Plugins are disabled by default.
- IPC socket requires explicit configuration.
- XWayland is disabled by default.
- Debug logging is disabled in release builds.
- Core dumps are disabled for the compositor process.
