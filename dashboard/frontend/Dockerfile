FROM nginx:alpine

WORKDIR /opt/webgateway
ADD run/* /opt/webgateway

RUN apk add --no-cache gettext && chmod +x entrypoint.sh

ENTRYPOINT ["./opt/webgateway/entrypoint.sh"]

CMD ["nginx", "-g", "daemon off;"]