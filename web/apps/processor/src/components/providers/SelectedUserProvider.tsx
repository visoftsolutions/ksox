import { Asset } from "@packages/types/asset";
import { User } from "@packages/types/user";
import {
  Accessor,
  createContext,
  createSignal,
  JSX,
  Setter,
  useContext,
} from "solid-js";

interface SelectedUserContextValue {
  selectedUser: Accessor<User | undefined>;
  setSelectedUser: Setter<User | undefined>;
}

export const [selectedUser, setSelectedUser] = createSignal<User | undefined>(
  undefined,
);
const SelectedUserContext = createContext<SelectedUserContextValue>({
  selectedUser,
  setSelectedUser,
});
export function SelectedUserProvider(props: { children: JSX.Element }) {
  return (
    <SelectedUserContext.Provider value={{ selectedUser, setSelectedUser }}>
      {props.children}
    </SelectedUserContext.Provider>
  );
}
export function useSelectedUser() {
  return useContext<SelectedUserContextValue>(SelectedUserContext);
}
