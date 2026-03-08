FROM debian:13.2-slim

# cp builds to the container
WORKDIR /opt/webgateway
ADD dashboard /opt/webgateway/dashboard
ADD webgateway-mnt /opt/webgateway/webgateway-mnt
# ln mnt to bin
RUN ln -s /opt/webgateway/webgateway-mnt /usr/bin/mnt

CMD ["/opt/webgateway/dashboard"]