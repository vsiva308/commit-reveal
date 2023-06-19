import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import fs from "fs";

const fileName = __dirname + '/info.json';

interface Config {
    id: string, //BN.toJSON()
    pubkeys: string[],
    salt: string,
}

export function createSalt(len: number): string {
    let chars = "0123456789abcdefghijklmnopqrstuvwxyz!@#$%^&*()ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let password = "";
    for (let i = 0; i <= len; i++) {
        var randomNumber = Math.floor(Math.random() * chars.length);
        password += chars.substring(randomNumber, randomNumber+1);
    }

    const configContents = fs.readFileSync(fileName, 'utf8');
    const config: Config = JSON.parse(configContents);

    config.salt = password;
    fs.writeFileSync(fileName, JSON.stringify(config, null, 2));

    return password;
}

export function getSalt(): string {
    const configContents = fs.readFileSync(fileName, 'utf8');

    const config: Config = JSON.parse(configContents);
    return config.salt;
}

export function setID(id: anchor.BN) {
    const configContents = fs.readFileSync(fileName, 'utf8');

    const config: Config = {
        id: id.toJSON(),
        pubkeys: [],
        salt: ""
    };

    fs.writeFileSync(fileName, JSON.stringify(config, null, 2));
}

export function getID(): anchor.BN {
    const configContents = fs.readFileSync(fileName, 'utf8');
    const config: Config = JSON.parse(configContents);

    let id = new anchor.BN(config.id);

    return id;
}

export function setPubkeys(keys: PublicKey[]) {
    const configContents = fs.readFileSync(fileName, 'utf8');
    const config: Config = JSON.parse(configContents);
    config.pubkeys = [];
    for (const key of keys) {
        config.pubkeys.push(key.toJSON());
    }

    fs.writeFileSync(fileName, JSON.stringify(config, null, 2));
}

export function getPubkeys(): PublicKey[] {
    let pubkeys = [];

    const configContents = fs.readFileSync(fileName, 'utf8');
    const config: Config = JSON.parse(configContents);

    for (const key of config.pubkeys) {
        pubkeys.push(new PublicKey(key))
    }

    return pubkeys;
}

export function getPubkey(index: number): PublicKey {
    const configContents = fs.readFileSync(fileName, 'utf8');
    const config: Config = JSON.parse(configContents);

    return new PublicKey(config.pubkeys[index]);
}
 
