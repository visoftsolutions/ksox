import { Accessor, JSX, createContext, createSignal, useContext } from "solid-js";
import { ValidateSignatureResponse } from "~/auth/mod";

export const [session, setSession] = createSignal<ValidateSignatureResponse | undefined>(undefined);
const SessionContext = createContext<Accessor<ValidateSignatureResponse | undefined>>(session);
export function SessionProvider(props: { children: JSX.Element }) {
  return <SessionContext.Provider value={session}>{props.children}</SessionContext.Provider>;
}
export function useSession() {
  return useContext<Accessor<ValidateSignatureResponse | undefined>>(SessionContext);
}
