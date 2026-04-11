# iab-specs-openrtb

Rust implementation of IAB OpenRTB 2.5, 2.6, and 3.0 specifications for real-time bidding in digital advertising.

Part of the [iab-specs](https://crates.io/crates/iab-specs) ecosystem.

## Features

- `openrtb_25` — OpenRTB 2.5 support
- `openrtb_26` — OpenRTB 2.6 support (includes 2.5, adds CTV/DOOH)
- `openrtb_30` — OpenRTB 3.0 support (layered architecture)

## Usage

```toml
[dependencies]
iab-specs-openrtb = { version = "0.4", features = ["openrtb_26"] }
```

## License

Apache-2.0
