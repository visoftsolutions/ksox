import requests

URL = "http://localhost:7979/api/public/search"

response = requests.get(URL).text
print(response)
