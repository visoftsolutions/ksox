from sseclient import SSEClient

URL = "http://localhost:7979/api/public/ohlc"

messages = SSEClient(URL)
for msg in messages:
    print(msg)