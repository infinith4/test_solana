https://examples.anchor-lang.com/docs/onchain-voting

anchor init onchainvoting



cargo clean

rm /home/solanauser/.config/solana/id.json

anchor build


solana-keygen new -o /home/solanauser/.config/solana/id.json

anchor keys list

```
solanauser ➜ /src/onchain-voting $ anchor keys list
onchain_voting: 87XWM9o7B1MxeaXQM6A3jAS96hmcKZJPNyYLjfDMLJxt
```

lib.rs の declare_id! macro に書く

Anchor.toml のprovider.cluster の値を devnet に変更する。

```
[provider]
cluster = "Localnet"
```


solana airdrop 2 GuFHVKfgeQfZ1ZwtbHhrPM7vCaP2xXG4vnXLBkVZqZLw --url https://api.devnet.solana.com


anchor deploy



```
solanauser ➜ /src/onchain-voting $  anchor deploy
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/solanauser/.config/solana/id.json
Deploying program "onchain_voting"...
Program path: /src/onchain-voting/target/deploy/onchain_voting.so...
Program Id: 87XWM9o7B1MxeaXQM6A3jAS96hmcKZJPNyYLjfDMLJxt

Deploy success
```

https://explorer.solana.com/address/87XWM9o7B1MxeaXQM6A3jAS96hmcKZJPNyYLjfDMLJxt?cluster=devnet




anchor test




-----

Deployment 🎉
Time to deploy and test our first hello world smart contract, yay!

We are going to deploy on devnet. Here is our deployment checklist 🚀

Run anchor build. Your program keypair is now in target/deploy. Keep this keypair secret 🤫.
Run anchor keys list to display the keypair's public key and copy it into your declare_id! macro at the top of lib.rs.
Run anchor build again. This step is necessary to include the new program id in the binary.
Change the provider.cluster variable in Anchor.toml to devnet.
Run anchor deploy
Run anchor test

https://explorer.solana.com/tx/cZRfUaFshh4jyweT3ZAtVxcCatxSHujku76ar6NJRjGxyyFE8BX7PPs8ZXv4cEvVXQKz1TX7XEqhrmeDcqHRf1j?cluster=devnet#ix-1