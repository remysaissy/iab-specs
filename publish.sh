#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Flags
DRY_RUN=false

# Function to display help
show_help() {
    cat << EOF
Usage: $0 [OPTION]...

Publish all workspace crates to crates.io in topological dependency order.

Options:
    --dry-run     Run 'cargo publish --dry-run' for each crate (no actual publish)
    --help        Display this help message

Publish Order (topological dependency groups):

  Group 1: iab-specs-core
  Group 2: iab-specs-adcom, iab-specs-ads_txt, iab-specs-sellers_json,
           iab-specs-artb, iab-specs-agentic_audience,
           iab-specs-agentic_direct, iab-specs-openrtb_native
  Group 3: iab-specs-openrtb, iab-specs-app_ads_txt, iab-specs-buyer_agent,
           iab-specs-seller_agent, iab-specs-registry_agent
  Group 4: iab-specs (umbrella crate, published last)

  A 30-second wait is inserted between groups to allow crates.io to index
  newly published crates before dependents are published.

Prerequisites:
  - Working directory must be clean (no uncommitted changes)
  - Current commit must be tagged with the version from Cargo.toml (e.g., v0.4.0)
  - You must be logged in to crates.io (cargo login)

Examples:
    $0 --dry-run    # Validate all crates without publishing
    $0              # Publish all crates to crates.io

EOF
}

# Function to print colored messages
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_dry_run() {
    echo -e "${YELLOW}[DRY-RUN]${NC} $1"
}

print_group() {
    echo -e "${BLUE}[GROUP $1]${NC} $2"
}

# Get current version from [workspace.package] section in Cargo.toml
get_current_version() {
    grep -A5 '^\[workspace\.package\]' Cargo.toml | grep 'version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# Check prerequisites
check_prerequisites() {
    # Check for clean working directory
    if ! git diff --quiet || ! git diff --cached --quiet; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        exit 1
    fi

    # Check for untracked files
    if [ -n "$(git ls-files --others --exclude-standard)" ]; then
        print_error "There are untracked files. Please commit or remove them."
        exit 1
    fi

    # Check that current commit is tagged with the version
    local version
    version=$(get_current_version)
    local tag="v${version}"

    if ! git tag --points-at HEAD | grep -q "^${tag}$"; then
        print_error "Current commit is not tagged with '${tag}'."
        print_error "Please create the tag first: git tag -a ${tag} -m 'Release ${tag}'"
        exit 1
    fi

    print_info "Prerequisites check passed"
    print_info "  Version: ${version}"
    print_info "  Tag: ${tag}"
}

# Publish a single crate
publish_crate() {
    local crate_dir=$1
    local crate_name=$2
    local dry_run=$3

    if [ "$dry_run" = true ]; then
        print_dry_run "Publishing ${crate_name} (from ${crate_dir})..."
        if cargo publish --dry-run -p "${crate_name}" 2>&1; then
            print_dry_run "${crate_name} ✓ dry-run passed"
            return 0
        else
            print_error "${crate_name} ✗ dry-run failed"
            return 1
        fi
    else
        print_info "Publishing ${crate_name} (from ${crate_dir})..."
        if cargo publish -p "${crate_name}" 2>&1; then
            print_info "${crate_name} ✓ published successfully"
            return 0
        else
            print_error "${crate_name} ✗ publish failed"
            return 1
        fi
    fi
}

# Publish a group of crates
publish_group() {
    local group_num=$1
    local dry_run=$2
    shift 2
    # Remaining args are "dir:name" pairs
    local crates=("$@")
    local failed=0

    print_group "$group_num" "Publishing ${#crates[@]} crate(s)..."

    for entry in "${crates[@]}"; do
        local dir="${entry%%:*}"
        local name="${entry##*:}"
        if ! publish_crate "$dir" "$name" "$dry_run"; then
            failed=1
        fi
    done

    if [ "$failed" -ne 0 ]; then
        print_error "Group ${group_num} had failures. Aborting."
        exit 1
    fi

    print_group "$group_num" "All crates published successfully ✓"
}

# Wait between groups for crates.io indexing
wait_between_groups() {
    local group_from=$1
    local group_to=$2
    local dry_run=$3

    if [ "$dry_run" = true ]; then
        print_dry_run "Would wait 30s between group ${group_from} and group ${group_to} for crates.io indexing"
    else
        print_info "Waiting 30s for crates.io to index group ${group_from} before publishing group ${group_to}..."
        sleep 30
    fi
}

# Main script
main() {
    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --dry-run)
                DRY_RUN=true
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo ""
                show_help
                exit 1
                ;;
        esac
        shift
    done

    echo ""
    if [ "$DRY_RUN" = true ]; then
        print_dry_run "=== Crates.io Publish (Dry Run) ==="
    else
        print_info "=== Crates.io Publish ==="
    fi
    echo ""

    # Check prerequisites
    check_prerequisites
    echo ""

    # Define publish groups (topological order)
    # Format: "directory:crate-name"

    # Group 1: Core (no workspace dependencies)
    local group1=(
        "crates/iab-specs-core:iab-specs-core"
    )

    # Group 2: Depends on core only
    local group2=(
        "crates/iab-specs-adcom:iab-specs-adcom"
        "crates/iab-specs-ads_txt:iab-specs-ads_txt"
        "crates/iab-specs-sellers_json:iab-specs-sellers_json"
        "crates/iab-specs-artb:iab-specs-artb"
        "crates/iab-specs-agentic_audience:iab-specs-agentic_audience"
        "crates/iab-specs-agentic_direct:iab-specs-agentic_direct"
        "crates/iab-specs-openrtb_native:iab-specs-openrtb_native"
    )

    # Group 3: Depends on group 2 crates
    local group3=(
        "crates/iab-specs-openrtb:iab-specs-openrtb"
        "crates/iab-specs-app_ads_txt:iab-specs-app_ads_txt"
        "crates/iab-specs-buyer_agent:iab-specs-buyer_agent"
        "crates/iab-specs-seller_agent:iab-specs-seller_agent"
        "crates/iab-specs-registry_agent:iab-specs-registry_agent"
    )

    # Group 4: Umbrella crate (depends on everything)
    local group4=(
        ".:iab-specs"
    )

    # Publish groups in order
    publish_group 1 "$DRY_RUN" "${group1[@]}"
    echo ""
    wait_between_groups 1 2 "$DRY_RUN"
    echo ""

    publish_group 2 "$DRY_RUN" "${group2[@]}"
    echo ""
    wait_between_groups 2 3 "$DRY_RUN"
    echo ""

    publish_group 3 "$DRY_RUN" "${group3[@]}"
    echo ""
    wait_between_groups 3 4 "$DRY_RUN"
    echo ""

    publish_group 4 "$DRY_RUN" "${group4[@]}"
    echo ""

    if [ "$DRY_RUN" = true ]; then
        print_dry_run "=== All crates passed dry-run checks ==="
    else
        print_info "=== All crates published successfully to crates.io ==="
    fi
}

main "$@"
