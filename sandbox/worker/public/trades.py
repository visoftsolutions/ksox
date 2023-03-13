import requests
from sseclient import SSEClient
from worker.const import BASE_URL
from worker.public.models import *

URL = f"{BASE_URL}/public/trades"

response = requests.get(
    URL,
    json={
        "quote_asset_id": "b6b20297-10ab-4f14-bff0-f630a09363e1",
        "base_asset_id": "b6b20297-10ab-4f14-bff0-f630a09363e1",
    },
)
print(response.text)

response = SSEClient(
    f"{URL}/sse",
    json={
        "quote_asset_id": "b6b20297-10ab-4f14-bff0-f630a09363e1",
        "base_asset_id": "b6b20297-10ab-4f14-bff0-f630a09363e1",
    },
)
for event in response:
    if event.data:
        print(event.data)
