// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

contract City2 {
    uint128 public nb_user;
    address public owner_address;
    mapping(uint8 => mapping(uint8 => address)) public home_map;
    bytes20[] public ids;
    bytes32[11] public things;
    bytes4[5] public other_things;
    address public guest_address;
    bytes20[][] public ids_2_dimensionnal;

    /// Write data to the contract's ith storage slot
    function write(uint256 i, bytes32 data) public {
        assembly {
            sstore(i, data)
        }
    }
}
