#!/usr/bin/env bash

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

show_help() {
    cat << 'EOF'
Usage: ./release.sh [OPTION]

Create a GitHub release for the current version, then publish all crates to
crates.io. Run this after the release PR has been merged to main.

Options:
    --dry-run     Validate packaging without pushing or publishing
    --help        Display this help message

Workflow:
    1. Verify current branch is main with a clean working directory
    2. Confirm the version in Cargo.toml has no -dev suffix
    3. Create a GitHub release (and tag) for vX.Y.Z
    4. Fetch the tag locally
    5. Run ./publish.sh to publish all crates to crates.io

Requirements:
    - gh    (GitHub CLI, brew install gh)
    - cargo (Rust toolchain, cargo login for crates.io auth)

After the release is complete, mark the start of the next development cycle:
    ./bump-version.sh --post-release
    git commit -m 'chore: begin development on next release'
    git push

EOF
}

print_info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
print_error()   { echo -e "${RED}[ERROR]${NC} $1" >&2; }
print_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
print_step()    { echo -e "${BLUE}[STEP]${NC} $1"; }
print_dry_run() { echo -e "${YELLOW}[DRY-RUN]${NC} $1"; }

check_dependencies() {
    if ! command -v gh &>/dev/null; then
        print_error "GitHub CLI (gh) is not installed."
        echo "Install it: https://cli.github.com/"
        exit 1
    fi
}

check_clean_workdir() {
    if ! git diff --quiet || ! git diff --cached --quiet; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        exit 1
    fi
    if [ -n "$(git ls-files --others --exclude-standard)" ]; then
        print_error "There are untracked files. Please commit or remove them."
        exit 1
    fi
}

check_on_main() {
    local branch
    branch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$branch" != "main" ]; then
        print_error "Releases must be made from the 'main' branch (currently on '$branch')."
        exit 1
    fi
}

get_current_version() {
    grep -A5 '^\[workspace\.package\]' Cargo.toml | grep 'version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

main() {
    local dry_run=false

    while [ $# -gt 0 ]; do
        case "$1" in
            --dry-run) dry_run=true ;;
            --help|-h) show_help; exit 0 ;;
            *) print_error "Unknown option: $1"; show_help; exit 1 ;;
        esac
        shift
    done

    check_dependencies
    check_on_main
    check_clean_workdir

    local version
    version=$(get_current_version)

    if [[ "$version" == *-dev ]]; then
        print_error "Version '$version' still has a -dev suffix."
        print_error "Run './bump-version.sh --release' first to prepare the release PR."
        exit 1
    fi

    local tag="v${version}"

    echo ""
    print_info "Version: $version"
    print_info "Tag:     $tag"
    if [ "$dry_run" = true ]; then
        print_warning "DRY RUN — no push or publish will be performed"
    fi
    echo ""

    read -rp "Proceed with release ${tag}? (y/N) " -n 1
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Release cancelled."
        exit 0
    fi

    echo ""

    if [ "$dry_run" = true ]; then
        print_step "1/3 [DRY RUN] Skipping GitHub release creation"
        print_step "2/3 [DRY RUN] Skipping tag fetch"
        print_step "3/3 [DRY RUN] Validating crate packages..."
        ./publish.sh --dry-run
        echo ""
        print_info "Dry run complete. To finalize, run: ./release.sh"
        return
    fi

    print_step "1/3 Creating GitHub release ${tag}..."
    gh release create "$tag" \
        --target main \
        --title "Release ${tag}" \
        --notes-file CHANGELOG.md
    print_info "GitHub release ${tag} created"

    print_step "2/3 Fetching tag locally..."
    git pull --tags
    print_info "Tag ${tag} fetched"

    print_step "3/3 Publishing crates to crates.io..."
    ./publish.sh
    print_info "All crates published"

    echo ""
    print_info "Release ${tag} complete!"
    echo ""
    print_info "Start the next development cycle:"
    print_info "  ./bump-version.sh --post-release"
    print_info "  git commit -m 'chore: begin development on next release'"
    print_info "  git push"
}

main "$@"
