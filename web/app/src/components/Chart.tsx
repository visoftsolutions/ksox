import { Asset } from "~/types/asset";
import { Market } from "~/utils/providers/MarketProvider";

export default function CreateChart(market: Market) {
  return () => <Chart quote_asset={market.quote_asset} base_asset={market.base_asset} width={100} height={100} />;
}

export function Chart(props: { quote_asset?: Asset; base_asset?: Asset; width?: number; height?: number }) {
  return <div />;
}
