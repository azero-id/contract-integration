# AZERO.ID - Contract level integration

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

This repository contains Rust crate to resolve domains & addresses registered with [AZERO.ID](https://azero.id).

## Integration Guide 📃

1. Include this crate in the contract's `Cargo.toml` file.

```rust
// file: ./Cargo.toml
[dependencies]
azns-integration = { git = "https://github.com/azero-id/contract-integration", default-features = false }

[features]
std = [
    "azns-integration/std",
    ..
]
```

2. Store the router address and import the type `AccountIdOrDomain`

```rust
// file: ./lib.rs
#[ink::contract]
mod example {
    // 1. Import `AccountIdOrDomain` type
    use azns_integration::AccountIdOrDomain;

    #[ink(storage)]
    pub struct Example {
        // 2. Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    impl Example {
        #[ink(constructor)]
        pub fn new(domain_router: AccountId) -> Self {
            Self { domain_router }
        }

        /// Returns the AccountId associated with the `user`
        #[ink(message)]
        pub fn simple_integration(&self, user: AccountIdOrDomain) -> Option<AccountId> {
            user.get_address(self.domain_router) // Resolves user to AccountId
        }
    }
}
```

Refer to the sample [example](./example/) for more details.

## Documentation 👩‍💻

View the full documentation & types here:

**https://docs.azero.id/integration/contract-level**

## Contract deployments link

**https://docs.azero.id/developers/deployments**

## APIs exposed by `AznsRouterRef`

* get_all_registries() -> Vec<AccountId>
* get_registry(tld: String) -> Option<AccountId>
* get_address(domain: String) -> Result<AccountId, u8>
* get_primary_domains(account: AccountId, tld: Option<String>) -> Vec<(AccountId, String)>

## Type - `AccountIdOrDomain`

Allows user to pass their `AccountId` or their AZERO.ID `Domain`

```rust
pub enum AccountIdOrDomain {
    AccountId(AccountId),
    Domain(String),
}
```
