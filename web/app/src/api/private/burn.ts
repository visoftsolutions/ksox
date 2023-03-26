import axios from "axios";
import { z } from "zod";
import { MintBurnRequest, PRIVATE_URL } from "./mod";

export const URL = PRIVATE_URL + "/burn";

export function post(payload: MintBurnRequest) {
  return axios
    .post(URL, payload, {
      withCredentials: true,
      headers: {
        "Content-Type": "application/json",
      },
    })
    .then((r) => z.string().parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse", { withCredentials: true });
}
