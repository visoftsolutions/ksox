import { z } from "zod";
import { Pagination } from "~/api/mod";
import { SessionId } from "~/api/auth/mod";
import { COOKIE_NAME, PRIVATE_URL } from "./mod";
import { Trade } from "~/types/trade";

export const URL = PRIVATE_URL + "/trades";

async function get(session: SessionId, pagination?: Pagination) {
  return fetch(URL, {
    method: "get",
    headers: {
      "Content-Type": "application/json",
      Cookie: `${COOKIE_NAME}=${session}`,
    },
    body: JSON.stringify(pagination),
  })
    .then((r) => r.json())
    .then((r) => z.array(Trade).parse(r));
}

async function sse(session: SessionId) {}
