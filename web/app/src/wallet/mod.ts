import { Signature } from "~/api/auth/mod";
import { Eip1193Provider, ethers } from "ethers";

export default class Wallet {
  provider: ethers.BrowserProvider;
  constructor() {
    this.provider = new ethers.BrowserProvider((window as any).ethereum as Eip1193Provider);
  }

  public async signdata(msg: string) {
    const signer = await this.provider.getSigner();
    return Signature.parse(await signer.signMessage(msg));
  }
}
