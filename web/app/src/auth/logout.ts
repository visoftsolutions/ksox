import { api } from "~/root";

export default async function logout() {
  const response = await fetch(`${api}/auth`, {
    method: "DELETE",
    credentials: "same-origin",
  }).then((r) => r.text());
  console.log(response);
  return undefined;
}
