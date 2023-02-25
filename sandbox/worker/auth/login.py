import datetime
import requests
from worker.auth.models import *
from web3.auto import w3
from web3 import Account
from eth_account.messages import encode_defunct
from colorama import Fore, Style

PRIVATE_KEY = "637cf0cb529ff41499db61e0913674570ebd68c9d7c9a7a021cffd3c02c85427" # TOP-SECRET

AUTH_URL = "http://localhost:7979/api/auth"

def login(private_key) -> ValidateSignatureResponse:
    address = Account.from_key(private_key).address

    generate_nonce_raw = requests.get(AUTH_URL, json=GenerateNonceRequest(address).__dict__)
    generate_nonce_response = GenerateNonceResponse(**generate_nonce_raw.json())

    signature = w3.eth.account.sign_message(
        encode_defunct(hexstr=generate_nonce_response.nonce), private_key=private_key
    ).signature.hex()

    print(f"nonce: {generate_nonce_response.nonce}")
    print(f"signature: {signature}")

    validate_signature_raw = requests.post(
        AUTH_URL, json=ValidateSignatureRequest(address, signature).__dict__
    )
    validate_signature_response = ValidateSignatureResponse(**validate_signature_raw.json())

    print(
        f"session_id: {validate_signature_response.session_id} expiring at: {datetime.datetime.now() + datetime.timedelta(seconds=validate_signature_response.expiration)}"
    )

    print(Fore.GREEN + "Logged in!" + Style.RESET_ALL + " as: " + Fore.BLUE + validate_signature_response.user_id + Style.RESET_ALL)
    return validate_signature_response

if __name__ == "__main__":
    login(PRIVATE_KEY)