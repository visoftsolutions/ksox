import { z } from "zod";
import { SessionId } from "~/api/auth/mod";
import { OhlcvRequest, PUBLIC_URL } from "./mod";
import { Candlestick } from "~/types/candlestick";

export const URL = PUBLIC_URL + "/depth";

async function get(request: OhlcvRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => z.optional(Candlestick).parse(r));
}

async function sse(session: SessionId) {}
