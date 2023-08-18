import { z } from "zod";

export const ContractResponse = z.object({
  contract_address: z.string(),
});
export type ContractResponse = z.infer<typeof ContractResponse>;
