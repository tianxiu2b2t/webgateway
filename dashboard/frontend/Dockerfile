FROM node:22

# cp builds to the container
WORKDIR /opt/webgateway
ADD . .

RUN apt-get update && apt-get install -y yarnpkg \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    && yarnpkg install && yarnpkg build

CMD ["yarnpkg", "vite", "preview"]
EXPOSE 5173