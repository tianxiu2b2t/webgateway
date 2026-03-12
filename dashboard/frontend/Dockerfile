FROM nginx:alpine

RUN apk add --no-cache gettext

ADD dist /usr/share/nginx/html

ADD default.conf.template /etc/nginx/templates/
ADD entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

CMD ["nginx", "-g", "daemon off;"]