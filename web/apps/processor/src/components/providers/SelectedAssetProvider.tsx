import { Asset } from "@packages/types/asset";
import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  Setter,
  useContext,
} from "solid-js";

export enum Nav {
  Home,
  Transfer,
  Settings,
  Notifications,
}

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
export function NavProvider(props: {
  children: JSX.Element;
  selected_asset: Asset;
}) {
  setSelectedAsset(props.selected_asset);
  return (
    <SelectedAssetContext.Provider value={{ selectedAsset, setSelectedAsset }}>
      {props.children}
    </SelectedAssetContext.Provider>
  );
}
export function useSelectedAsset() {
  return useContext<SelectedAssetContextValue>(SelectedAssetContext);
}