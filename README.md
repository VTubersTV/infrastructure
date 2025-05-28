# VTubers.TV Infrastructure

This repository contains the infrastructure and deployment configurations for VTubers.TV services.

## Overview

This repository provides:
- Docker configurations for various services
- A CLI tool for managing Docker configurations
- Infrastructure-as-code templates
- Deployment scripts and utilities

## Quick Start

### Using the CLI

The easiest way to get started is using our CLI tool:

```bash
# Clone the repository
git clone https://github.com/VTubersTV/infrastructure.git

# Navigate to the CLI directory
cd infrastructure/cli

# Build and run the CLI
cargo build --release
./target/release/docker-cli
```

### Manual Setup

For manual setup of individual services:

1. Navigate to the service directory in `docker/`
2. Copy the example environment file if provided
3. Configure the environment variables
4. Run with Docker Compose

Example for Redis:
```bash
cd docker/redis
cp .env.example .env  # Configure your environment variables
docker-compose up -d
```

## CLI Tool

The Docker Configuration Manager CLI provides an interactive way to:
- Select and configure Docker services
- Manage environment variables
- Deploy services with proper configuration

[View CLI Documentation](cli/README.md)

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/YourFeature`)
3. Make your changes
4. Commit your changes (`git commit -m "feat(feature-name): add new feature"`)
5. Push to the branch (`git push origin feature/YourFeature`)
6. Create a new Pull Request

Please ensure your pull request adheres to our [Code of Conduct](.github/CODE_OF_CONDUCT.md).

## Development

### Prerequisites

- Docker and Docker Compose
- Rust (for CLI development)
- Git

### Building the CLI

```bash
cd cli
cargo build
```


## License

This project is licensed under the AGPL-3.0 License and the VTubers.TV Commercial License (VCL) v1.0. See the [LICENSE](LICENSE) and [LICENSE-VCL](LICENSE-VCL.md) files for details.

## Security

Please report any security issues to our security team. See [SECURITY.md](.github/SECURITY.md) for details.

## Support

For support, please:
1. Check the documentation
2. Search existing issues
3. Create a new issue if needed

## Acknowledgments

- Thanks to all contributors
- Special thanks to the open-source community


