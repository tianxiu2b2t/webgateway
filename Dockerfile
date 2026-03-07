FROM debian:13.2-slim

# cp builds to the container
COPY builds/* /opt/webgateway/
COPY entrypoint.sh /opt/webgateway/

RUN chmod +x /opt/webgateway/entrypoint.sh \
    && ln -s /opt/webgateway/webgateway-mnt /usr/local/bin/wg-mnt 2>/dev/null || true