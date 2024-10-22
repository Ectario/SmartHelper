const { ethers } = require("hardhat");

async function main() {
    const contractFactory = await ethers.getContractFactory("City");
    const cityContract = await contractFactory.deploy();
    const address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";


    const result_old = await cityContract.owner_address();
    console.log("Old owner address:", result_old);
    await cityContract.write(1, "0x000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266");

    const result = await cityContract.owner_address();
    console.log("New owner address:", result);


}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});