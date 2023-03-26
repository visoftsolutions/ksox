import requests
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/search"

response = requests.get(URL, params={"input": "btceth"})

for element in response.json():
    print(element[0], element[1][0]["symbol"], element[1][1]["symbol"])
