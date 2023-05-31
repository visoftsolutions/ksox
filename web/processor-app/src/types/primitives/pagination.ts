import { z } from "zod";

export const Pagination = z.object({
  limit: z.number().nonnegative(),
  offset: z.number().nonnegative(),
});
export type Pagination = z.infer<typeof Pagination>;
