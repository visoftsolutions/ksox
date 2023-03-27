import { z } from "zod";
import axios from "axios";

export const BASE_URL = "http://localhost:7979/api";

export const Pagination = z.object({
  limit: z.number().nonnegative(),
  offset: z.number().nonnegative(),
});
export type Pagination = z.infer<typeof Pagination>;

const URL = BASE_URL;

export async function get() {
  return axios.get(URL).then((r) => z.string().parse(r.data));
}

export function sse() {
  return new EventSource(URL + "/sse");
}
