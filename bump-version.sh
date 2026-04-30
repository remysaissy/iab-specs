#!/usr/bin/env bash

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

show_help() {
    cat << 'EOF'
Usage: ./bump-version.sh COMMAND [OPTIONS]

Manage project version following the X.Y.Z-dev → X.Y.Z release cycle.

COMMANDS

  --post-release
        Add -dev suffix to the current version immediately after a release.
        Does NOT bump the version number — the suffix signals "unreleased work
        on top of X.Y.Z". No CHANGELOG change.

        Example: 0.5.1  →  0.5.1-dev

  --release [--minor | --major | --revision]
        Prepare a release PR. Strips -dev, derives the next version from
        conventional commits via git-cliff --bump, and finalises CHANGELOG.
        Pass an explicit bump flag to override the auto-derived type.
        Creates a release branch, commits (signed), pushes, and opens a PR.

        Examples:
          ./bump-version.sh --release              # auto-derived from commits
          ./bump-version.sh --release --minor      # force minor bump
          ./bump-version.sh --release --major      # force major bump
          ./bump-version.sh --release --revision   # force patch bump

        Input → output (auto-derived):
          0.5.1-dev + only fix: commits  →  0.5.2
          0.5.1-dev + feat: commit       →  0.6.0
          0.5.1-dev + feat!: commit      →  1.0.0

  --revision
        Mid-cycle patch re-plan. Keeps -dev suffix.
        Example: 0.5.1-dev  →  0.5.2-dev

  --minor
        Mid-cycle minor re-plan. Keeps -dev suffix.
        Example: 0.5.1-dev  →  0.6.0-dev

  --major
        Mid-cycle major re-plan. Keeps -dev suffix.
        Example: 0.5.1-dev  →  1.0.0-dev

  --dry-run
        Show what would happen without modifying any files.
        Applies to --release, --post-release, and mid-cycle bumps.

  --help
        Show this message.

TYPICAL LIFECYCLE

  # Immediately after releasing v0.5.1:
  ./bump-version.sh --post-release
  git commit -m 'chore: begin development on next release'
  git push

  # (All feature work lands on main; version stays 0.5.1-dev throughout)

  # Ready to ship:
  ./bump-version.sh --release           # or: --release --minor to override
  # → Creates release/vX.Y.Z branch, commits, pushes, and opens a PR

  # After the PR is merged:
  ./release.sh

REQUIREMENTS
  git-cliff must be installed: cargo install git-cliff
  gh CLI must be installed and authenticated: https://cli.github.com/
EOF
}

print_info()    { echo -e "${GREEN}[INFO]${NC} $1"; }
print_error()   { echo -e "${RED}[ERROR]${NC} $1" >&2; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_dry_run() { echo -e "${YELLOW}[DRY-RUN]${NC} $1"; }

check_dependencies() {
    if ! command -v git-cliff &>/dev/null; then
        print_error "git-cliff is not installed."
        echo "Install with: cargo install git-cliff"
        exit 1
    fi
}

check_gh() {
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

# Get raw version string (may include -dev) from [workspace.package] in Cargo.toml
get_raw_version() {
    grep -A5 '^\[workspace\.package\]' Cargo.toml | grep 'version' | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

# Get base version without -dev suffix
get_base_version() {
    get_raw_version | sed 's/-dev$//'
}

parse_version() {
    echo "$1" | sed 's/\./ /g'
}

apply_bump() {
    local base="$1" type="$2"
    read -r major minor patch <<< "$(parse_version "$base")"
    case "$type" in
        major)    echo "$((major + 1)).0.0" ;;
        minor)    echo "$major.$((minor + 1)).0" ;;
        revision) echo "$major.$minor.$((patch + 1))" ;;
        *) print_error "Unknown bump type: $type"; exit 1 ;;
    esac
}

# Get the cargo target directory from cargo metadata
get_target_dir() {
    cargo metadata --format-version 1 --no-deps 2>/dev/null | grep -o '"target_directory":"[^"]*"' | sed 's/"target_directory":"//;s/"$//'
}

# Update version in [workspace.package] section of Cargo.toml
update_workspace_version() {
    local new_version=$1
    if [[ "$OSTYPE" == darwin* ]]; then
        sed -i '' '/^\[workspace\.package\]/,/^\[/ s/^version = ".*"/version = "'"$new_version"'"/' Cargo.toml
    else
        sed -i '/^\[workspace\.package\]/,/^\[/ s/^version = ".*"/version = "'"$new_version"'"/' Cargo.toml
    fi
    print_info "Cargo.toml [workspace.package] → version = \"$new_version\""
}

# Update inter-crate dependency versions across all workspace Cargo.toml files
update_inter_crate_versions() {
    local old_version=$1
    local new_version=$2
    local target_dir
    target_dir=$(get_target_dir)

    print_info "Updating inter-crate dependency versions: $old_version → $new_version"

    while IFS= read -r cargo_file; do
        if [[ "$OSTYPE" == darwin* ]]; then
            sed -i '' '/iab-specs-/ s/version = "'"$old_version"'"/version = "'"$new_version"'"/' "$cargo_file"
        else
            sed -i '/iab-specs-/ s/version = "'"$old_version"'"/version = "'"$new_version"'"/' "$cargo_file"
        fi
    done < <(find . -name "Cargo.toml" -not -path "${target_dir}/*")
}

update_lockfile() {
    print_info "Updating Cargo.lock..."
    if cargo generate-lockfile; then
        print_info "Cargo.lock updated successfully"
    else
        print_error "Failed to update Cargo.lock"
        exit 1
    fi
}

update_changelog() {
    local tag="$1"
    print_info "Generating CHANGELOG.md for $tag…"
    if git-cliff --unreleased --tag "$tag" -o CHANGELOG.md; then
        print_info "CHANGELOG.md updated."
    else
        print_error "Failed to generate CHANGELOG.md"
        exit 1
    fi
}

stage_cargo_files() {
    local target_dir
    target_dir=$(get_target_dir)
    find . -name "Cargo.toml" -not -path "${target_dir}/*" -exec git add {} +
    git add -f Cargo.lock
}

confirm() {
    read -rp "$1 (y/N) " -n 1
    echo
    [[ "$REPLY" =~ ^[Yy]$ ]]
}

# ─────────────────────────────────────────────────────────────────────────────
# Command implementations
# ─────────────────────────────────────────────────────────────────────────────

cmd_post_release() {
    local dry_run="${1:-false}"
    check_clean_workdir

    local raw; raw=$(get_raw_version)

    if [[ "$raw" == *-dev ]]; then
        print_warning "Version is already a dev version: $raw"
        exit 0
    fi

    local new_ver="${raw}-dev"

    if [ "$dry_run" = true ]; then
        echo ""
        print_dry_run "Post-release plan:"
        print_dry_run "  Current version: $raw"
        print_dry_run "  New version:     $new_ver"
        print_dry_run ""
        print_dry_run "Actions that would be performed:"
        print_dry_run "  1. Update Cargo.toml [workspace.package] version to $new_ver"
        print_dry_run "  2. Update inter-crate dependency versions to $new_ver"
        print_dry_run "  3. Run cargo generate-lockfile"
        print_dry_run "  4. Stage all Cargo.toml files and Cargo.lock"
        print_dry_run ""
        print_dry_run "No files were modified."
        return
    fi

    confirm "Add -dev suffix: $raw → $new_ver?" \
        || { print_warning "Cancelled."; exit 0; }

    update_workspace_version "$new_ver"
    update_inter_crate_versions "$raw" "$new_ver"
    update_lockfile
    stage_cargo_files

    echo ""
    print_info "Done. Next step:"
    print_info "  git commit -m 'chore: begin development on next release'"
    print_info "  git push"
}

cmd_dev_bump() {
    local type="$1"
    local dry_run="${2:-false}"
    check_clean_workdir

    local raw; raw=$(get_raw_version)
    local base; base=$(get_base_version)
    local new_ver; new_ver="$(apply_bump "$base" "$type")-dev"

    if [ "$dry_run" = true ]; then
        echo ""
        print_dry_run "Mid-cycle re-plan:"
        print_dry_run "  Bump type:    $type"
        print_dry_run "  Current:      $raw"
        print_dry_run "  New:          $new_ver"
        print_dry_run ""
        print_dry_run "Actions that would be performed:"
        print_dry_run "  1. Update Cargo.toml [workspace.package] version to $new_ver"
        print_dry_run "  2. Update inter-crate dependency versions to $new_ver"
        print_dry_run "  3. Run cargo generate-lockfile"
        print_dry_run "  4. Stage all Cargo.toml files and Cargo.lock"
        print_dry_run ""
        print_dry_run "No files were modified."
        return
    fi

    confirm "Mid-cycle re-plan: $raw → $new_ver?" \
        || { print_warning "Cancelled."; exit 0; }

    update_workspace_version "$new_ver"
    update_inter_crate_versions "$raw" "$new_ver"
    update_lockfile
    stage_cargo_files

    echo ""
    print_info "Done. Next step:"
    print_info "  git commit -m 'chore: re-plan next release as $new_ver'"
    print_info "  git push"
}

cmd_release() {
    local bump_override="$1"
    local dry_run="${2:-false}"

    check_dependencies
    check_gh
    check_clean_workdir

    local raw; raw=$(get_raw_version)
    local base; base=$(get_base_version)

    if [[ "$raw" != *-dev ]]; then
        print_warning "Current version '$raw' has no -dev suffix."
        confirm "Proceed anyway?" || { print_warning "Cancelled."; exit 0; }
    fi

    local new_ver
    if [[ -n "$bump_override" ]]; then
        new_ver=$(apply_bump "$base" "$bump_override")
        print_info "Using explicit bump ($bump_override): $base → $new_ver"
    else
        local cliff_out
        cliff_out=$(git-cliff --bumped-version 2>/dev/null || true)
        new_ver="${cliff_out#v}"            # strip leading 'v' if present
        new_ver="${new_ver//[[:space:]]/}"  # trim whitespace

        if [[ -z "$new_ver" ]]; then
            print_error "git-cliff --bumped-version returned nothing."
            print_error "Specify the bump explicitly: --release --minor | --major | --revision"
            exit 1
        fi
        print_info "git-cliff derived next version: $new_ver"
    fi

    local release_branch="release/v${new_ver}"

    if [ "$dry_run" = true ]; then
        echo ""
        print_dry_run "Release preparation plan:"
        print_dry_run "  Current version: $raw"
        print_dry_run "  New version:     $new_ver"
        print_dry_run ""
        print_dry_run "Actions that would be performed:"
        print_dry_run "  1. Create branch ${release_branch} from origin/main"
        print_dry_run "  2. Update Cargo.toml [workspace.package] version to $new_ver"
        print_dry_run "  3. Update inter-crate dependency versions to $new_ver"
        print_dry_run "  4. Run cargo generate-lockfile"
        print_dry_run "  5. Generate CHANGELOG.md for v$new_ver using git-cliff"
        print_dry_run "  6. Commit (signed) and push ${release_branch}"
        print_dry_run "  7. Create PR to main"
        print_dry_run ""
        print_dry_run "No files were modified."
        return
    fi

    confirm "Release: $raw → $new_ver (will create PR for v$new_ver)?" \
        || { print_warning "Cancelled."; exit 0; }

    print_info "Fetching latest main from origin..."
    git fetch origin main

    if git show-ref --verify --quiet "refs/heads/${release_branch}"; then
        print_error "Branch ${release_branch} already exists. Delete it first or choose a different version."
        exit 1
    fi

    git checkout -b "$release_branch" origin/main
    print_info "Created branch ${release_branch}"

    update_workspace_version "$new_ver"
    update_inter_crate_versions "$raw" "$new_ver"
    update_lockfile
    update_changelog "v$new_ver"

    stage_cargo_files
    git add -f CHANGELOG.md

    git commit -S -m "chore(release): prepare for v$new_ver"
    print_info "Changes committed (signed)"

    print_info "Pushing ${release_branch}..."
    git push -u origin "$release_branch"

    print_info "Creating pull request..."
    local pr_url
    pr_url=$(gh pr create \
        --base main \
        --head "$release_branch" \
        --title "chore(release): prepare for v$new_ver" \
        --body "Bump version from ${raw} to ${new_ver}.

Changes:
- Updated workspace version in Cargo.toml
- Updated inter-crate dependency versions
- Updated Cargo.lock
- Generated CHANGELOG.md with git-cliff

After this PR is merged, run:
\`\`\`
./release.sh
\`\`\`")

    echo ""
    print_info "Pull request created: ${pr_url}"
    echo ""
    print_info "After the PR is merged, run:"
    print_info "  ./release.sh"
}

# ─────────────────────────────────────────────────────────────────────────────
# Argument parsing
# ─────────────────────────────────────────────────────────────────────────────

main() {
    if [[ $# -eq 0 ]]; then
        show_help
        exit 0
    fi

    local dry_run=false

    # Pre-scan for --dry-run so it can appear in any position
    for arg in "$@"; do
        if [[ "$arg" == "--dry-run" ]]; then
            dry_run=true
        fi
    done

    case "$1" in
        --post-release)
            cmd_post_release "$dry_run"
            ;;
        --release)
            local bump_override=""
            shift
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --revision) bump_override="revision" ;;
                    --minor)    bump_override="minor"    ;;
                    --major)    bump_override="major"    ;;
                    --dry-run)  ;;  # already handled above
                    *)
                        print_error "Unknown option for --release: $1"
                        echo ""
                        show_help
                        exit 1
                        ;;
                esac
                shift
            done
            cmd_release "$bump_override" "$dry_run"
            ;;
        --revision) cmd_dev_bump "revision" "$dry_run" ;;
        --minor)    cmd_dev_bump "minor"    "$dry_run" ;;
        --major)    cmd_dev_bump "major"    "$dry_run" ;;
        --dry-run)
            print_error "--dry-run must be used alongside another command."
            echo ""
            show_help
            exit 1
            ;;
        --help|-h) show_help ;;
        *)
            print_error "Unknown option: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"
