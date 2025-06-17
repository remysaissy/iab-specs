# IAB specs - An unofficial Rust implementation of various IAB specifications

iab-specs is an unofficial implementation of IAB specifications in Rust.
This is not a parser.
Instead, it aims at providing the various IAB specifications using simple data structures around the serde 
and from_str/to_string patterns and encompass each specification logic inside of it.

This is a work in progress and I welcome any contribution to add new specification to the list or improve the existing
implementations.

## Quickstart

### Installation
You can install iab-specs by adding the iab-specs dependency to your Cargo file or by running the following command:

```shell
cargo add iab-specs
```

### Usage

### Roadmap

[X] Ads.txt 1.1 
[X] Sellers.json 1.0
[] OpenRTB 2.x
[] OpenRTB 3.x
[] ... (feel free to propose and add anything you need)


### Examples
Usage examples are documented in the test cases. Please refer to unit tests in the source code for concrete examples of how to use each feature.
All unit test suite includes both serialization and deserialization example.

## License
Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution
Unless you explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be  licensed as above, without any additional terms or conditions.
