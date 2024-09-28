# template-rs-server

[![clippy](https://github.com/FL03/template-rs-server/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/template-rs-server/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/template-rs-server/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/template-rs-server/actions/workflows/rust.yml)

[![docker](https://github.com/FL03/template-rs-server/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/template-rs-server/actions/workflows/docker.yml)

***

_**The application is currently in the early stages of development and is not yet ready for production use.**_

A server optimized for WASM applications

## Features

- [x] Feature 1

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/template-rs-server.git
cd pzzld-server
```

#### _Building the project_

```bash
cargo build --all-features -r -v --workspace
```

#### _Running tests_

```bash
cargo test --all-features -r -v --workspace
```

## Usage

### Installation

```bash
cargo install template-rs-server
```

### Running the server

```bash
pzzld serve --port 8080
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
