# iab-specs-app_ads_txt

Rust implementation of IAB App-ads.txt 1.0 specification for the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Overview

Provides typed Rust data structures for parsing and generating App-ads.txt 1.0 files, including:

- **AppAdsTxt** — Root container for app-ads.txt entries with contact, subdomain, and inventory partner domain
- **AdsTxtSystem** — Individual authorized seller entries (re-exported from `iab-specs-ads_txt`)
- **SellerRelationType** — DIRECT or RESELLER relationship types (re-exported from `iab-specs-ads_txt`)

App-ads.txt v1.0 is based on an earlier ads.txt specification and does **not** support the ads.txt 1.1 features (`OWNERDOMAIN`, `MANAGERDOMAIN`).

## License

Apache-2.0
