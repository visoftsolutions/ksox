import axios from "axios";
import { z } from "zod";
import { DepthRequest, PriceLevel, PUBLIC_URL } from "./mod";

export const URL = PUBLIC_URL + "/depth";

export async function get(params: DepthRequest) {
  return axios.get(URL, { params: params }).then((r) => z.array(PriceLevel).parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse");
}
