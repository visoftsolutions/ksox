import axios from "axios";
import { z } from "zod";
import { Asset } from "~/types/asset";
import { PUBLIC_URL } from "./mod";

const URL = PUBLIC_URL + "/assets";

export async function get() {
  return axios.get(URL).then((r) => z.array(Asset).parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse");
}
