SELECT orderbook.price, SUM(orderbook.volume) AS volume FROM 
	(SELECT 
	 	spot.orders.base_asset_volume/spot.orders.quote_asset_volume AS price,
		spot.orders.quote_asset_volume_left AS volume
	FROM spot.orders
	WHERE  spot.orders.is_active = true
		AND spot.orders.quote_asset_id = '9b1d1f98-c369-4d6a-afab-0247a8900573'
		AND spot.orders.base_asset_id = '9b1d1f98-c369-4d6a-afab-0247a8900573'
	) AS orderbook
GROUP BY orderbook.price;