import { z } from "zod";

export const Volume = z.coerce.number();
export type Volume = z.infer<typeof Volume>;
