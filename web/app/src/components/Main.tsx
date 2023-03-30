import { useParams } from "solid-start";

export default function Main() {
  const { baseAssetId, quoteAssetId } = useParams<{ baseAssetId: string; quoteAssetId: string }>();

  return (
    <div class="justify-center, grid h-full grid-cols-[1fr_auto] grid-rows-[1fr] items-center">
      {`baseAssetId: ${baseAssetId}, quoteAssetId: ${quoteAssetId}`}
    </div>
  );
}
