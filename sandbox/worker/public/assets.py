import requests
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/assets"

response = requests.get(URL, params={"ids": "223808d0-7647-4078-89d9-2a5f76bc507a"})
print(response.url)
print(response)
print(response.text)

# URL = f"{BASE_URL}/public/assets?ids[0]=aaa"

# response = requests.get(URL)
# print(response.text)