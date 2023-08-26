import { z } from "zod";
import { Datetime } from "./primitives/datetime";
import { Uuid } from "./primitives/uuid";
import { Address } from "@packages/components/providers/SessionProvider/models";

export const User = z.object({
  id: Uuid,
  created_at: Datetime,
  last_modification_at: Datetime,
  address: Address,
  name: z.string().nullable(),
});
export type User = z.infer<typeof User>;

export const UserRecognitionResult = z.object({
  score: z.number(),
  user: User,
});
export type UserRecognitionResult = z.infer<typeof UserRecognitionResult>;
