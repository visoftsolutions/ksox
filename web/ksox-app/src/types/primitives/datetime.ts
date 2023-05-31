import { z } from "zod";

export const Datetime = z.coerce.date();
export type Datetime = z.infer<typeof Datetime>;
