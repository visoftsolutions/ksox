import { z } from "zod";
import { SessionId } from "~/api/auth/mod";
import { PUBLIC_URL, TradesRequest } from "./mod";
import { Trade } from "~/types/trade";

export const URL = PUBLIC_URL + "/search";

async function get(request: TradesRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => z.array(Trade).parse(r));
}

async function sse(session: SessionId) {}
