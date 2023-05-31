import { mainnet } from "@wagmi/core";
import { CustomTransport, WalletClient } from "viem";
import { api } from "~/root";
import params from "~/utils/params";
import { GenerateMessageRequest, GenerateMessageResponse, ValidateSignatureRequest, ValidateSignatureResponse } from "./mod";

export default async function login(wallet: WalletClient<CustomTransport, typeof mainnet>) {
  const address = await wallet.getAddresses().then((addresses) => addresses[0]);

  const generateMessageResponse = await fetch(
    `${api}/auth?${params(
      GenerateMessageRequest.parse({
        address,
      })
    )}`,
    {
      method: "GET",
    }
  )
    .then((r) => r.json())
    .then((r) => GenerateMessageResponse.parse(r));

  const signature = await wallet.signMessage({
    account: address,
    message: generateMessageResponse.message,
  });

  const validateSignatureResponse = await fetch(`${api}/auth`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(
      ValidateSignatureRequest.parse({
        address,
        signature,
      })
    ),
  })
    .then((r) => r.json())
    .then((r) => ValidateSignatureResponse.parse(r));

  return validateSignatureResponse;
}
