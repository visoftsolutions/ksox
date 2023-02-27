from sseclient import SSEClient
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME

URL = "http://localhost:7878/private/balance"

session = login(PRIVATE_KEY)

messages = SSEClient(URL, cookies={AUTH_COOKIE_NAME: session.session_id})
for msg in messages:
    if msg: print(msg)
