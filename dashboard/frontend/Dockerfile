FROM nginx:alpine

RUN apk add --no-cache gettext

COPY dist /usr/share/nginx/html
COPY default.conf.template /etc/nginx/templates/
COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

CMD ["nginx", "-g", "daemon off;"]