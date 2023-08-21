import {
  Accessor,
  JSX,
  createContext,
  createSignal,
  onMount,
  useContext,
} from "solid-js";
import { WalletClient } from "viem";
import params from "@packages/utils/params";
import {
  GenerateMessageRequest,
  GenerateMessageResponse,
  ValidateSignatureRequest,
  SessionResponse,
} from "./SessionProvider/models";

export const [session, setSession] = createSignal<SessionResponse | undefined>(
  undefined,
);
const SessionContext =
  createContext<Accessor<SessionResponse | undefined>>(session);
export function SessionProvider(props: {
  children: JSX.Element;
  api_url: string;
}) {
  onMount(async () => {
    const response = await fetch(`${props.api_url}/auth/session`, {
      method: "GET",
      credentials: "same-origin",
    })
      .then((r) => r.json())
      .then((r) => SessionResponse.nullable().parse(r));

    if (response != null) {
      setSession(response);
    }
  });

  return (
    <SessionContext.Provider value={session}>
      {props.children}
    </SessionContext.Provider>
  );
}
export function useSession() {
  return useContext<Accessor<SessionResponse | undefined>>(SessionContext);
}

export async function login(api_url: string, wallet: WalletClient) {
  const address = await wallet.getAddresses().then((addresses) => addresses[0]);

  const generateMessageResponse = await fetch(
    `${api_url}/auth?${params(
      GenerateMessageRequest.parse({
        address,
      }),
    )}`,
    {
      method: "GET",
    },
  )
    .then((r) => r.json())
    .then((r) => GenerateMessageResponse.parse(r));

  const signature = await wallet.signMessage({
    account: address,
    message: generateMessageResponse.message,
  });

  const session = await fetch(`${api_url}/auth`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(
      ValidateSignatureRequest.parse({
        address,
        signature,
      }),
    ),
  })
    .then((r) => r.json())
    .then((r) => SessionResponse.parse(r));

  return session;
}

export async function logout(api_url: string) {
  await fetch(`${api_url}/auth`, {
    method: "DELETE",
    credentials: "same-origin",
  }).then((r) => r.text());
  return undefined;
}
