import { Accessor, JSX, createContext, createSignal, useContext } from "solid-js";
import { ValidateSignatureResponse } from "~/auth/mod";

export const [session, setSession] = createSignal<ValidateSignatureResponse | null>(null);
const SessionContext = createContext<Accessor<ValidateSignatureResponse | null>>(session);
export function SessionProvider(props: { children: JSX.Element }) {
  return <SessionContext.Provider value={session}>{props.children}</SessionContext.Provider>;
}
export function useSession() {
  return useContext<Accessor<ValidateSignatureResponse | null>>(SessionContext);
}