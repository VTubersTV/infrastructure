FROM redis:7.2-alpine

# Create a non-root user
RUN addgroup -S redis && adduser -S redis -G redis

# Copy custom Redis configuration
COPY redis.conf /usr/local/etc/redis/redis.conf

# Set proper permissions
RUN chown -R redis:redis /usr/local/etc/redis/redis.conf

# Switch to non-root user
USER redis

# Expose Redis port
EXPOSE 6379

# Set healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD redis-cli ping || exit 1

# Start Redis with custom config
CMD ["redis-server", "/usr/local/etc/redis/redis.conf"]
