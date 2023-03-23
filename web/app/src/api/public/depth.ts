import { z } from "zod";
import { SessionId } from "~/api/auth/mod";
import { DepthRequest, PriceLevel, PUBLIC_URL } from "./mod";

export const URL = PUBLIC_URL + "/depth";

async function get(request: DepthRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => z.array(PriceLevel).parse(r));
}

async function sse(session: SessionId) {}
