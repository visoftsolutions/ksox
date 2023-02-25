import requests

URL = "http://localhost:7979/api/public/assets"

response = requests.get(URL).json()
print(response)