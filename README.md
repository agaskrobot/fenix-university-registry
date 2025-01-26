# University Registry Smart Contract

This smart contract, written in Rust using the NEAR Protocol framework, provides functionality for managing a registry of universities. It enables adding, retrieving, and organizing universities by their account IDs and names.

## Features

- **Add University**: Allows the contract owner to register universities with a name and unique account ID.
- **Retrieve by Account ID**: Fetch a university's details using its account ID.
- **Retrieve by Name**: Get a list of universities that share the same name.
- **Retrieve All Universities**: Retrieve a list of all registered universities.

## Prerequisites

- Rust installed on your system. Follow the instructions [here](https://www.rust-lang.org/tools/install).
- NEAR CLI installed. Follow the instructions [here](https://docs.near.org/docs/tools/near-cli).
- NEAR testnet or mainnet account for deploying the contract.

## Smart Contract Structure

### Main Components

1. **Data Structures**
   - `University`: Represents a university with two fields:
     - `name`: The name of the university.
     - `account_id`: A unique account ID associated with the university.

   - `StorageKey`: Enum to manage keys used for persistent storage.

2. **Contract Implementation**
   - `UniversityRegistry`: Contains two main maps:
     - `universities_accounts`: Maps account IDs to `University` objects.
     - `universities_by_name`: Maps university names to a list of `University` objects.

### Key Methods

#### Public Methods

- `add_university(name: String, account_id: String) -> University`
  - Adds a new university to the registry.
  - Accessible only by the contract owner.
  - Panics if the account ID already exists.

- `get_all_universities() -> Vec<(String, University)>`
  - Returns a list of all universities in the registry.

- `get_universities_by_name(name: String) -> Vec<University>`
  - Retrieves universities that match the given name.

- `get_university_by_account_id(account_id: String) -> Option<University>`
  - Fetches a university by its account ID.

#### Internal Methods

- `add_university_by_name(university: University)`
  - Helper method to organize universities by name.

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
