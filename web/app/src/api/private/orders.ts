import { z } from "zod";
import { Pagination } from "~/api/mod";
import { PRIVATE_URL } from "./mod";
import { Order } from "~/types/order";
import axios from "axios";

export const URL = PRIVATE_URL + "/orders";

export function get(params?: Pagination) {
  return axios.get(URL, { withCredentials: true, params: params }).then((r) => z.array(Order).parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse", { withCredentials: true });
}
