import requests
from sseclient import SSEClient
from worker.const import BASE_URL

URL = f"{BASE_URL}/public/depth"

response = requests.get(URL, json={"quote_asset_id": "5864a1b9-4ae1-424f-bdb4-f644cb359463", "base_asset_id": "7a99f052-d941-4dcc-aabd-6695c24deed5", "precision": 1})
print(response.text)

response = SSEClient(f"{URL}/sse", json={"quote_asset_id": "5864a1b9-4ae1-424f-bdb4-f644cb359463", "base_asset_id": "7a99f052-d941-4dcc-aabd-6695c24deed5", "precision": 1})
for event in response:
    if event.data:
        print(event.data)
