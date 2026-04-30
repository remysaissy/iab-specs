# How to contribute

So, you've decided to contribute, that's great!

You can use this document to figure out how and where to start.

## Getting started

- Make sure you have a [GitHub account](https://github.com/join).
- Take a look at [existing issues](https://github.com/remysaissy/iab-specs/issues).
- If you need to create an issue:
    - Make sure to clearly describe it.
    - Including steps to reproduce when it is a bug.
    - Include the version of iab-specs used.

## Signing Your Commits

This repository requires signed commits. GitHub supports several signing methods — GPG, SSH, and S/MIME. Choose whichever works best for you.

For setup instructions, see GitHub's official documentation:

- **[Signing commits](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits)** — how to sign with GPG or SSH
- **[Generating a new GPG key](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key)** — if you don't have a GPG key yet
- **[Adding a GPG key to your GitHub account](https://docs.github.com/en/authentication/managing-commit-signature-verification/adding-a-gpg-key-to-your-github-account)** — so GitHub can verify your signatures
- **[Telling Git about your signing key](https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key)** — configuring Git to sign automatically

### Quick Setup (GPG)

Once your GPG key is configured with GitHub:

```bash
# Enable automatic signing for commits and tags
git config --local commit.gpgsign true
git config --local tag.gpgsign true
```

### Verifying Signatures

```bash
# View signature details in the git log
git log --show-signature -1
```

### CI Verification

All pull requests are automatically checked for valid commit signatures. Unsigned commits will cause the check to fail.

## Making changes

- Fork the repository on GitHub.
- Create a branch on your fork.
    - You can usually base it on the `main` branch.
    - Make sure not to commit directly to `main`.
- Make commits of logical and atomic units.
- **Sign all your commits** (see [Signing Your Commits](#signing-your-commits) above).
- **Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)** for all commit messages. This is enforced by CI on pull requests.
- Make sure you have added the necessary tests for your changes.
- Push your changes to a topic branch in your fork of the repository.
- Submit a pull request to the original repository.

### Commit Message Format

All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification:

```
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`.

Examples:
```
feat(openrtb): add OpenRTB 2.7 support
fix: prevent panic on empty ads.txt content
docs: update README with sellers.json examples
chore(deps): bump serde to 1.0.200
```

This format is used by [git-cliff](https://git-cliff.org/) to auto-generate the changelog.

## What to work on

We try to mark issues with a suggested level of experience (in Rust/SQL).
Where possible we try to spell out how to go about implementing the feature.

To start with, check out:
- Issues labeled as ["good first issue"](https://github.com/remysaissy/iab-specs/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).
- Issues labeled as ["Easy"](https://github.com/remysaissy/iab-specs/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy).

Additionally, it's always good to work on improving/adding examples and documentation.

## Development Setup

### Quick Start

The easiest way to set up your development environment is using the provided initialization script:

```bash
# Clone the repository
git clone https://github.com/remysaissy/iab-specs.git
cd iab-specs

# Run the setup script (installs all required tools)
./init-dev.sh

# Or for minimal setup (essential tools only)
./init-dev.sh --minimal
```

This script will:
- Install Rust 1.70 (MSRV) or later (Edition 2021)
- Install required components: rustfmt, clippy, rust-src, rust-docs, rust-analyzer
- Install cargo-llvm-cov for code coverage
- Install cargo-edit for dependency management (optional)
- Verify the installation
- Test that the project compiles

### Manual Setup

If you prefer to set up manually:

**Prerequisites:**
- Rust 1.70 or later (MSRV, Edition 2021)
- rustfmt: `rustup component add rustfmt`
- clippy: `rustup component add clippy`
- rust-src: `rustup component add rust-src`
- cargo-llvm-cov: `cargo install cargo-llvm-cov`
- act (optional, for running CI tests locally): `brew install act` (macOS) or see [act installation](https://github.com/nektos/act)
- Docker (required if using act)

### Building the Project

This project is a Cargo workspace. You can build/test the entire workspace or target individual crates:

```bash
# Build the entire workspace
cargo build --workspace --all-features

# Build only the umbrella crate
cargo build --all-features

# Run tests across all workspace crates
cargo test --workspace --all-features

# Run tests for a specific sub-crate
cargo test -p iab-specs-openrtb --all-features

# Run tests for the umbrella crate only
cargo test --all-features

# Build documentation
cargo doc --open
```

### Running CI Tests Locally

Before committing, you can run all GitHub Actions CI tests locally using `act`:

```bash
# Install act (one-time setup)
brew install act  # macOS
# or follow instructions at https://github.com/nektos/act

# Ensure Docker is running
# Then run all CI jobs
./test.sh

# Run specific jobs only
./test.sh --format      # Run format check
./test.sh --check       # Run clippy
./test.sh --test        # Run unit tests
./test.sh --examples    # Run examples
./test.sh --coverage    # Run coverage check

# List all available jobs
./test.sh --list

# Run a specific job by name
./test.sh --job coverage
```

This allows you to catch CI failures before pushing, saving time and ensuring your PR is ready for review.

**Note**: Running all jobs locally can take several minutes. Consider running specific jobs during development and run all jobs before committing.

### Code Coverage

Iab-Specs uses `cargo-llvm-cov` for code coverage reporting:

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate HTML coverage report (opens in browser)
./coverage.sh

# Generate lcov report for CI
./coverage.sh --lcov

# Show coverage summary in terminal
./coverage.sh --text

# Generate all formats
./coverage.sh --all

# Check coverage meets thresholds (80% minimum)
./coverage.sh --check-thresholds

# Combine flags
./coverage.sh --clean --html --check-thresholds
```

#### Coverage Requirements

Iab-Specs enforces minimum coverage thresholds in CI:
- **Line Coverage**: ≥ 80%
- **Region Coverage**: ≥ 80%
- **Function Coverage**: ≥ 80%

Pull requests that reduce coverage below these thresholds will fail CI. Coverage reports are automatically generated and uploaded to codecov.io. When contributing, aim to maintain or improve coverage for modified code.

### Inspecting Generated Code

To see what code Iab-Specs generates, use `cargo-expand`:

```bash
# Install cargo-expand
cargo install cargo-expand

# Expand macros in tests
cd iab-specs
cargo expand --test main
```

This is helpful for:
- Understanding how the macro works
- Debugging macro issues
- Verifying generated code correctness

### Code Style

- Follow standard Rust formatting: `cargo fmt`
- Ensure code passes clippy: `cargo clippy -- -D warnings`
- Add documentation comments for public APIs
- Keep generated code clean and readable

### Documentation

When contributing:
- Update README.md if adding user-facing features
- Add rustdoc comments for public APIs
- Create examples for significant features
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

## Communication

If you're unsure about your contribution or simply want to ask a question about anything, you can:
- Discuss something directly in the [Github issue](https://github.com/remysaissy/iab-specs/issues).

## Repository Rules

This repository enforces the following policies via GitHub branch protection and CI:

- **Signed commits required**: All commits pushed to `main` (and included in pull requests) must be cryptographically signed. See [Signing Your Commits](#signing-your-commits) for setup instructions.
- **CI must pass**: Format, clippy, tests, coverage (≥ 80%), and signature verification checks must all pass before merging.
- **Vigilant mode**: Repository maintainers use GitHub's vigilant mode, which marks all unsigned commits as "Unverified."

If you're a first-time contributor and need help setting up commit signing, feel free to open an issue — we're happy to help.

## Release Process (Maintainers)

The project follows a `X.Y.Z-dev → X.Y.Z` release cycle. The `-dev` suffix in
`Cargo.toml` signals ongoing development; it is never published to crates.io.

### Requirements

```bash
cargo install git-cliff   # changelog generation
brew install gh           # GitHub CLI
cargo login               # crates.io authentication
```

### Typical lifecycle

```
main branch: 0.5.1-dev  →  (feature work)  →  0.5.1-dev  →  release PR  →  0.5.2  →  0.5.2-dev  →  …
```

#### 1. During development

All feature/fix commits land on `main`. The version stays `0.5.1-dev` throughout.

#### 2. Prepare the release PR

When ready to ship, run `bump-version.sh --release` from `main`:

```bash
# Auto-derive next version from conventional commits:
./bump-version.sh --release

# Or force a specific bump type:
./bump-version.sh --release --minor
./bump-version.sh --release --major
./bump-version.sh --release --revision
```

This script will:
1. Strip `-dev`, compute the next version (via `git-cliff --bumped-version` or explicit flag)
2. Create a `release/vX.Y.Z` branch
3. Update all `Cargo.toml` files and `Cargo.lock`
4. Generate `CHANGELOG.md` with `git-cliff`
5. Commit (signed), push the branch, and open a PR

Review and merge the PR. **Do not squash-merge** — the commit history is used by git-cliff.

#### 3. Create the GitHub release and publish

After the PR is merged and you are on `main`:

```bash
git checkout main && git pull
./release.sh
```

This script will:
1. Create a GitHub release and tag (`vX.Y.Z`) from `CHANGELOG.md`
2. Fetch the tag locally
3. Publish all workspace crates to crates.io in topological dependency order

#### 4. Start the next development cycle

```bash
./bump-version.sh --post-release
git commit -m 'chore: begin development on next release'
git push
```

This adds the `-dev` suffix back to the version (e.g. `0.5.2` → `0.5.2-dev`).

### Mid-cycle re-plan

If the scope of the next release changes significantly mid-cycle, you can re-plan the target version while keeping the `-dev` suffix:

```bash
./bump-version.sh --minor    # 0.5.1-dev → 0.6.0-dev
./bump-version.sh --major    # 0.5.1-dev → 1.0.0-dev
./bump-version.sh --revision # 0.5.1-dev → 0.5.2-dev
git commit -m 'chore: re-plan next release as 0.6.0-dev'
git push
```

### Dry run

All version commands support `--dry-run` to preview actions without modifying files:

```bash
./bump-version.sh --release --dry-run
./bump-version.sh --post-release --dry-run
./release.sh --dry-run
```

## Code of Conduct

Be respectful, constructive, and welcoming to all contributors.
