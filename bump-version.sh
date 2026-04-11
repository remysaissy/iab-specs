#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Flags
DRY_RUN=false

# Function to display help
show_help() {
    cat << EOF
Usage: $0 [OPTION]...

Bump the project version and update the CHANGELOG using git-cliff.

Options:
    --revision    Bump the revision/patch version (0.0.X)
    --minor       Bump the minor version (0.X.0)
    --major       Bump the major version (X.0.0)
    --dry-run     Show what would happen without modifying any files
    --help        Display this help message

Examples:
    $0 --revision              # 0.0.9 -> 0.0.10
    $0 --minor                 # 0.0.9 -> 0.1.0
    $0 --major                 # 0.0.9 -> 1.0.0
    $0 --revision --dry-run    # Show version plan without modifying files

Note: This script will:
  1. Update the version in Cargo.toml ([workspace.package])
  2. Update inter-crate dependency versions in all workspace Cargo.toml files
  3. Run cargo generate-lockfile to update Cargo.lock
  4. Generate/update CHANGELOG.md using git-cliff
  5. Optionally commit, tag, and push

Requirements:
  - git-cliff must be installed (cargo install git-cliff)

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

# Check if git-cliff is installed
check_dependencies() {
    if ! command -v git-cliff &> /dev/null; then
        print_error "git-cliff is not installed."
        echo "Please install it with: cargo install git-cliff"
        exit 1
    fi
}

# Get current version from [workspace.package] section in Cargo.toml
get_current_version() {
    grep -A5 '^\[workspace\.package\]' Cargo.toml | grep 'version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# Parse version components
parse_version() {
    local version=$1
    echo "$version" | sed 's/\./ /g'
}

# Bump version based on type
bump_version() {
    local current_version=$1
    local bump_type=$2

    read -r major minor patch <<< "$(parse_version "$current_version")"

    case $bump_type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        revision)
            patch=$((patch + 1))
            ;;
        *)
            print_error "Unknown bump type: $bump_type"
            exit 1
            ;;
    esac

    echo "$major.$minor.$patch"
}

# Update version in [workspace.package] section of Cargo.toml
update_cargo_version() {
    local new_version=$1
    local cargo_file="Cargo.toml"

    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' '/^\[workspace\.package\]/,/^\[/ s/^version = ".*"/version = "'"$new_version"'"/' "$cargo_file"
    else
        # Linux
        sed -i '/^\[workspace\.package\]/,/^\[/ s/^version = ".*"/version = "'"$new_version"'"/' "$cargo_file"
    fi

    print_info "Updated $cargo_file [workspace.package] to version $new_version"
}

# Get the cargo target directory from cargo metadata
get_target_dir() {
    cargo metadata --format-version 1 --no-deps 2>/dev/null | grep -o '"target_directory":"[^"]*"' | sed 's/"target_directory":"//;s/"$//'
}

# Update inter-crate dependency versions in all workspace Cargo.toml files
update_inter_crate_versions() {
    local old_version=$1
    local new_version=$2
    local target_dir
    target_dir=$(get_target_dir)

    print_info "Updating inter-crate dependency versions..."

    # Find all Cargo.toml files in the workspace (excluding the actual target directory)
    while IFS= read -r cargo_file; do
        if [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS
            sed -i '' '/iab-specs-/ s/version = "'"$old_version"'"/version = "'"$new_version"'"/' "$cargo_file"
        else
            # Linux
            sed -i '/iab-specs-/ s/version = "'"$old_version"'"/version = "'"$new_version"'"/' "$cargo_file"
        fi
    done < <(find . -name "Cargo.toml" -not -path "${target_dir}/*")

    print_info "Updated inter-crate dependency versions to $new_version"
}

# Update Cargo.lock by running cargo generate-lockfile
update_lockfile() {
    print_info "Updating Cargo.lock..."

    if cargo generate-lockfile; then
        print_info "Cargo.lock updated successfully"
    else
        print_error "Failed to update Cargo.lock"
        exit 1
    fi
}

# Generate changelog using git-cliff
update_changelog() {
    local new_version=$1

    print_info "Generating CHANGELOG.md using git-cliff..."

    if git-cliff --unreleased --tag "v$new_version" -o CHANGELOG.md; then
        print_info "CHANGELOG.md updated successfully"
    else
        print_error "Failed to generate CHANGELOG.md"
        exit 1
    fi
}

# Main script
main() {
    # Check if no arguments provided
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    # Parse arguments
    bump_type=""

    while [ $# -gt 0 ]; do
        case "$1" in
            --revision)
                bump_type="revision"
                ;;
            --minor)
                bump_type="minor"
                ;;
            --major)
                bump_type="major"
                ;;
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

    if [ -z "$bump_type" ]; then
        print_error "No bump type specified. Use --revision, --minor, or --major."
        echo ""
        show_help
        exit 1
    fi

    # Check dependencies
    check_dependencies

    # Get current version
    current_version=$(get_current_version)
    print_info "Current version: $current_version"

    # Calculate new version
    new_version=$(bump_version "$current_version" "$bump_type")
    print_info "New version: $new_version"

    # Dry run mode: show plan and exit
    if [ "$DRY_RUN" = true ]; then
        echo ""
        print_dry_run "Version bump plan:"
        print_dry_run "  Bump type:    $bump_type"
        print_dry_run "  Current:      $current_version"
        print_dry_run "  New:          $new_version"
        print_dry_run ""
        print_dry_run "Actions that would be performed:"
        print_dry_run "  1. Update Cargo.toml [workspace.package] version to $new_version"
        print_dry_run "  2. Update inter-crate dependency versions in all Cargo.toml files"
        print_dry_run "  3. Run cargo generate-lockfile to update Cargo.lock"
        print_dry_run "  4. Generate CHANGELOG.md using git-cliff with tag v$new_version"
        print_dry_run "  5. Prompt to commit changes"
        print_dry_run "  6. Prompt to create tag v$new_version"
        print_dry_run "  7. Prompt to push changes and tags"
        echo ""
        print_dry_run "No files were modified."
        exit 0
    fi

    # Confirm with user
    read -p "Do you want to proceed with version bump from $current_version to $new_version? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "Version bump cancelled"
        exit 0
    fi

    # Update Cargo.toml
    update_cargo_version "$new_version"

    # Update inter-crate dependency versions
    update_inter_crate_versions "$current_version" "$new_version"

    # Update Cargo.lock
    update_lockfile

    # Update CHANGELOG.md
    update_changelog "$new_version"

    # Stage files
    print_info "Staging updated files..."
    local target_dir
    target_dir=$(get_target_dir)
    find . -name "Cargo.toml" -not -path "${target_dir}/*" -exec git add {} +
    git add -f Cargo.lock CHANGELOG.md

    echo ""
    print_info "Version bump complete!"
    echo ""

    # Prompt to commit
    read -p "Do you want to commit? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git commit -m "chore(release): prepare for v$new_version"
        print_info "Changes committed"

        # Prompt to create tag
        read -p "Do you want to create tag v$new_version? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git tag -a "v$new_version" -m "Release v$new_version"
            print_info "Tag v$new_version created"

            # Prompt to push
            read -p "Do you want to push changes and tags? (y/N) " -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                git push && git push --tags
                print_info "Changes and tags pushed"
            else
                print_warning "Push skipped. Run manually:"
                print_info "  git push && git push --tags"
            fi
        else
            print_warning "Tag creation skipped. Run manually:"
            print_info "  git tag -a v$new_version -m 'Release v$new_version'"
        fi
    else
        print_warning "Commit skipped. Run manually:"
        print_info "  1. Review the changes: git diff --cached"
        print_info "  2. Commit the changes: git commit -m 'chore(release): prepare for v$new_version'"
        print_info "  3. Create a git tag: git tag -a v$new_version -m 'Release v$new_version'"
        print_info "  4. Push changes: git push && git push --tags"
    fi
}

main "$@"
