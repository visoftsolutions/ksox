import { z } from "zod";
import { PUBLIC_URL, TradesRequest } from "./mod";
import { Trade } from "~/types/trade";
import axios from "axios";

export const URL = PUBLIC_URL + "/search";

export function get(params: TradesRequest) {
  return axios.get(URL, { params: params }).then((r) => z.array(Trade).parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse");
}
