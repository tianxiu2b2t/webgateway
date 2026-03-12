FROM nginx:alpine

WORKDIR /opt/webgateway
ADD run/* /opt/webgateway

RUN apk add --no-cache gettext && chmod +x entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]

CMD ["nginx", "-g", "daemon off;"]