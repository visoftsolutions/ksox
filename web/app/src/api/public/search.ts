import { z } from "zod";
import { SessionId } from "~/api/auth/mod";
import { AssetResponse, PUBLIC_URL, SearchRequest } from "./mod";

export const URL = PUBLIC_URL + "/search";

async function get(request: SearchRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => z.array(z.tuple([z.number(), z.tuple([AssetResponse, AssetResponse])])).parse(r));
}

async function sse(session: SessionId) {}
