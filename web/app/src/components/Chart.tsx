import { AssetResponse } from "./Markets";

export default function CreateChart(quote_asset?: AssetResponse, base_asset?: AssetResponse, width?: number, height?: number) {
  return () => <Chart quote_asset={quote_asset} base_asset={base_asset} width={width} height={height} />;
}

export function Chart(props: { quote_asset?: AssetResponse; base_asset?: AssetResponse; width?: number; height?: number }) {
  return <div />;
}
