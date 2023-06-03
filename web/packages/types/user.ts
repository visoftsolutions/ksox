import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { EvmAddress } from "@web/components/providers/SessionProvider/models";

export const User = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  address: EvmAddress,
  name: z.nullable(z.string()),
  phone: z.nullable(z.string()),
  email: z.nullable(z.string()),
});
export type User = z.infer<typeof User>;
