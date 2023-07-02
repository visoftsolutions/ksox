# Generate self-signed certificates

```shell
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout nginx-selfsigned.key -out nginx-selfsigned.crt
```

# Generate strong Diffie Hellman group

```shell
openssl dhparam -out dhparam.pem 4096
```

# Add domain to your hosts file

```shell
vim /etc/hosts
```

```
[ip of server] internal.ksox.exchange
ex
127.0.0.1 internal.ksox.exchange
```
