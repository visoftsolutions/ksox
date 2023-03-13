class CancelRequest:
    def __init__(self, order_id: str) -> None:
        self.order_id = order_id


class MintRequest:
    def __init__(self, asset_id: str, amount: str) -> None:
        self.asset_id = asset_id
        self.amount = amount


class BurnRequest:
    def __init__(self, asset_id: str, amount: str) -> None:
        self.asset_id = asset_id
        self.amount = amount


class SubmitRequest:
    def __init__(
        self,
        quote_asset_id: str,
        base_asset_id: str,
        quote_asset_volume: str,
        base_asset_volume: str,
    ) -> None:
        self.quote_asset_id = quote_asset_id
        self.base_asset_id = base_asset_id
        self.quote_asset_volume = quote_asset_volume
        self.base_asset_volume = base_asset_volume
