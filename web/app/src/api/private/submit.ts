import axios from "axios";
import { z } from "zod";
import { PRIVATE_URL, SubmitRequest } from "./mod";

export const URL = PRIVATE_URL + "/submit";

export function post(payload: SubmitRequest) {
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
