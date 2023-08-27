import { privateKeyToAccount } from "viem/accounts";
import { toHex } from "viem/utils";
import { randomBytes } from "crypto";

let privateKey = toHex(randomBytes(32));
let account = privateKeyToAccount(privateKey);

console.log(privateKey);
console.log(account.address);
