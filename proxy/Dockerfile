FROM nginx
COPY ./conf.d /etc/nginx/conf.d
COPY ./certs/nginx-selfsigned.crt /etc/ssl/certs/nginx-selfsigned.crt
COPY ./certs/nginx-selfsigned.key /etc/ssl/private/nginx-selfsigned.key
COPY ./certs/dhparam.pem /etc/nginx/dhparam.pem
COPY ./snippets /snippets
