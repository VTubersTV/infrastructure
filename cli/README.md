# Docker Configuration Manager CLI

A command-line tool for managing Docker configurations and environment variables across different projects.

## Features

- Interactive selection of Docker configurations
- Automatic environment variable detection and configuration
- Support for both local and remote Docker configurations
- Secure handling of sensitive environment variables
- Graceful cleanup on program termination

## Installation

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/VTubersTV/infrastructure.git

# Navigate to the CLI directory
cd infrastructure/cli

# Build the project
cargo build --release

# The binary will be available at target/release/docker-cli
```

## Usage

```bash
# Run the CLI
./target/release/docker-cli
```

The CLI will:
1. Look for Docker configurations in the local `docker` directory
2. If not found, clone the infrastructure repository
3. Present an interactive menu to select a Docker configuration
4. Guide you through setting up required environment variables
5. Create a `.env` file with your configuration

## Configuration

The CLI supports two modes of operation:

1. **Local Mode**: Uses Docker configurations from the local `docker` directory
2. **Remote Mode**: Clones and uses configurations from the infrastructure repository

## Environment Variables

The CLI automatically detects required environment variables from:
- `docker-compose.yml` files
- Existing `.env` files

It will prompt you to enter values for any undefined variables.

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](../../.github/CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the AGPL-3.0 License and the VTubers.TV Commercial License (VCL) v1.0. See the [LICENSE](../../LICENSE) and [LICENSE-VCL](../../LICENSE-VCL.md) files for details. 