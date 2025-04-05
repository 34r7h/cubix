# Cubix - Graypaper // v 1.0.1

Cubes XMBL: A Distributed Ledger System with Geometric Cryptography


34r7h
34r7h@proton.mail

Abstract: A protocol for a decentralized ledger that structures transactions within a growing multi-dimensional cube. The placement of transactions and the derivation of a cryptographic protocol are based on the hashes of the transactions and a set of deterministic heuristics. The goal is to enable Verkle-like commitments and proofs for efficient verification.

## 1. Introduction


## 2. Ledger Construction

**Block**: The fundamental unit. Initially a 1x1x1 spatial volume representing a single transaction. Subsequent "blocks" can be cubes of transactions/cubes of transactions.

**Face**: A 3x3 grid of 9 blocks.

**Cube**: Composed of 3 faces.

#### Block Placement in Face: 
For a given block ID, its placement within a face (3x3 grid, top-left to bottom-right indexed 1-9) is determined by the digital root of the block ID.

#### Face Placement in Cube: 
For a given block ID, the face's placement within a cube (3 faces, indexed 0-2) is determined by the block ID modulo 3.

**Growing Super-Structure**: The system starts with a 1x1x1 block, followed by a 3x3x3 cube, then a 9x9x9 super-cube, and so forth, with the side length of the cube growing as powers of 3 (3⁰, 3¹, 3², ...). Partial cubes await the next entry required for completion.

## 3. Transaction Pools

A transaction is delivered to the network transaction pool with a network-determined stake and send timestamp. The transaction pool applies another timestamp and creates a block ID by averaging the timestamps and hashing with the TX's content. A digital root is then derived from the block ID for placement on a face, as per the construction rules above. When sending a TX to the pool, the sender includes a network-determined stake and validates a prior face or cube. A wallet implementation generates a proof and commitment The network provides a 


## 4. Consensus

When a face or cube is validated, all stakes are returned and that face/cube is hashed with the timestamp at validation. The internal data is able to be pruned from the ledger, updating only the record of token balances and associated commitments. 

Protocol: Hotstuff


## 5. Network Topography

STUN/TURN/ICE


## 6. Identity
https://en.wikipedia.org/wiki/Geometric_cryptography

The geometric cryptographic method for identity is based on the impossibility of trisecting an angle using ruler and compass. Given an arbitrary angle, there is a straightforward ruler and compass construction for finding the triple of the given angle. But there is no ruler and compass construction for finding the angle which is exact one-third of an arbitrary angle. Hence the function which assigns the triple of an angle to a given angle can be thought of as a one-way function, the only constructions allowed being ruler and compass constructions.

Assume that Alice wishes to establish a means of proving her identity later to Bob.

Initialization: Alice publishes a copy of an angle YA which is constructed by Alice as the triple of an angle XA she has constructed at random. Because trisecting an angle is impossible Alice is confident that she is the only one who knows XA.

Identification Protocol:

Alice gives Bob a copy of an angle R which she has constructed as the triple of an angle K that she has selected at random.
Bob flips a coin and tells Alice the result.
If Bob says "heads" Alice gives Bob a copy of the angle K and Bob checks that 3*K = R.
If Bob says "tails" Alice gives Bob a copy of the angle L = K + XA and Bob checks that 3*L = R + YA.
The four steps are repeated t times independently. Bob accepts Alice's proof of identity only if all t checks are successful.

This protocol is an interactive proof of knowledge of the angle XA (the identity of Alice) with error 2−t. The protocol is also zero-knowledge.

## 6. Commitments

The system is identity agnostic. and uses only 


## 7. State Machine

The state is a sidechain consisting of diffs


## 8. Maths

Lol.


## 9. Future Developments

> Tokens

> Primitives


## 10. Conclusions


References
[1] https://web.archive.org/web/20011116115341/http://theory.lcs.mit.edu/~rivest/BurmesterRivestShamir-geometric.pdf