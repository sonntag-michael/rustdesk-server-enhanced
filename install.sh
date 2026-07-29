#!/bin/bash

if [ ! -f .env ]
then
    echo "Please create .env file first (based on .env.example)!"
    exit
fi

# Get configuration variables from .env file
source ./.env

# Prepare for database
mkdir -p data
chmod 770 data
touch data/database.db
chmod 660 data/database.db
chown :1001 data data/*

# Fill in random key in .env if empty 
if [ -z `awk -F '=' '/^SECRET_KEY/{printf $NF}' .env` ]
then
    echo "Generating server secret"
    KEY=`openssl rand -base64 32`
    sed -i "s/^SECRET_KEY.*/SECRET_KEY=\"${KEY}\"/" .env
fi

# Prepare nginx configuration
if ! grep -q "zone=backend_zone:10m rate=10r/m;" /etc/nginx/nginx.conf; then
    echo "Inserting rate limiting into /etc/nginx/nginx.conf"
    # Must be inserted into the file nginx.conf into the "http" block
    sed -i "s/http {/http {\n    # Rate limiting\n    limit_req_zone \$binary_remote_addr zone=backend_zone:10m rate=1r\/s;/" /etc/nginx/nginx.conf
fi
if [ ! -f /etc/nginx/conf.d/rustdesk-api.conf ]
then
    echo "Deploying nginx API website config"
    cp nginx.conf /etc/nginx/conf.d/rustdesk-api.conf
    if [ -n ${SERVER_DOMAIN_NAME} ]
    then
        echo "Inserting webserver domain name '${SERVER_DOMAIN_NAME}' into '/etc/nginx/conf.d/rustdesk-api.conf'"
        sed -i "s/\$SERVER-DOMAIN-NAME\$/${SERVER_DOMAIN_NAME}/" /etc/nginx/conf.d/rustdesk-api.conf
    else
        echo "Please manually check the webserver domain name is correct in '/etc/nginx/conf.d/rustdesk-api.conf' !"
    fi
    systemctl reload nginx
else
    echo "Nginx website config already exists..."
fi

echo "Installation finished. Please now configure httpS for the website!"
