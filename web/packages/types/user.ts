import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Address } from "@packages/components/providers/SessionProvider/models";

export const User = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  address: Address,
  name: z.nullable(z.string()),
  phone: z.nullable(z.string()),
  email: z.nullable(z.string()),
});
export type User = z.infer<typeof User>;
