import { createStore } from "solid-js/store";

export interface AuthStore {
  isLoggedIn: boolean;
}

export const [store, setStore] = createStore<AuthStore>({
  isLoggedIn: false,
});
