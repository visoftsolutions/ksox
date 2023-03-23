import { z } from "zod";
import { Uuid } from "~/types/primitives/uuid";
import { Volume } from "~/types/primitives/volume";

export const BASE_URL = "http://localhost:7979";

export const Pagination = z.object({
  limit: z.number().nonnegative(),
  offset: z.number().nonnegative(),
});
export type Pagination = z.infer<typeof Pagination>;
