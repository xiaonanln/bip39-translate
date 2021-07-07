# bip39-translate

A BIP39 ENG-CHS translator written in Rust.

## Usage

This is used to transate BIP39 mnemonic code from English to Chinese and Chinese to English.

```bash 
$ cargo run jar gown call crew direct legal diesel pool future keen radio ketchup
赵 孩 队 半
官 枪 听 锋
待 炉 鬼 弱

$ cargo run 赵 孩 队 半 官 枪 听 锋 待 炉 鬼 弱
jar gown call crew
direct legal diesel pool
future keen radio ketchup
```