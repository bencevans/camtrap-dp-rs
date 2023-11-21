# camtrap_dp

[![Documentation](https://img.shields.io/badge/docs-latest-brightgreen.svg?style=flat-square)](https://docs.rs/camtrap_dp/latest/camtrap_dp/)
[![Crate](https://img.shields.io/crates/v/camtrap_dp.svg?style=flat-square)](https://crates.io/crates/camtrap_dp)
[![License](https://img.shields.io/crates/l/camtrap_dp.svg?style=flat-square)](https://crates.io/crates/camtrap_dp)

## Description

`camtrap_dp` is a Rust crate designed for handling Camera Trap Data Packages as specified by the [Camera Trap Data Package](https://camtrap-dp.tdwg.org/data/) standard. It provides a standardized format for describing camera trap deployments and the media files recorded, based on [Data Package](https://frictionlessdata.io/specs/data-package/) and [Tabular Data Package](https://frictionlessdata.io/specs/tabular-data-package/) specifications.

This library streamlines both reading and writing camera trap data, including deployments, media files, and observations derived from these files.

## Features

- Structures for representing camera trap deployments, media files, and observations.
- CSV serialization and deserialization capabilities.
- Comprehensive handling of various camera trap data attributes.

## Installation

Add `camtrap_dp` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
camtrap_dp = "0.1.0"  # Replace with the latest version
```

## Usage

### Reading Data

Reading deployment and media data example:

```rust
use camtrap_dp::{Deployment, Medium, Observation};

// Reading deployment data from a CSV file
let deployments = Deployment::from_file("path/to/deployments.csv").unwrap();

// Reading media data from a URL
let media_data = Medium::from_url("http://example.com/media.csv").unwrap();

// Reading observations from a file
let observations = Observation::from_file("path/to/observations.csv").unwrap();
```

### Writing Data

Writing deployment data to a CSV file example:

```rust
// Assuming deployments is a Vec<Deployment> populated with data
deployments.to_file("path/to/output/deployments.csv").unwrap();
```

## Contributing

Contributions are welcome! Feel free to report issues, suggest features, or submit pull requests on our GitHub repository.

## License

camtrap_dp is licensed under the MIT License - see the LICENSE file for more information.

## Acknowledgments

Thanks to all contributors and the teams behind the Camera Trap Data Package standards.
