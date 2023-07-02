minikube start --cpus 4 --memory 8192 --addons=metallb

kubectl apply -f - <<EOF
apiVersion: v1
data:
  config: |
    address-pools:
    - name: default
      protocol: layer2
      addresses:
      - 192.168.49.100-192.168.49.120
kind: ConfigMap
metadata:
  name: config
  namespace: metallb-system
EOF

kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v0.7.1/standard-install.yaml

helm install istio-base base \
    --atomic \
    --create-namespace \
    --repo https://istio-release.storage.googleapis.com/charts \
    --namespace istio-system \
    --wait

helm install istiod istiod \
    --atomic \
    --create-namespace \
    --repo https://istio-release.storage.googleapis.com/charts \
    --namespace istio-system \
    --wait

kubectl label namespace default istio-injection=enabled

helm install openebs openebs \
    --atomic \
    --create-namespace \
    --repo https://openebs.github.io/charts \
    --namespace openebs \
    --wait

kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.12.0/cert-manager.yaml

kubectl apply -f - <<EOF
apiVersion: v1
kind: Namespace
metadata:
  name: prod
EOF

kubectl apply -f - <<EOF
apiVersion: gateway.networking.k8s.io/v1beta1
kind: Gateway
metadata:
  name: prod-gateway
  namespace: prod
spec:
  gatewayClassName: istio
  addresses:
    - type: IPAddress
      value: 192.168.49.105
  listeners:
    - name: http
      port: 80
      protocol: HTTP
      allowedRoutes:
        namespaces:
          from: All
    - name: https
      port: 443
      protocol: HTTPS
      allowedRoutes:
        namespaces:
          from: All
      tls:
        mode: Terminate
        certificateRefs:
          - kind: Secret
            name: prod-ksox-finance-tls
EOF

kubectl apply -f - <<EOF
apiVersion: cert-manager.io/v1
kind: Issuer
metadata:
  name: prod-letsencrypt-http01
  namespace: prod
spec:
  selfSigned: {}
EOF