import {
  Accessor,
  JSX,
  createContext,
  createSignal,
  useContext,
} from "solid-js";
import { mainnet } from "@wagmi/core";
import { CustomTransport, WalletClient } from "viem";
import params from "@web/utils/params";
import {
  GenerateMessageRequest,
  GenerateMessageResponse,
  ValidateSignatureRequest,
  ValidateSignatureResponse,
} from "./SessionProvider/models";

export const [session, setSession] = createSignal<
  ValidateSignatureResponse | undefined
>(undefined);
const SessionContext =
  createContext<Accessor<ValidateSignatureResponse | undefined>>(session);
export function SessionProvider(props: { children: JSX.Element }) {
  return (
    <SessionContext.Provider value={session}>
      {props.children}
    </SessionContext.Provider>
  );
}
export function useSession() {
  return useContext<Accessor<ValidateSignatureResponse | undefined>>(
    SessionContext
  );
}

export async function login(
  api_url: string,
  wallet: WalletClient<CustomTransport, typeof mainnet>
) {
  const address = await wallet.getAddresses().then((addresses) => addresses[0]);

  const generateMessageResponse = await fetch(
    `${api_url}/auth?${params(
      GenerateMessageRequest.parse({
        address,
      })
    )}`,
    {
      method: "GET",
    }
  )
    .then((r) => r.json())
    .then((r) => GenerateMessageResponse.parse(r));

  const signature = await wallet.signMessage({
    account: address,
    message: generateMessageResponse.message,
  });

  const validateSignatureResponse = await fetch(`${api_url}/auth`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(
      ValidateSignatureRequest.parse({
        address,
        signature,
      })
    ),
  })
    .then((r) => r.json())
    .then((r) => ValidateSignatureResponse.parse(r));

  return validateSignatureResponse;
}

export async function logout(api_url: string) {
  await fetch(`${api_url}/auth`, {
    method: "DELETE",
    credentials: "same-origin",
  }).then((r) => r.text());
  return undefined;
}
