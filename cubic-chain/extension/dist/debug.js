import init, { CubicKey, generate_proof, NetworkState } from 'cubic-chain';
class DebugInterface {
    constructor() {
        this.wasmReady = false;
        this.initWasm().then(() => this.startMonitoring());
        this.logElement = document.getElementById('debug-log');
    }
    async initWasm() {
        try {
            await init();
            this.wasmReady = true;
            this.log('WASM module initialized');
        }
        catch (err) {
            this.log(`WASM Error: ${err.message}`, 'error');
        }
    }
    startMonitoring() {
        setInterval(() => this.updateNetworkStatus(), 2000);
        this.initKeyRotation();
        this.startCubeBuilder();
    }
    async updateNetworkStatus() {
        if (!this.wasmReady)
            return;
        const state = NetworkState.current();
        document.getElementById('peer-count').textContent =
            state.peers.length.toString();
        document.getElementById('block-height').textContent =
            state.chainHeight.toString();
    }
    initKeyRotation() {
        const seed = crypto.getRandomValues(new Uint8Array(32));
        const key = new CubicKey(seed);
        setInterval(() => {
            const nextKey = key.next_key();
            document.getElementById('next-address').textContent =
                this.truncateHash(nextKey);
        }, 5000);
    }
    startCubeBuilder() {
        setInterval(async () => {
            const cubeProgress = document.getElementById('cube-progress');
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
    log(message, type = 'info') {
        const entry = document.createElement('div');
        entry.style.color = type === 'error' ? '#ff5555' : '#50fa7b';
        entry.textContent = `[${new Date().toISOString()}] ${message}`;
        this.logElement.prepend(entry);
    }
    truncateHash(hash, length = 8) {
        return Array.from(hash.slice(0, length))
            .map(b => b.toString(16).padStart(2, '0')).join('');
    }
}
new DebugInterface();
