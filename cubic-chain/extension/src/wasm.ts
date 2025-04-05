import init, { CubicKey, compute_position } from 'cubic-chain';

export async function initWasm() {
    await (init as unknown as () => Promise<void>)();
}

export class WasmCrypto {
    static createKey(seed: Uint8Array): CubicKey {
        return new CubicKey(seed);
    }
} 