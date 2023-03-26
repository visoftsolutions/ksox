import { z } from "zod";
import { OhlcvRequest, PUBLIC_URL } from "./mod";
import { Candlestick } from "~/types/candlestick";
import axios from "axios";

export const URL = PUBLIC_URL + "/ohlcv";

export function get(params: OhlcvRequest) {
  return axios.get(URL, { params: params }).then((r) => z.array(Candlestick).parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse");
}
