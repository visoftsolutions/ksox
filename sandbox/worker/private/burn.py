import requests
from worker.const import BASE_URL
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME, logout

URL = f"{BASE_URL}/private/burn"

session = login(PRIVATE_KEY)

response = requests.get(URL, cookies={AUTH_COOKIE_NAME: session.session_id})
print(response.text)

logout(session)
