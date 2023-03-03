import requests
from sseclient import SSEClient
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/ohlc"

response = requests.get(URL)
print(response.text)

response = SSEClient(f"{URL}/sse")
for event in response:
    if event.data:
        print(event.data)
