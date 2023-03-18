class Request:
    def __init__(self, quote_asset_id: str, base_asset_id: str) -> None:
        self.quote_asset_id = quote_asset_id
        self.base_asset_id = base_asset_id