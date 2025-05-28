# Redis Docker Configuration

A production-ready Redis Docker configuration with security best practices and optimized settings.

## Features

- Redis 7.2 on Alpine Linux for minimal image size
- Custom Redis configuration with optimized settings
- Password protection enabled
- Persistent storage with AOF (Append Only File)
- Memory management with LRU eviction policy
- Health checks and monitoring
- Resource limits and security hardening

## Prerequisites

- Docker and Docker Compose
- At least 2GB of available memory
- Port 6379 available (or configurable)

## Configuration

### Environment Variables

Create a `.env` file in the same directory with the following variables:

```env
REDIS_PASSWORD=your_secure_password_here
```

### Port Configuration

By default, Redis is only accessible from localhost (127.0.0.1) on port 6379. To modify this, edit the `ports` section in `docker-compose.yml`.

## Usage

```bash
# Start Redis
docker-compose up -d

# Check logs
docker-compose logs -f

# Stop Redis
docker-compose down
```

## Configuration Details

### Redis Settings

- **Memory Management**: 2GB max memory with LRU eviction
- **Persistence**: AOF enabled with every-second fsync
- **Security**: Password protection and protected mode enabled
- **Performance**: Optimized for high throughput and low latency

### Resource Limits

- CPU: 1 core limit, 0.5 core reservation
- Memory: 2GB limit, 1GB reservation
- File descriptors: 65536 limit

## Monitoring

The container includes:
- Health checks every 30 seconds
- Slow log monitoring
- Latency monitoring
- Memory usage tracking

## Data Persistence

Data is persisted in two ways:
1. AOF (Append Only File) for durability
2. Volume mount at `./data` for data persistence

## Security

- Non-root user execution
- Password protection
- Protected mode enabled
- No new privileges
- Localhost-only access by default

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](../../../.github/CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the AGPL-3.0 License and the VTubers.TV Commercial License (VCL) v1.0. See the [LICENSE](../../../LICENSE) and [LICENSE-VCL](../../../LICENSE-VCL.md) files for details. 