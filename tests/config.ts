import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export const id = new anchor.BN(0);
export const pubkey = PublicKey.default;

export function createSalt(len: number): string {
    let chars = "0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let password = "";
    for (let i = 0; i <= len; i++) {
        var randomNumber = Math.floor(Math.random() * chars.length);
        password += chars.substring(randomNumber, randomNumber+1);
    }
    return password;
}

