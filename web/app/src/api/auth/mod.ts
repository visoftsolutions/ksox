import { z } from "zod";

export const EvmAddress = z
  .string()
  .length(2 + 20)
  .regex(/0[xX][0-9a-fA-F]+/);
export const Nonce = z
  .string()
  .length(2 + 32)
  .regex(/0[xX][0-9a-fA-F]+/);
export const Signature = z
  .string()
  .length(2 + 65)
  .regex(/0[xX][0-9a-fA-F]+/);
export const SessionId = z
  .string()
  .length(2 + 32)
  .regex(/0[xX][0-9a-fA-F]+/);
export const UserId = z.string().uuid();
export const Expiration = z.number().nonnegative();
export const StringResponse = z.string();

export type EvmAddress = z.infer<typeof EvmAddress>;
export type Nonce = z.infer<typeof Nonce>;
export type Signature = z.infer<typeof Signature>;
export type SessionId = z.infer<typeof SessionId>;
export type UserId = z.infer<typeof UserId>;
export type Expiration = z.infer<typeof Expiration>;
export type StringResponse = z.infer<typeof StringResponse>;

export const GenerateNonceRequest = z.object({
  address: EvmAddress,
});
type GenerateNonceRequest = z.infer<typeof GenerateNonceRequest>;

export const GenerateNonceResponse = z.object({
  nonce: Nonce,
  expiration: Expiration,
});

export const ValidateSignatureRequest = z.object({
  address: EvmAddress,
  signature: Signature,
});

export const ValidateSignatureResponse = z.object({
  session_id: SessionId,
  user_id: UserId,
  expiration: Expiration,
});

export const User = z.object({
  session_id: SessionId,
  user_id: UserId,
});
