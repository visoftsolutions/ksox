#!/bin/bash

cleanup() {
  # Remove Ingress hosts from /etc/hosts
  echo "$unique_ingresses" | while read -r host; do
    sudo sed -i "/$host/d" /etc/hosts
  done

  echo "Ingress hosts removed from /etc/hosts."
  exit 0;
}

# Register cleanup function to be called on Ctrl+C
trap cleanup INT

# Get all Ingress resources from Minikube
ingresses=$(kubectl get ingress -o jsonpath='{range .items[*]}{.spec.rules[*].host}{"\n"}{end}')

# Remove duplicates from Ingress hosts
unique_ingresses=$(echo "$ingresses" | awk '!seen[$0]++')

# Add Ingress hosts to /etc/hosts
echo "$ingresses" | while read -r host; do
  ip=$(minikube ip)
  echo "$ip $host" | sudo tee -a /etc/hosts >/dev/null
done

echo "Ingress hosts added to /etc/hosts."

# Wait for user input to continue
read -rp "Press Enter to remove Ingress hosts from /etc/hosts..."

# Remove Ingress hosts from /etc/hosts
echo "$ingresses" | while read -r host; do
  sudo sed -i "/$host/d" /etc/hosts
done

echo "Ingress hosts removed from /etc/hosts."
