# Contributing to Glimpse Launcher

First off, thank you for considering contributing to Glimpse Launcher! 🚀

Glimpse Launcher is an ultralight desktop launcher for Windows 11, built with Rust for maximum performance and minimal resource usage. Every contribution — whether it's a bug report, a feature request, or a pull request — helps make Glimpse better for everyone.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Code Style](#code-style)
- [Branch Naming Convention](#branch-naming-convention)
- [Commit Convention](#commit-convention)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Bugs](#reporting-bugs)
- [Feature Requests](#feature-requests)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior as described in the Code of Conduct.

## Getting Started

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/GlimpseLauncher.git
   cd GlimpseLauncher
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/DevFreitas/GlimpseLauncher.git
   ```
4. **Create a branch** for your work (see [Branch Naming Convention](#branch-naming-convention)).
5. **Make your changes**, ensuring they follow our [Code Style](#code-style).
6. **Submit a Pull Request** (see [Submitting a Pull Request](#submitting-a-pull-request)).

## Development Environment

### Prerequisites

| Tool       | Version     | Notes                              |
| ---------- | ----------- | ---------------------------------- |
| **Rust**   | Stable      | Managed via `rust-toolchain.toml`  |
| **OS**     | Windows 10+ | Windows 11 recommended            |
| **Target** | `x86_64-pc-windows-msvc` | MSVC toolchain required |

### Setup

1. Install [Rust](https://rustup.rs/) via `rustup`. The project's `rust-toolchain.toml` will automatically select the correct toolchain.

2. Build the project:
   ```bash
   cargo build
   ```

3. Run in development mode:
   ```bash
   cargo run
   ```

4. Run the test suite:
   ```bash
   cargo test
   ```

5. Check for linting issues:
   ```bash
   cargo clippy -- -D warnings
   ```

6. Format your code:
   ```bash
   cargo fmt
   ```

### Recommended IDE

- **VS Code** with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension.
- Enable format-on-save with `rustfmt`.

## Code Style

We enforce consistent code style through automated tooling:

- **`rustfmt`** — All code must be formatted with `rustfmt` using the project's [`rustfmt.toml`](rustfmt.toml) configuration. Run `cargo fmt` before committing.
- **`clippy`** — All code must pass `clippy` lints with no warnings. Run `cargo clippy -- -D warnings` to verify. Configuration is in [`clippy.toml`](clippy.toml).

### General Guidelines

- Prefer **safe Rust** whenever possible. Document any `unsafe` usage with a `// SAFETY:` comment explaining the invariants.
- Keep functions focused and small. If a function exceeds ~50 lines, consider refactoring.
- Use meaningful variable and function names. Abbreviations are acceptable only when well-established (e.g., `ctx`, `cfg`).
- Write doc comments (`///`) for all public items.
- Add `#[must_use]` to functions where ignoring the return value is likely a bug.

## Branch Naming Convention

Use the following prefixes for your branches:

| Prefix      | Purpose                          | Example                          |
| ----------- | -------------------------------- | -------------------------------- |
| `feature/`  | New features or enhancements     | `feature/plugin-system`          |
| `fix/`      | Bug fixes                        | `fix/multi-monitor-positioning`  |
| `docs/`     | Documentation changes            | `docs/update-readme`             |
| `refactor/` | Code refactoring (no behavior change) | `refactor/search-engine`    |
| `test/`     | Adding or improving tests        | `test/fuzzy-search-coverage`     |
| `chore/`    | Maintenance tasks                | `chore/update-dependencies`      |

Always branch from `main` with the latest changes:

```bash
git checkout main
git pull upstream main
git checkout -b feature/your-feature-name
```

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type         | Description                                  |
| ------------ | -------------------------------------------- |
| `feat`       | A new feature                                |
| `fix`        | A bug fix                                    |
| `docs`       | Documentation only changes                   |
| `style`      | Formatting, missing semicolons, etc.         |
| `refactor`   | Code change that neither fixes a bug nor adds a feature |
| `perf`       | A code change that improves performance      |
| `test`       | Adding missing tests or correcting existing ones |
| `build`      | Changes to the build system or dependencies  |
| `ci`         | Changes to CI configuration                  |
| `chore`      | Other changes that don't modify src or tests |

### Examples

```
feat(search): add inline calculator for math expressions
fix(ui): correct window positioning on multi-monitor setups
docs(readme): update installation instructions
perf(indexer): optimize UWP app discovery with parallel iteration
```

### Breaking Changes

Append `!` after the type/scope or add `BREAKING CHANGE:` in the footer:

```
feat(config)!: change configuration file format to TOML

BREAKING CHANGE: Configuration files must be migrated from JSON to TOML.
```

## Submitting a Pull Request

1. **Ensure your branch is up to date** with `main`:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run all checks** before pushing:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```

3. **Push your branch** and open a PR on GitHub.

4. **In your PR description**, include:
   - A clear description of what the PR does and why.
   - Reference any related issues (e.g., `Closes #42`).
   - Screenshots or GIFs for UI changes.
   - Any breaking changes or migration steps.

5. **Be responsive** to code review feedback. We aim to review PRs within a few days.

### PR Checklist

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt --check`)
- [ ] Clippy passes (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated if needed
- [ ] Commit messages follow Conventional Commits
- [ ] CHANGELOG.md is updated for user-facing changes

## Reporting Bugs

Found a bug? Please help us fix it by [opening an issue](https://github.com/DevFreitas/GlimpseLauncher/issues/new) with the following information:

- **Glimpse Launcher version** (check `--version` or About dialog)
- **Windows version** (e.g., Windows 11 23H2)
- **Steps to reproduce** the issue
- **Expected behavior** vs. **actual behavior**
- **Screenshots or logs** if applicable
- **System information** (display setup, scaling, relevant software)

## Feature Requests

We welcome feature ideas! Please [open an issue](https://github.com/DevFreitas/GlimpseLauncher/issues/new) and describe:

- **The problem** you're trying to solve.
- **Your proposed solution** or how you envision the feature.
- **Alternatives** you've considered.

---

Thank you for helping make Glimpse Launcher faster, lighter, and better! ⚡
