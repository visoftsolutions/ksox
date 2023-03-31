import { mainnet } from "@wagmi/core";
import { CustomTransport, getAccount, WalletClient } from "viem";
import { api } from "~/root";
import params from "~/utils/params";
import { GenerateMessageRequest, GenerateMessageResponse, ValidateSignatureRequest, ValidateSignatureResponse } from "./mod";

export default async function logout() {
    const response = await fetch(
        `${api}/auth`,
        {
            method: "DELETE",
            credentials: "same-origin",
        }
    )
        .then((r) => r.text())
    console.log(response);
    return null;
}