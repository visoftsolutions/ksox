import requests
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/assets"

response = requests.get(URL).json()
print(response)
