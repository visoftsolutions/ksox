import { mainnet } from "@wagmi/core";
import { Accessor, JSX, createContext, createSignal, useContext } from "solid-js";
import { CustomTransport, WalletClient } from "viem";

export const [wallet, setWallet] = createSignal<WalletClient<CustomTransport, typeof mainnet> | null>(null);
const WalletContext = createContext<Accessor<WalletClient<CustomTransport, typeof mainnet> | null>>(wallet);
export function WalletProvider(props: { children: JSX.Element }) {
  return <WalletContext.Provider value={wallet}>{props.children}</WalletContext.Provider>;
}
export function useWallet() {
  return useContext<Accessor<WalletClient<CustomTransport, typeof mainnet> | null>>(WalletContext);
}