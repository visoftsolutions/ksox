import { z } from "zod";

export const EvmAddress = z
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

export type EvmAddress = z.infer<typeof EvmAddress>;
export type Signature = z.infer<typeof Signature>;
export type SessionId = z.infer<typeof SessionId>;
export type UserId = z.infer<typeof UserId>;
export type Expiration = z.infer<typeof Expiration>;

export const GenerateMessageRequest = z.object({
  address: EvmAddress,
});
export type GenerateMessageRequest = z.infer<typeof GenerateMessageRequest>;

export const GenerateMessageResponse = z.object({
  message: z.string(),
  expiration: Expiration,
});
export type GenerateMessageResponse = z.infer<typeof GenerateMessageResponse>;

export const ValidateSignatureRequest = z.object({
  address: EvmAddress,
  signature: Signature,
});
export type ValidateSignatureRequest = z.infer<typeof ValidateSignatureRequest>;

export const ValidateSignatureResponse = z.object({
  sessionId: SessionId,
  userId: UserId,
  expiration: Expiration,
});
export type ValidateSignatureResponse = z.infer<typeof ValidateSignatureResponse>;

export const User = z.object({
  sessionId: SessionId,
  userId: UserId,
});
export type User = z.infer<typeof User>;
