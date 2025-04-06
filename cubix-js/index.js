/*

Cubic blockchain system

1. Initiate
create pools. 

2. Identity
3. Transaction
4. Construction
5. Verification

*/

import { sha256 } from 'js-sha256'

let tx1 = {
    timestamp: Date.now(),
    from: [],
    to: [],
    meta: {
        type: 'genesis',
        parameters: {
            txtypes: ['genesis', 'asset', 'identity', 'state'],
        }
        
    }
}

let stacks = [{
    blocks: [],
    faces: [[]],
    cubes: [[]],
}]
let gentxs = (txcount) => {
    let txs = []
    for (let i = 0; i < txcount; i++) {
        txs.push({
            timestamp: Date.now(),
            from: [Math.random().toString(36).substring(2, 15)],
            to: [Math.random().toString(36).substring(2, 15)],
            meta: {
                type: 'asset',
                parameters: {
                    txtypes: ['genesis', 'asset', 'identity', 'state'],
                }
                
            }
        })
    }
    return txs
}
let addtx = (txs) => {
    txs.forEach(tx => {
        stacks[0].blocks.push(tx)
    })
}
let numericalroot = (hash, type) => {
    const number = parseInt(hash, 16);
    return type === 'mod3' ? number % 3 : number % 9 === 0 ? 9 : number % 9
}
addtx(gentxs(200))
console.log(stacks[0].blocks.length)

let struct = (order) => {
    stacks[order].blocks.map((tx, txindex) => {
        let hash = sha256(JSON.stringify(tx))
        let dr = numericalroot(hash)
        // stacks[0].faces[0].find(dr)
        const indexToCheck = dr - 1; // Adjust index to be 0-based
    
        const foundIndex = stacks[order].faces.findIndex(face => face.length <= indexToCheck || face[indexToCheck] === undefined);
    
        if (foundIndex !== -1) {
            stacks[order].faces[foundIndex][indexToCheck] = hash;
        } else {
            const newFace = [];
            newFace[indexToCheck] = hash;
            stacks[order].faces.push(newFace);
        }
    })
    stacks[order].blocks=[]
    let filledFaces = [];
    stacks[order].faces = stacks[order].faces.filter(face => {
        if (face.length >= 9 && face.slice(0, 9).every(slot => slot !== undefined)) {
            filledFaces.push(face.slice(0, 9));
            return false; // Remove from faces
        }
        return true; // Keep in faces
    });
    stacks[order].cubes.push(...filledFaces.map((face, faceindex) => {
        let facehash = sha256(JSON.stringify(face))

        const indexToCheck = numericalroot(facehash, 'mod3'); // Adjust index to be 0-based
    
        const foundIndex = stacks[order].cubes.findIndex(cube => cube.length <= indexToCheck || cube[indexToCheck] === undefined);
    
        if (foundIndex !== -1) {
            stacks[order].cubes[foundIndex][indexToCheck] = facehash;
        } else {
            const newFace = [];
            newFace[indexToCheck] = hash;
            stacks[order].faces.push(newFace);
        }
    }));

    
}
struct(0)



console.log(stacks[0])



