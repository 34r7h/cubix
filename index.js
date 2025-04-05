/*

Cubic blockchain system

1. Initiate
create pools. 

2. Identity
3. Transaction
4. Construction
5. Verification

*/

let txstack = []
let tx1 = {
    timestamp: Date.now(),
    from: [],
    to: [],
    meta: {
        type: 'genesis',
        parameters: {
            txtypes: ['asset', 'identity', 'state'],
            
        }
        
    }
}