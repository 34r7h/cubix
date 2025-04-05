declare module 'cubic-chain' {
  export function init(): Promise<void>;

  export class CubicKey {
    constructor(seed: Uint8Array);
    next_key(): Uint8Array;
  }

  export class NetworkState {
    static current(): NetworkState;
    peers: string[];
    chainHeight: number;
  }

  export function generate_proof(): Promise<Uint8Array>;
  export function compute_position(hash: Uint8Array): { face: number; depth: number; x: number; y: number; };
} 