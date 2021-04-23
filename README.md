[中文](https://github.com/patractlabs/ZoPatract/blob/master/README_zh.md)

# ZoPatract

ZoPatract is a toolbox adapted to ink contract for zkSNARKs on jupiter. It is a side project for [zkMega](https://github.com/patractlabs/zkmega)

A complete zero-knowledge application is mainly composed of On-chain verification calculation and Off-chain verification calculation. Without the help of the auxiliary toolbox and deep professional domain knowledge and skills, the off-chain part of the program is difficult to be mastered and widely used by ordinary developers.

Therefore, in order to further reduce the threshold and cost of developing ink! zero-knowledge applications, we will build an off-chain cryptography toolbox in the next v0.2 version to help developers use high-level languages to generate Off-chain computable proofs, and Verify the proof in the ink! environment of On-chain. Connect On-chain and Off-chain to create a closed-loop zero-knowledge application development ecosystem for developers.

[ZoKrates](https://github.com/Zokrates/ZoKrates) is a toolbox on Ethereum that supports zkSNARKs application construction. It helps developers generate computable proofs using high-level languages and verify the proofs in the Solidity environment. The ZoKrates community is active, with many developers, and iterative upgrades are fast. In addition to the following advantages:

- Simple and easy-to-use high-level programming language and reusable standard library (including Hasher, Elliptic curve cryptography, Multiplexer, etc.)
- Powerful basic functions (supported Curves are ALT_BN128, BLS12_381, BLS12_377, BW6_761, Schemes support G16, GM17, PGHR13, Backends support Bellman, Libsnark, Arkworks)
- Complete development components (Javascript toolkit)
- Complete documentation and rich use cases


Therefore, we will transplant and transform the toolbox based on ZoKrates to create ZoPatract that is compatible with the Ink! smart contract environment. Achieve the following main goals :

- Make ZoPatract's Arkworks Proving schemes support G16, and Curves support bls12_381, bn254 (aligned to v0.1)

- Enable ZoPatract to support the complete commands of the zkSNARKs protocol
- Provide ZoPatract Javascript toolkit
- Provide ZoPatract documentation and sample programs

In the future, Patract will integrate ZoPatract into Online IDE products through plug-ins, providing a lighter development environment.

_This is a proof-of-concept implementation. It has not been tested for production.

For detailed report, please check out [Report](https://github.com/patractlabs/ZoPatract/blob/master/REPORT.md)

## License

ZoPatract is released under the GNU Lesser General Public License v3.

## Contributing

We happily welcome contributions. You can either pick an existing issue to resolve.

Unless you explicitly state otherwise, any contribution you intentionally submit for inclusion in the work shall be licensed as above, without any additional terms or conditions.


