# npm dependencies

## install tests dependencies

in tests/:

- npm init -y
- npm i --save-dev hardhat @nomicfoundation/hardhat-foundry @nomicfoundation/hardhat-toolbox dotenv

## to test

to use:

in tests/basic_mapping_slots:

in one terminal: `anvil`
in an other one: `npx hardhat run scripts/test.js`