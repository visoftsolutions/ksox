import { z } from "zod";
import { Pagination } from "~/api/mod";
import { SessionId } from "~/api/auth/mod";
import { Order } from "~/types/order";
import { COOKIE_NAME, PRIVATE_URL } from "./mod";

async function get(session: SessionId, pagination: Pagination) {
  let response = await fetch(PRIVATE_URL, {
    method: "get",
    headers: {
        "Content-Type": "application/json",
        "Cookie": `${COOKIE_NAME}=${session}`,
    },
    body: JSON.stringify(pagination)
  })
  .then(response => response.json())
  .catch(error => {
    throw new Error("request failed");
  });

  console.log(response)
}

async function sse(session: SessionId) {}
