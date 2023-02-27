from worker.auth.login import AUTH_URL, login, PRIVATE_KEY
import requests
from colorama import Fore, Style

AUTH_COOKIE_NAME = "session_id"


def logout(session):
    logout_raw = requests.delete(
        AUTH_URL, cookies={AUTH_COOKIE_NAME: session.session_id}
    )
    print(logout_raw.text)
    print(Fore.RED + "Logged out!" + Style.RESET_ALL)


if __name__ == "__main__":
    # login first :)
    session = login(PRIVATE_KEY)
    logout(session)
