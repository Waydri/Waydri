# Security Policy

## Reporting a Vulnerability

Please report security vulnerabilities privately. Do not create a public issue for security bugs.

To report a vulnerability, email `security@waydri.org` or open a draft security advisory on GitHub using the "Report a vulnerability" feature on the repository.

Please include:

- A description of the vulnerability
- The affected version(s) and component
- Steps to reproduce, if available
- Any proposed fix or mitigation

We aim to acknowledge reports within 48 hours and provide a progress update within one week.

## Scope

This policy covers the Waydri compositor, its IPC interface, the plugin system, and the Android/Linux integration layers.

## Sandbox Capabilities

The plugin sandbox gates access through declared capabilities. Sensitive operations (process execution, input injection, state mutation, IPC, filesystem, network) require the corresponding capability. Plugins that request ungranted capabilities fail to load. See [PLUGINS.md](PLUGINS.md).

## Permission Model

- The IPC Unix socket is created with mode 0600, accessible only to the owning user.
- The Wayland socket is created under `$XDG_RUNTIME_DIR`.
- On Linux, device access relies on systemd logind session grants or the `uaccess` udev tag rather than global permissions.
- On Android, the app requests only the permissions required for compositor operation.

## Disclosure Policy

We follow a coordinated disclosure process:

1. The reporter privately notifies the maintainers.
2. Maintainers triage, reproduce, and fix the issue.
3. A fix is prepared and a release is cut.
4. The vulnerability is disclosed publicly after the fix is available.

## Security Roles

- All maintainers triage and evaluate incoming reports.
- Critical vulnerabilities are assigned a high priority and expedited release.
- Non-critical findings are queued for the next release alongside documentation.

## Security Testing

Contributors are encouraged to include tests for security-relevant behavior in `tests/`. The plugin capability checks and IPC permission handling have dedicated test coverage.
