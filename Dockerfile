FROM nginx
COPY ./dist /var/www/bytie-moe
COPY ./config/nginx.conf /etc/nginx/conf.d/default.conf