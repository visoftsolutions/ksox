import { z } from "zod";
import { SessionId, StringResponse } from "~/api/auth/mod";
import { COOKIE_NAME, PRIVATE_URL, SubmitRequest } from "./mod";

export const URL = PRIVATE_URL + "/submit";

async function get(session: SessionId, request: SubmitRequest) {
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
