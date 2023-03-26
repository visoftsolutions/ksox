import axios from "axios";
import { z } from "zod";
import { AssetResponse, PUBLIC_URL, SearchRequest } from "./mod";

export const URL = PUBLIC_URL + "/search";

export function get(params: SearchRequest) {
  return axios.get(URL, { params: params }).then((r) => z.array(z.tuple([z.number(), z.tuple([AssetResponse, AssetResponse])])).parse(r.data));
}
