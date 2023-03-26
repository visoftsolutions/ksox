import requests
from worker.const import BASE_URL
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME, logout
from worker.private.models import MintRequest

URL = f"{BASE_URL}/private/mint"

session = login(PRIVATE_KEY)

response = requests.post(
    URL,
    cookies={AUTH_COOKIE_NAME: session.session_id},
    json=MintRequest(
        asset_id="5864a1b9-4ae1-424f-bdb4-f644cb359463", amount="100"
    ).__dict__,
)
print(response.text)

logout(session)
