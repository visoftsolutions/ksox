import { z } from "zod";
import { EvmAddress, Expiration, Nonce, SessionId, Signature, UserId } from "./mod";

export const GenerateNonceRequest = z.object({
  address: EvmAddress,
});
type GenerateNonceRequest = z.infer<typeof GenerateNonceRequest>;

const GenerateNonceResponse = z.object({
  nonce: Nonce,
  expiration: Expiration,
});

const ValidateSignatureRequest = z.object({
  address: EvmAddress,
  signature: Signature,
});

const ValidateSignatureResponse = z.object({
  session_id: SessionId,
  user_id: UserId,
  expiration: Expiration,
});

export const User = z.object({
  session_id: SessionId,
  user_id: UserId,
});
