import { z } from "zod";
import { SessionId, StringResponse } from "~/api/auth/mod";
import { CancelRequest, COOKIE_NAME, PRIVATE_URL } from "./mod";

export const URL = PRIVATE_URL + "/cancel";

async function get(session: SessionId, request: CancelRequest) {
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
