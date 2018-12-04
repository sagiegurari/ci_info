# ci_info

[![crates.io](https://img.shields.io/crates/v/ci_info.svg)](https://crates.io/crates/ci_info) [![Build Status](https://travis-ci.org/sagiegurari/ci_info.svg)](http://travis-ci.org/sagiegurari/ci_info) [![Build status](https://ci.appveyor.com/api/projects/status/yrb4y9cbaf6wtlk7?svg=true)](https://ci.appveyor.com/project/sagiegurari/ci-info) [![codecov](https://codecov.io/gh/sagiegurari/ci_info/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/ci_info)<br>
[![license](https://img.shields.io/crates/l/ci_info.svg)](https://github.com/sagiegurari/ci_info/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/ci_info.svg)](https://libraries.io/cargo/ci_info) [![Documentation](https://docs.rs/ci_info/badge.svg)](https://docs.rs/crate/ci_info/) [![downloads](https://img.shields.io/crates/d/ci_info.svg)](https://crates.io/crates/ci_info)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Provides current CI environment information.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/ci_info/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
This library main goal is to provide development/build tools such as [cargo-make](https://sagiegurari.github.io/cargo-make/) the needed information on the current CI environment.<br>
The code is based on the [ci-info](https://github.com/watson/ci-info) npm module.

<a name="usage"></a>
## Usage
Simply include the library and invoke the get function to pull all info as follows:

````rust
extern crate ci_info;

fn main() {
    // Just check if a CI environment is detected.
    let ci = ci_info::is_ci();
    println!("Is CI: {}", ci);

    // Get CI environment information
    let info = ci_info::get();
    println!("Is CI: {}", info.ci);
    if info.ci {
        println!("Vendor: {:#?}", info.vendor.unwrap());
    }
}
````

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

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2018-11-17  | v0.2.1  | Maintenance |
| 2017-10-10  | v0.1.0  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
