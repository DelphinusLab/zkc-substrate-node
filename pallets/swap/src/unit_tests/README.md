The input directory is data for unit test.
Content of json file is created in zkp repo which is also used to generate input.json for circom. The input.json is valid because unit_run_full.sh in zkp/circom/tools can be ran successfully in zkp repo. So the data in input directory is valid.

## How to run unit test

```
cd zkp/
git checkout -b akayi/testNFT
npx tsc
node dist/tests/genInput.js
cd ../substrate-node/pallets/swap
cargo test -- --nocapture

```
## How to add unit test
Add function in substrate-node/pallets/swap/tests.rs and zkp/tests/genInput.ts.
Please ensure the functions in both repo execute the same command. For example, If we want to test deposit, we should execute setkey for accountIndex 0, 1 and 2 in substrate-node at first. The funtion in zkp must include them.
