import requests
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/search"

response = requests.get(URL).text
print(response)
