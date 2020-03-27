# ci_info

[![crates.io](https://img.shields.io/crates/v/ci_info.svg)](https://crates.io/crates/ci_info) [![Build Status](https://travis-ci.org/sagiegurari/ci_info.svg?branch=master)](http://travis-ci.org/sagiegurari/ci_info) [![Build status](https://ci.appveyor.com/api/projects/status/github/sagiegurari/ci_info?branch=master&svg=true)](https://ci.appveyor.com/project/sagiegurari/ci-info) [![codecov](https://codecov.io/gh/sagiegurari/ci_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/ci_info)<br>
[![license](https://img.shields.io/crates/l/ci_info.svg)](https://github.com/sagiegurari/ci_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/ci_info.svg)](https://libraries.io/cargo/ci_info) [![Documentation](https://docs.rs/ci_info/badge.svg)](https://docs.rs/crate/ci_info/) [![downloads](https://img.shields.io/crates/d/ci_info.svg)](https://crates.io/crates/ci_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Provides current CI environment information.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/ci_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current CI environment.<br>
The code is based on the [ci-info](https://github.com/watson/ci-info) npm module.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

### Fetching Info

```rust
fn main() {
    // Just check if a CI environment is detected.
    let ci = ci_info::is_ci();
    println!("Is CI: {}", ci);

    // Get CI environment information
    let info = ci_info::get();
    println!("Is CI: {}", info.ci);
    if info.vendor.is_some() {
        println!("Vendor: {:#?}", info.vendor.unwrap());
        println!("Name: {:#?}", info.name.unwrap());
    }
    if info.pr.is_some() {
        println!("Is PR: {:#?}", info.pr.unwrap());
    }
    if info.branch_name.is_some() {
        println!("Branch Name: {:#?}", info.branch_name.unwrap());
    }
}
```

### Mocking CI environment

```rust
use ci_info::types::{CiInfo, Vendor};

fn main() {
    // create the CI info manually
    let mut mock_info = CiInfo::new();
    mock_info.vendor = Some(Vendor::TravisCI);
    mock_info.ci = true;
    mock_info.pr = Some(true);
    mock_info.branch_name = Some("dev_branch".to_string());

    // mock environment
    ci_info::mock_ci(&mock_info);

    let info = ci_info::get();

    assert!(info.ci);
    assert!(info.pr.unwrap());
    assert_eq!(info.vendor.unwrap(), Vendor::TravisCI);
    assert_eq!(info.name.unwrap(), "Travis CI");
    assert_eq!(info.branch_name.unwrap(), "dev_branch");
}
```

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
ci_info = "*"
```

There is optional `serde` support that can be enabled via the `serde-1` feature:

```ini
[dependencies]
ci_info = { version = "*", features = ["serde-1"] }
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/ci_info/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
