#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

print_success() {
    echo -e "${BLUE}[SUCCESS]${NC} $1"
}

# Function to display help
show_help() {
    cat << EOF
Usage: $0 [OPTION]

Run rustfmt to check or format code.

Options:
    --check         Check formatting without modifying files (default)
    --fix           Format code in place
    --help          Display this help message

Examples:
    $0                  # Check formatting
    $0 --check          # Check formatting
    $0 --fix            # Format code

Requirements:
  - rustfmt must be installed (rustup component add rustfmt)

EOF
}

# Check if rustfmt is installed
check_rustfmt() {
    if ! rustup component list | grep -q 'rustfmt.*installed'; then
        print_error "rustfmt is not installed."
        echo ""
        echo "Please install it with:"
        echo "  rustup component add rustfmt"
        echo ""
        exit 1
    fi
}

# Main script
main() {
    local check_mode=true

    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --check)
                check_mode=true
                shift
                ;;
            --fix)
                check_mode=false
                shift
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
    done

    # Check dependencies
    check_rustfmt

    # Run format check or fix
    if [ "$check_mode" = true ]; then
        print_info "Checking code formatting..."
        cargo fmt --all -- --check
        print_success "Format check complete!"
    else
        print_info "Formatting code..."
        cargo fmt --all
        print_success "Code formatted!"
    fi
}

main "$@"
