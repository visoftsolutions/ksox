insert into public.users (id, address) VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1')
insert into public.users (id, address) VALUES ('1f71f5e5-ca64-4063-b1b3-43b7900be49a', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b2')

insert into spot.assets (id, name, symbol, maker_fee_num, maker_fee_denum, taker_fee_num, taker_fee_denum)
VALUES ('5864a1b9-4ae1-424f-bdb4-f644cb359463', 'bitcoin', 'BTC', 1,10,1,10)

insert into spot.assets (id, name, symbol, maker_fee_num, maker_fee_denum, taker_fee_num, taker_fee_denum)
VALUES ('7a99f052-d941-4dcc-aabd-6695c24deed5', 'ethereum', 'ETH', 1,10,1,10)

insert into spot.orders (id, user_id, status, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_volume)
VALUES ('3b7adec3-8dab-40ee-87ac-9cbf30487a13', 'ead19fc2-9652-444d-8d3c-5256ae80a210', 'active','5864a1b9-4ae1-424f-bdb4-f644cb359463','7a99f052-d941-4dcc-aabd-6695c24deed5', 10, 10)

insert into spot.orders (user_id, status, quote_asset_id, base_asset_id, quote_asset_volume, base_asset_volume)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', 'active','5864a1b9-4ae1-424f-bdb4-f644cb359463','7a99f052-d941-4dcc-aabd-6695c24deed5', 10, 10)

insert into spot.trades (taker_id, order_id, taker_quote_volume, taker_base_volume, maker_quote_volume, maker_base_volume)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', '3b7adec3-8dab-40ee-87ac-9cbf30487a13', 10, 10, 10, 10)

insert into spot.valuts (user_id, asset_id, balance)
VALUES ('ead19fc2-9652-444d-8d3c-5256ae80a210', '7a99f052-d941-4dcc-aabd-6695c24deed5', 11)

UPDATE spot.valuts SET balance = 14000010
WHERE id = 'f98a35b104bc4eeb9506bb7c8c13d27f'