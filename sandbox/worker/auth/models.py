class GenerateNonceRequest:
    def __init__(self, address: str) -> None:
        self.address = address


class GenerateNonceResponse:
    def __init__(self, nonce: str, expiration: int) -> None:
        self.nonce = nonce
        self.expiration = expiration


class ValidateSignatureRequest:
    def __init__(self, address: str, signature: str) -> None:
        self.address = address
        self.signature = signature


class ValidateSignatureResponse:
    def __init__(self, session_id: str, user_id: str, expiration: int) -> None:
        self.session_id = session_id
        self.user_id = user_id
        self.expiration = expiration
