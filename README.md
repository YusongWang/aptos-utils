# Aptos-utils
a bot for buy nft on bluemove

# Future
- batch generate account.
- transfer with contract. no need use wallet,batch 100 account in one tx.
- Wait mint started and auto mint for may account.

# TODO
- transfer nft to owner account .
- Or list nft on bluemove.

# System require.

Only ubuntu ^20.04 from released

# how to use ?

## 1. isntall


first all , you need install Rust cargo.
quick do that, you can use aptos-core setup.

follow

[build-aptos-cli](https://aptos.dev/cli-tools/build-aptos-cli#linux)


## 2. help



### 2.1 generate accounts 

use command.

```shell
aptos-utils gen 100
```

Well generate 100 keypair in execution dir. call keys.txt you can view that account and privatekey

### Transfer amount to all account with same amount.

first do that. you need publish a contract to you main wallet.

source code at (this)[https://github.com/YusongWang/aptos-utils/tree/main/batch-trasfer-move]

to use this. you need init you aptos account. with aptos cli

```shell
aptos init
```

and select mainnet, and push you main wallet privateKey.

Then change Move.Toml selation addresses.owner = "you main address start with 0x"

and In the same dir. run commond.

```shell
aptos move pushlic
```
you well success publish you first contract to mainnet..

### 2.2 batch transfer use aptos-utils

So you doing good. let's use commond trasfer a amount

```shell
PRIVATE_KEY="You main wallet privateKey" aptos-utils split 100 100000000 100000000 100
```
split accounts amount gaslimit gasprice.

accounts you want transfer to how may account.

amount eve account has amount balance

gas limit that tx MAX cost gas fee.

gas price just lavel 100.

if success you well see nothing.

### 2.3 batch buy.

TODO
