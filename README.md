# vetis-dns

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-dns)](https://crates.io/crates/vetis-dns) [![crates.io](https://img.shields.io/crates/v/vetis-dns?style=flat-square)](https://crates.io/crates/vetis-dns) [![Build Status](https://github.com/vetis-server/vetis-dns/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-dns/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-dns) [![Documentation](https://docs.rs/vetis-dns/badge.svg)](https://docs.rs/vetis-dns/latest/vetis-dns) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-dns/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-dns/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-dns)

vetis-dns adds diffent rewrite types for your host path, choose the most appropriate one for your usecase!

## Features

- basic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-dns = "0.1.0"
```

## Configuration

Below you can find instruction on how to configure many auth options availble
on vetis.

### stdout

#### Using API

```rust,ignore
use vetis_log::StdoutLogConfig;
use log::Level;

StdoutLogConfig::builder()
  .log_leve(Level::Info)
  .build()?;
```

#### vetis.yaml

```yaml
dest: stdout
log_level: "INFO"
```

#### Configuration Options

- **log_level**: Log level: TRACE, WARN, DEBUG, INFO, ERROR and FATAL

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
