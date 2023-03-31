import { ethers } from "ethers";

export function toWei(number: number): bigint {
    return ethers.utils.parseEther(number.toFixed(6)).toBigInt()
}

export function fromWei(number: bigint): number {
    return Number(ethers.utils.formatEther(number))
}