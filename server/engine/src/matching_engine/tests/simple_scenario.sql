-- insert assets
insert into spot.assets (id, name, symbol, maker_fee_num, maker_fee_denum, taker_fee_num, taker_fee_denum)
VALUES ('b89ac651-e3ef-4902-9549-f3d29b582233', 'bitcoin', 'BTC', 11, 10000, 17, 10000);
insert into spot.assets (id, name, symbol, maker_fee_num, maker_fee_denum, taker_fee_num, taker_fee_denum)
VALUES ('f96083a1-c5d5-4a1b-809b-0fa4eca0b051', 'ethereum', 'ETH', 23, 1000, 32, 1000);

-- insert users
insert into public.users (id, address) VALUES ('418704df-53dc-449d-8767-12578c5f0d61', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1');
insert into public.users (id, address) VALUES ('e106a9b0-8b7a-40f8-92e3-ce17af265f8d', '0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b2');

-- insert valuts for users
insert into spot.valuts (user_id, asset_id, balance)
VALUES ('418704df-53dc-449d-8767-12578c5f0d61', 'b89ac651-e3ef-4902-9549-f3d29b582233', 10007);
insert into spot.valuts (user_id, asset_id, balance)
VALUES ('418704df-53dc-449d-8767-12578c5f0d61', 'f96083a1-c5d5-4a1b-809b-0fa4eca0b051', 59);
insert into spot.valuts (user_id, asset_id, balance)
VALUES ('e106a9b0-8b7a-40f8-92e3-ce17af265f8d', 'b89ac651-e3ef-4902-9549-f3d29b582233', 101);
insert into spot.valuts (user_id, asset_id, balance)
VALUES ('e106a9b0-8b7a-40f8-92e3-ce17af265f8d', 'f96083a1-c5d5-4a1b-809b-0fa4eca0b051', 703);

update spot.valuts
Set balance = 1292384030948230482304412
Where user_id = '418704df-53dc-449d-8767-12578c5f0d61' And asset_id = 'b89ac651-e3ef-4902-9549-f3d29b582233';
