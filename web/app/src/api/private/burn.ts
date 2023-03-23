import { z } from "zod";
import { SessionId, StringResponse } from "~/api/auth/mod";
import { COOKIE_NAME, MintBurnRequest, PRIVATE_URL } from "./mod";

export const URL = PRIVATE_URL + "/burn";

async function get(session: SessionId, request: MintBurnRequest) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
      Cookie: `${COOKIE_NAME}=${session}`,
    },
    body: JSON.stringify(request),
  })
    .then((r) => r.json())
    .then((r) => StringResponse.parse(r));
}

async function sse(session: SessionId) {}
