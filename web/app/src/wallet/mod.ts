import { Signature } from "~/api/auth/mod";
import { ethers } from "ethers";

export default class Wallet {
    protected provider: ethers.BrowserProvider;
    constructor() {
        this.provider = new ethers.BrowserProvider((window as any).ethereum)
    }

    public async signdata(msg: string) {
        let signer = await this.provider.getSigner();
        return Signature.parse(await signer.signMessage(msg))
    }
}