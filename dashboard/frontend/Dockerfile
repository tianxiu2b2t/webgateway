FROM node:22

# cp builds to the container
WORKDIR /opt/webgateway
ADD . .

RUN apt-get update && apt-get install -y yarnpkg && cd /opt/webgateway && yarnpkg install . 

CMD ["yarnpkg vite preview"]
EXPOSE 5173