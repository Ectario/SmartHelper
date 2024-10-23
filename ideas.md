# Ideas

- Create a visual map of where are mapped variable in which slot

- Take a full solidity script and extract the prototype of each function to get an output like:

```sol
    interface IBadMechSuit {
        function upgradeTo(uint8 mode) external;
        function shootTrustyRockets(uint128 x, uint128 y) external view returns (bytes32);
        function swingSword() external view returns (bytes32);
    }
```
