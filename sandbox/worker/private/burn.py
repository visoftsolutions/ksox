import requests
from worker.auth import login, PRIVATE_KEY, AUTH_COOKIE_NAME, logout

URL = "http://localhost:7979/api/private/burn"

session = login(PRIVATE_KEY)

response = requests.get(URL, cookies={AUTH_COOKIE_NAME: session.session_id})
print(response.text)

logout(session)