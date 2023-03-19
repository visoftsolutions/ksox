import requests
from sseclient import SSEClient
from worker.const import BASE_URL
from datetime import datetime, timezone

URL = f"{BASE_URL}/public/ohlcv"

response = requests.get(
    URL,
    json={
        "quote_asset_id": "5864a1b9-4ae1-424f-bdb4-f644cb359463",
        "base_asset_id": "7a99f052-d941-4dcc-aabd-6695c24deed5",
        "kind": "Interval",
        "reference_point": "2023-03-19T12:00:00.000000Z",
        "span": 100000000,
    }
)
print(response.json())

# response = SSEClient(
#     f"{URL}/sse",
#     json={
#         "quote_asset_id": "5864a1b9-4ae1-424f-bdb4-f644cb359463",
#         "base_asset_id": "7a99f052-d941-4dcc-aabd-6695c24deed5",
#         "kind": "Interval",
#         "reference_point": "2023-03-18T15:30:45.123456Z",
#         "span": 100000000,
#     }
# )
# for event in response:
#     if event.data:
#         print(event.data)
