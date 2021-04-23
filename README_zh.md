# ZoPatract

ZoPatract 是为Ink!适配zkSNARKs的密码学工具箱.

一个完整的零知识应用主要由On-chain的验证计算和Off-chain的证明计算组成．如果没有辅助工具箱的帮助和深厚的专业领域知识技能，Off-chain部分的程序很难被普通开发者掌握和广泛使用．

因此，为了进一步降低开发ink!零知识应用的门槛和成本，我们打造了链下密码学工具箱ZoPatract，帮助开发者使用高级语言生成Off-chain可计算证明，并在On-chain的ink!环境中验证证明．连接On-chain和Off-chain，为开发者打造闭环的零知识应用开发生态.

[ZoKrates](https://github.com/Zokrates/ZoKrates) 是以太坊上的支持zkSNARKs应用构建的工具箱。它帮助开发者使用高级语言生成可计算证明，并在Solidity环境中验证证明。ZoKrates社区活跃，开发者众多，迭代升级较快，除此之外还有以下优点：
- 简单易用的高级程序语言和可重用的标准库(包括Hasher、Elliptic curve cryptography、Multiplexer等等)
- 基础功能强大(支持的Curves有ALT_BN128、BLS12_381、BLS12_377、BW6_761,Schemes支持G16、GM17、PGHR13,Backends支持Bellman、Libsnark、Arkworks)
- 开发组件完善(Javascript工具包)
- 文档齐全,用例丰富


因此，我们将基于ZoKrates移植改造该工具箱，打造适配Ink!智能合约环境的*ZoPatract*。实现了以下主要目标:
- 使ZoPatract的Arkworks的Proving schemes支持G16,Curves支持bls12_381、bn254(对齐v0.1)
- 使ZoPatract支持zkSNARKs协议的完整命令
- 提供ZoPatract的Javascript工具包
- 提供ZoPatract文档与示例程序

在未来，Patract Hub将通过插件的方式为ZoPatract集成到Online IDE产品中，提供更轻便的开发环境．

注意:该产品仍处于概念验证阶段,并未在生产中充分验证.

## How To Use
请查看[设计实现说明和使用参考](https://github.com/patractlabs/ZoPatract/blob/master/REPORT_zh.md).

## License

GNU Lesser General Public License v3.

## Contributing

我们很高兴欢迎您的贡献。 您可以选择一个现有的问题来解决。

除非您明确声明，否则，您有意提交供包含在作品中的任何贡献均应按上述许可，而无需任何其他条款或条件。

