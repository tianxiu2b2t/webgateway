FROM debian:13.2-slim

# cp builds to the container
WORKDIR /opt/webgateway
ADD run/ /opt/webgateway/
# ln mnt to bin
RUN ln -s /opt/webgateway/webgateway-mnt /usr/bin/mnt

CMD ["/opt/webgateway/dashboard"]