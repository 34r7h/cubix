impl Circuit<bls12_381::Scalar> for StakeCircuit {
    fn configure(meta: &mut ConstraintSystem<bls12_381::Scalar>) -> Self::Config {
        let stake = meta.advice_column();
        let minimum = meta.fixed_column();
        
        meta.create_gate("Stake >= Minimum", |vc| {
            let s = vc.query_advice(stake, Rotation::cur());
            let m = vc.query_fixed(minimum, Rotation::cur());
            vec![s - m]
        });
        // ...
    }
} 