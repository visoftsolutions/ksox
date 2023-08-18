import {
  Accessor,
  createContext,
  createEffect,
  createResource,
  createSignal,
  JSX,
  onMount,
  useContext,
} from "solid-js";
import { ContractResponse } from "@packages/types/contract";
import { Address } from "./SessionProvider/models";
import { api } from "@apps/exchange/src/root";

const ContractAddressContext = createContext<Accessor<Address>>();

export function ContractAddressProvider(props: { children: JSX.Element }) {
  const [data, { refetch }] = createResource(async () => {
    return await fetch(`${api}/public/contract`)
      .then((r) => r.json())
      .then((r) => ContractResponse.parse(r));
  });
  const [contractAddress, setContractAddress] = createSignal("0x");

  onMount(() => {
    refetch();
  });

  createEffect(() => {
    const result = data();
    if (result && result?.contract_address) {
      setContractAddress(result.contract_address);
      console.log(`Treasury Contract Address: ${result.contract_address}`);
    }
  });

  return (
    <ContractAddressContext.Provider value={contractAddress}>
      {props.children}
    </ContractAddressContext.Provider>
  );
}
export function useContractAddress() {
  return useContext<Accessor<Address> | undefined>(ContractAddressContext);
}