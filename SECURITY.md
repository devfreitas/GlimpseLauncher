# Security Policy

## Supported Versions

Only the latest release is actively supported for security updates.

| Version | Supported          |
| ------- | ------------------ |
| 0.7.x   | :white_check_mark: |
| < 0.7.0 | :x:                |

## Reporting a Vulnerability

Please report security issues via GitHub Security Advisories or by contacting the author (DevFreitas).

We aim to acknowledge receipt of vulnerability reports within **48 hours**.

## Scope of Vulnerabilities

The following types of issues are considered within scope for security reports:
- Arbitrary code execution without explicit user intent.
- Privilege escalation.
- Memory safety issues (since this is a Rust project, these are of high interest).

**Out of Scope:**
- **Terminal Command Execution**: The `> ` prefix feature is an intentional design choice to allow the user to execute shell commands. This is not considered a vulnerability, as it runs with the user's current privileges and requires explicit user input to trigger.
