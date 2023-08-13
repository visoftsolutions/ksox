# Run app
```
docker compose up --build
```

# Prune Docker
```
docker system prune -a
```

## run code in minikube
make sure you have minikube, docker and skaffold installed
make sure you are in a docker group
```shell
minikube start --driver=docker --cpus 16 --memory 8192 --addons ingress
```

To deploy application to minikube do
```shell
kubectl config use-context minikube
skaffold run
```

you can now connect with your browser to ingress, ask minikube for ip
```shell
minikube ip
```

Inject routes to /etc/hosts for dns resolution
```shell
./minikube-inject-hosts.sh
```

To remove application from minikube do
```shell
kubectl config use-context minikube
skaffold delete
```

## run code in production
To deploy application to production do
```shell
kubectl config use-context visoft-prod
skaffold -d registry.internal.visoft.solutions run
```

To remove application from production do
```shell
kubectl config use-context visoft-prod
skaffold -d registry.internal.visoft.solutions delete
```
