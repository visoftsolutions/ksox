insert into public.users (id, address) VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1');
insert into public.users (id, address) VALUES ('1f71f5e5-ca64-4063-b1b3-43b7900be49a', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b2');

insert into spot.assets (id, name, symbol, maker_fee, taker_fee)
VALUES ('5864a1b9-4ae1-424f-bdb4-f644cb359463', 'bitcoin', 'BTC', (1,10),(1,10));

insert into spot.assets (id, name, symbol, maker_fee, taker_fee)
VALUES ('7a99f052-d941-4dcc-aabd-6695c24deed5', 'ethereum', 'ETH', (1,10),(1,10));

insert into spot.valuts (user_id, asset_id, balance)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', '5864a1b9-4ae1-424f-bdb4-f644cb359463', 1000);

insert into spot.orders (user_id, is_active, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_volume, quote_asset_volume_left, maker_fee)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', true,'5864a1b9-4ae1-424f-bdb4-f644cb359463','7a99f052-d941-4dcc-aabd-6695c24deed5', 10, 10, 10, (1, 100));

insert into spot.orders (user_id, is_active, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_volume, quote_asset_volume_left, maker_fee)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', true,'5864a1b9-4ae1-424f-bdb4-f644cb359463','7a99f052-d941-4dcc-aabd-6695c24deed5', 12, 13, 10, (1, 100));

insert into spot.trades (quote_asset_id, base_asset_id, taker_id, order_id, taker_quote_volume, taker_base_volume, maker_quote_volume, maker_base_volume)
VALUES ('5864a1b9-4ae1-424f-bdb4-f644cb359463','7a99f052-d941-4dcc-aabd-6695c24deed5', 'ead19fc2-9652-444d-8d3c-5256ae80a210', '84843f17-3318-488d-816d-9777a073a59d', 133231, 11240, 10, 10);

UPDATE spot.valuts
SET balance = 100
WHERE user_id = 'ead19fc2-9652-444d-8d3c-5256ae80a210' AND asset_id = '5864a1b9-4ae1-424f-bdb4-f644cb359463';