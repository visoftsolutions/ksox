import { z } from "zod";

export const Volume = z.coerce.bigint();
export type Volume = z.infer<typeof Volume>;
