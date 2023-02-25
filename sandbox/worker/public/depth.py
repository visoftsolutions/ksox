from sseclient import SSEClient

URL = "http://localhost:7979/api/public/depth"

messages = SSEClient(URL)
for msg in messages:
    print(msg)