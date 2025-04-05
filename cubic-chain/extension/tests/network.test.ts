test('broadcast tx to correct cube', async () => {
    const tx = new Transaction(...);
    await network.broadcastTx(tx);
    expect(network.getCube(tx.position)).toContain(tx);
}); 