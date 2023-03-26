import requests
from worker.const import BASE_URL
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME, logout
from worker.private.models import SubmitRequest

URL = f"{BASE_URL}/private/submit"

session = login(PRIVATE_KEY)

response = requests.post(
    URL,
    cookies={AUTH_COOKIE_NAME: session.session_id},
    json=SubmitRequest(
        quote_asset_id="5864a1b9-4ae1-424f-bdb4-f644cb359463",
        base_asset_id="7a99f052-d941-4dcc-aabd-6695c24deed5",
        quote_asset_volume="399",
        base_asset_volume="2000",
    ).__dict__,
)
print(response.text)

logout(session)
