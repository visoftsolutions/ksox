import { Asset } from "@packages/types/asset";
import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  Setter,
  useContext,
} from "solid-js";

interface SelectedAssetContextValue {
  selectedAsset: Accessor<Asset | undefined>;
  setSelectedAsset: Setter<Asset | undefined>;
}

export const [selectedAsset, setSelectedAsset] = createSignal<
  Asset | undefined
>(undefined);
const SelectedAssetContext = createContext<SelectedAssetContextValue>({
  selectedAsset,
  setSelectedAsset,
});
export function SelectedAssetProvider(props: { children: JSX.Element }) {
  return (
    <SelectedAssetContext.Provider value={{ selectedAsset, setSelectedAsset }}>
      {props.children}
    </SelectedAssetContext.Provider>
  );
}
export function useSelectedAsset() {
  return useContext<SelectedAssetContextValue>(SelectedAssetContext);
}
