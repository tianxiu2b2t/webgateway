FROM node:22

# cp builds to the container
COPY ../builds/dashboard /opt/webgateway/

RUN apt-get update && apt-get install -y yarnpkg && cd /opt/webgateway && yarnpkg install . 

WORKDIR /opt/webgateway

CMD ["yarnpkg vite preview"]
EXPOSE 5173