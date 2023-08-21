import { Address } from "@packages/components/providers/SessionProvider/models";
import { z } from "zod";

export const ContractResponse = z.object({
  contract_address: Address
});
export type ContractResponse = z.infer<typeof ContractResponse>;
