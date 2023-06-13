import { z } from "zod";

export const EnumValue = z.object({
  name: z.string(),
  description: z.string(),
  value: z.number(),
});

export type EnumValue = z.infer<typeof EnumValue>;
