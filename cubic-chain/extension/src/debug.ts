import init, { 
  CubicKey,
  compute_position,
  generate_proof,
  NetworkState 
} from 'cubic-chain';

class DebugInterface {
  private wasmReady = false;
  private networkState!: NetworkState;
  private logElement: HTMLElement;

  constructor() {
    this.initWasm().then(() => this.startMonitoring());
    this.logElement = document.getElementById('debug-log')!;
  }

  private async initWasm() {
    try {
      await (init as unknown as () => Promise<void>)();
      this.wasmReady = true;
      this.log('WASM module initialized');
    } catch (err: any) {
      this.log(`WASM Error: ${err.message}`, 'error');
    }
  }

  private startMonitoring() {
    setInterval(() => this.updateNetworkStatus(), 2000);
    this.initKeyRotation();
    this.startCubeBuilder();
  }

  private async updateNetworkStatus() {
    if (!this.wasmReady) return;

    const state = NetworkState.current();
    document.getElementById('peer-count')!.textContent = 
      state.peers.length.toString();
    document.getElementById('block-height')!.textContent = 
      state.chainHeight.toString();
  }

  private initKeyRotation() {
    const seed = crypto.getRandomValues(new Uint8Array(32));
    const key = new CubicKey(seed);
    
    setInterval(() => {
      const nextKey = key.next_key();
      document.getElementById('next-address')!.textContent = 
        this.truncateHash(nextKey);
    }, 5000);
  }

  private startCubeBuilder() {
    setInterval(async () => {
      const cubeProgress = document.getElementById('cube-progress') as HTMLProgressElement;
      const currentCount = parseInt(cubeProgress.value.toString());
      
      if (currentCount < 27) {
        cubeProgress.value = currentCount + 1;
        this.log(`Added transaction to cube ${currentCount + 1}/27`);
        
        if (currentCount === 26) {
          const proof = await generate_proof();
          this.log(`Cube proof generated: ${this.truncateHash(proof)}`);
        }
      }
    }, 1000);
  }

  private log(message: string, type: 'info' | 'error' = 'info') {
    const entry = document.createElement('div');
    entry.style.color = type === 'error' ? '#ff5555' : '#50fa7b';
    entry.textContent = `[${new Date().toISOString()}] ${message}`;
    this.logElement.prepend(entry);
  }

  private truncateHash(hash: Uint8Array, length = 8): string {
    return Array.from(hash.slice(0, length))
      .map(b => b.toString(16).padStart(2, '0')).join('');
  }
}

new DebugInterface(); 