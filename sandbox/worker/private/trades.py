import requests
from sseclient import SSEClient
from worker.const import BASE_URL
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME

URL = f"{BASE_URL}/private/trades"

session = login(PRIVATE_KEY)

response = requests.get(URL, cookies={AUTH_COOKIE_NAME: session.session_id}, json={})
print(response.text)

response = SSEClient(f"{URL}/sse", cookies={AUTH_COOKIE_NAME: session.session_id})
for event in response:
    if event.data:
        print(event.data)
