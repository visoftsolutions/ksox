import axios from "axios";
import { z } from "zod";
import { CancelRequest, PRIVATE_URL } from "./mod";

export const URL = PRIVATE_URL + "/cancel";

export function cancel(params: CancelRequest) {
  return axios.delete(URL, { withCredentials: true, params: params }).then((r) => z.string().parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse", { withCredentials: true });
}
