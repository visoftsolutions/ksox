import { mainnet } from "@wagmi/core";
import { joinPaths } from "solid-start/islands/server-router";
import { CustomTransport, getAccount, WalletClient } from "viem";
import params from "~/utils/params";
import { GenerateMessageRequest, GenerateMessageResponse, ValidateSignatureRequest, ValidateSignatureResponse } from "./mod";

export default async function login(wallet: WalletClient<CustomTransport, typeof mainnet>) {
  const BASE_URL = location.pathname;
  const API_URL = joinPaths(BASE_URL, "/api");
  const AUTH_URL = joinPaths(API_URL, "/auth");

  const address = await wallet.getAddresses().then((addresses) => addresses[0]);

  const generateMessageResponse = await fetch(
    `${AUTH_URL}?${params(
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
    account: getAccount(address),
    message: generateMessageResponse.message,
  });

  const validateSignatureResponse = await fetch(`${AUTH_URL}`, {
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
