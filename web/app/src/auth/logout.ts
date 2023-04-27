import { api } from "~/root";

export default async function logout() {
  await fetch(`${api}/auth`, {
    method: "DELETE",
    credentials: "same-origin",
  }).then((r) => r.text());
  return undefined;
}
