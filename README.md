# ViaCEP Rust client

🇧🇷📍 [ViaCEP](https://viacep.com.br) client wrapper for Rust projects.

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/guibranco/viacep-sdk-rs/main)
[![wakatime](https://wakatime.com/badge/github/guibranco/viacep-sdk-rs.svg)](https://wakatime.com/badge/github/guibranco/viacep-sdk-rs)

[![Maintainability](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/maintainability)](https://codeclimate.com/github/guibranco/viacep-sdk-rs/maintainability)
[![Test Coverage](https://api.codeclimate.com/v1/badges/392b044637f43eb881ac/test_coverage)](https://codeclimate.com/github/guibranco/viacep-sdk-rs/test_coverage)
[![CodeFactor](https://www.codefactor.io/repository/github/guibranco/viacep-sdk-rs/badge)](https://www.codefactor.io/repository/github/guibranco/viacep-sdk-rs)

| Service   | Status |
| --------- | :----: |
| crates.io | [![Crates.io](https://img.shields.io/crates/v/viacep.svg)](https://crates.io/crates/viacep) |

Pure Rust bindings to the [ViaCEP API](https://viacep.com.br).

## Dependencies and support

`viacep` is intended to work on all tier 1 supported Rust systems:

- macOS
- Linux
- Windows

## Minimum Compiler Version

`viacep` requires `rustc` **1.56** or higher (edition 2021).

## Getting Started

Add the following to your `Cargo.toml`:

```toml
[dependencies]
viacep = "1.0.0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Then in your `lib.rs` or `main.rs` file:

```rust
use viacep::ViaCepClient;

let client = ViaCepClient::new();

// Single Zip Code lookup
match client.get_zipcode("03177010") {
    Err(e) => eprintln!("{:?}", e),
    Ok(data) => {
        let cep = data.unwrap();
        println!(
            "IBGE: {} | Address: {} | Neighborhood: {} | City: {} | UF: {}",
            cep.ibge, cep.address, cep.neighborhood, cep.city, cep.state_initials
        );
    }
}

// Search by address
match client.search("SP", "São Paulo", "Paulista") {
    Err(e) => eprintln!("{:?}", e),
    Ok(data) => {
        let addresses = data.unwrap();
        for address in addresses {
            println!(
                "IBGE: {} | Address: {} | City: {} | Zip: {}",
                address.ibge, address.address, address.city, address.zip
            );
        }
    }
}
```

## License

Licensed under the MIT license ([LICENSE](https://github.com/guibranco/viacep-sdk-rs/blob/main/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)).
