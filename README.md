# bread

> CLI byte stream conversion tool in various formats

## Table of Contents

- [About](#about)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build from sources](#build-from-sources)
- [Usage](#usage)
- [License](#license)
- [Acknowledgements](#acknowledgements)

---

## About

bread is a cross-platform command-line tool useful for converting from and to a stream of:
- raw bytes
- ascii characters
- binary, hexadecimal or any other numeric base representation

See 
```
bread --help
```
for more info about supported formats.

> **[?]**
> Please provide your screenshots here.

|                               Home Page                               |                               Login Page                               |
| :-------------------------------------------------------------------: | :--------------------------------------------------------------------: |
| <img src="docs/images/screenshot.png" title="Home Page" width="100%"> | <img src="docs/images/screenshot.png" title="Login Page" width="100%"> |


## Getting Started

### Prerequisites

bread is cross-platofrm, coded in Rust; you need to have a valid [Rust](https://rustup.rs/) installation.
Nightly version would be required for running benchmarks.

### Build from sources
1. clone this repository
2. build with cargo:
```
cargo build --release
```
binary will be in ```target/release/bread```


## Usage

> **[?]**
> How does one go about using it?
> Provide various use cases and code examples here.

## License

This project is licensed under the **GNU General Public License v3**.

See [LICENSE](LICENSE) for more information.

## Acknowledgements

1. [clap](https://github.com/clap-rs/clap)
2. [amazing-github-template](https://github.com/dec0dOS/amazing-github-template)
