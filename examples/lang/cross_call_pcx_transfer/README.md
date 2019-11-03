# Delegate PCX Transfer Smart Contract

The `cross_call_pcx_transfer` smart contract is demonstrated to delegate a PCX transfer call to the other smart contract, which consists in two smart contracts:

1. `cross_call_pcx_transfer`: Delegate the call to the `pcx_transfer` smart contract.
2. `pcx_transfer`: Actually dispatch the PCX transfer call to the ChainX runtime.

In order to test `cross_call_pcx_transfer` you need to do the following:

1. Compile and deploy [the complete `pcx_transfer` contract](../pcx_transfer).
2. Compile and deploy the `cross_call_pcx_transfer` contract.
    - Note that you need to specify the contract address of `pcx_transfer` contract we deployed in the previous step when deploying the `cross_call_pcx_transfer` contract.
    - Put the source code of `pcx_transfer` contract in this crate with `ink-as-dependency` feature enabled, see [Cargo.toml](./Cargo.toml).
3. Now you can `delegate` the PCX transfer call via `cross_call_pcx_transfer` contract.
