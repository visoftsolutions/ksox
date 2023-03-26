import requests
from worker.const import BASE_URL
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME, logout
from worker.private.models import CancelRequest

URL = f"{BASE_URL}/private/cancel"

session = login(PRIVATE_KEY)

response = requests.delete(
    URL,
    cookies={AUTH_COOKIE_NAME: session.session_id},
    params=CancelRequest(
        order_id="3774b22f-4b04-40bd-ae3f-900934d2af4c",
    ).__dict__,
)
print(response.text)

logout(session)
