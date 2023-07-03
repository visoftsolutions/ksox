#!/bin/bash

cleanup() {
  # Remove Ingress hosts from /etc/hosts
  echo "$unique_dns" | while read -r host; do
    sudo sed -i "/$host/d" /etc/hosts
  done

  echo "Ingress hosts removed from /etc/hosts."
  exit 0;
}

# Register cleanup function to be called on Ctrl+C
trap cleanup INT

# Get all Ingress resources from Minikube
dns=$(kubectl get certificate -n prod -o jsonpath='{range .items[*]}{.spec.dnsNames[*]}{"\n"}{end}')
ip=$(kubectl get gateway prod-gateway -n prod -o jsonpath='{.spec.addresses[0].value}')

# Remove duplicates from Ingress hosts
unique_dns=$(echo "$dns" | awk '!seen[$0]++')

# Add Ingress hosts to /etc/hosts
echo "$unique_dns" | while read -r host; do
  echo "$ip $host" | sudo tee -a /etc/hosts >/dev/null
done

echo "Ingress hosts added to /etc/hosts."

# Wait for user input to continue
read -rp "Press Enter to remove Ingress hosts from /etc/hosts..."

# Remove Ingress hosts from /etc/hosts
echo "$unique_dns" | while read -r host; do
  sudo sed -i "/$host/d" /etc/hosts
done

echo "Ingress hosts removed from /etc/hosts."
