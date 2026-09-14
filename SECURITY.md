# Security policy

## Reporting a vulnerability

Report suspected vulnerabilities through GitHub's private vulnerability reporting on this repository:
open the **Security** tab and choose **Report a vulnerability**. That channel is private between you
and the maintainer. Please do not open a public issue for a suspected vulnerability.

This is a single-maintainer, pre-1.0 prototype. There is no guaranteed response time; reports are
read and answered as time allows. Do not wait for a fix before protecting yourself.

## Supported versions

Only the current `main` branch and the most recent tagged release receive fixes. There are no
long-term-support branches.

## In scope

deltasafe is an authenticated file-transfer tool for a **trusted LAN**. In scope:

- Breaking the confidentiality or integrity of a transfer between two peers that share a key.
- Producing a frame that authenticates without the shared key, or forging a final status.
- Causing a receiver to publish a file that does not match the sender's declared size and BLAKE3
  digest.
- Escaping the receive root through a crafted relative path, a symlink parent, or a destination
  collision that overwrites an existing file.
- A denial of service against the receiver from an unauthenticated peer that is materially worse
  than the protocol's bounded frame and header sizes imply.

## Out of scope

These are documented non-goals, not vulnerabilities:

- No certificate-based peer identity, no TLS, and no forward secrecy.
- No durable replay protection across receiver restarts.
- No protection against a local administrator who can modify the receive directory during a
  transfer, or against OS-specific TOCTOU races that standard-library path checks cannot close.
- No disk-quota or general resource-exhaustion protection beyond the bounded frame and header sizes.
- Anyone who already holds the shared password or key is trusted by design.

## No audit

This project has not been independently audited or penetration-tested. The test suite and the
recorded loopback evidence under [`docs/validation/`](docs/validation/) show that specific scenarios
behave as documented. They are not a general security certification.

## Deployment boundary

Keep the receiver on a network you control, and do not expose the listening port directly to the
public internet.
