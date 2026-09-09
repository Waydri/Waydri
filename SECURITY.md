# Security

Waydri runs an unprivileged Wayland compositor. This page describes the
security model and how to report issues.

## Reporting a vulnerability

Do not open a public issue for a security problem. Report it privately to
the maintainers by email. Maintainer addresses are listed in MAINTAINERS.

Include in the report:

- The affected version.
- The platform and hardware.
- A minimal reproduction.
- The impact and any exploit code.

The maintainers will acknowledge the report within 72 hours, coordinate a
fix, and publish a security notice once the fix is available.

## Supported versions

Security fixes are released for the latest minor version only.

| Version | Supported          |
|---------|--------------------|
| 0.11.x  | yes                |
| older   | no                 |

## Threat model

- IPC commands are accepted from local users only. The socket is created with
  owner-only permissions and rejects connections from other users.
- Plugin code runs in the compositor process. Plugins must be treated as
  trusted until the sandbox lands.
- The software renderer validates all surface buffers before copying pixels.
- Configuration files are read-only inputs and never executed.

## Hardening

- Run Waydri as a dedicated unprivileged user on Linux.
- Do not expose the IPC socket over the network.
- Keep the compositor and the Android app up to date.

## Responsible disclosure

We ask that you not publish details of a vulnerability before a fix has been
released and users have had time to update.