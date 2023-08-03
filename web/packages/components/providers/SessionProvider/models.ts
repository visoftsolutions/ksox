import { z } from "zod";

export const Address = z
  .string()
  .length(2 + 20 * 2)
  .regex(/0[xX][0-9a-fA-F]+/);
export const Signature = z
  .string()
  .length(2 + 65 * 2)
  .regex(/0[xX][0-9a-fA-F]+/);
export const SessionId = z
  .string()
  .length(2 + 32 * 2)
  .regex(/0[xX][0-9a-fA-F]+/);
export const UserId = z.string().uuid();
export const Expiration = z.number().nonnegative();

export type Address = z.infer<typeof Address>;
export type Signature = z.infer<typeof Signature>;
export type SessionId = z.infer<typeof SessionId>;
export type UserId = z.infer<typeof UserId>;
export type Expiration = z.infer<typeof Expiration>;

export const GenerateMessageRequest = z.object({
  address: Address,
});
export type GenerateMessageRequest = z.infer<typeof GenerateMessageRequest>;

export const GenerateMessageResponse = z.object({
  message: z.string(),
  expiration: Expiration,
});
export type GenerateMessageResponse = z.infer<typeof GenerateMessageResponse>;

export const ValidateSignatureRequest = z.object({
  address: Address,
  signature: Signature,
});
export type ValidateSignatureRequest = z.infer<typeof ValidateSignatureRequest>;

export const SessionResponse = z.object({
  address: Address,
  session_id: SessionId,
  user_id: UserId,
  expiration: Expiration,
});
export type SessionResponse = z.infer<typeof SessionResponse>;

export const User = z.object({
  session_id: SessionId,
  user_id: UserId,
});
export type User = z.infer<typeof User>;
