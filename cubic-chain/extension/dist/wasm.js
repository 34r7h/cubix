import init, { CubicKey } from 'cubic-chain';
export async function initWasm() {
    await init();
}
export class WasmCrypto {
    static createKey(seed) {
        return new CubicKey(seed);
    }
}
