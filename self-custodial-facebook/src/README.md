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


solana airdrop 5 69fhtbhT2u51ixLeGB9ESgT7UaMbRvFU6CivK3GKU9V --url https://api.devnet.solana.com


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


Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/solanauser/.config/solana/id.json
Deploying program "onchainvoting"...
Program path: /src/target/deploy/onchainvoting.so...
Program Id: DS8Ccd3ZqQNvF3Ftsq8H1J9p8P7GVHgY4qxCS3hNGmb7

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/src/Anchor.toml"

yarn run v1.22.22
warning package.json: No license field
$ /src/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'


  onchainvoting
TxHash :: 4ga46A2y3Vc4DojgRMcxtw9WMDe4tZ6WNHq9tCrEYbjBA3sJFBcb1JkTQAydneuPo8K71FmVKUqreFQHJhFafbXz
    ✔ Creating vote bank for public to vote (1082ms)
TxHash :: 4z2tKpv27TsFXT1j1EqMCmHrmscV8n4SnQAQqgqS7gHs8TC8o9Sx7dau36VMWna5Lb8Jzq1x2Qe47qpjhfuqWfmm
Total GMs :: 1
Total GNs :: 0
    ✔ Vote for GM (1052ms)
TxHash :: 5a4sbN14dmqwdCyTbiVb7zoytbkvvKsJggGLnRRf6Y7eXPFQutgY8GNXVyer4f99PB2HNcDYxhhPPsz29G76YcRC
Total GMs :: 1
Total GNs :: 1
    ✔ Vote for GN (716ms)


  3 passing (3s)

Done in 5.21s.



https://explorer.solana.com/tx/4ga46A2y3Vc4DojgRMcxtw9WMDe4tZ6WNHq9tCrEYbjBA3sJFBcb1JkTQAydneuPo8K71FmVKUqreFQHJhFafbXz?cluster=devnet

https://explorer.solana.com/tx/4z2tKpv27TsFXT1j1EqMCmHrmscV8n4SnQAQqgqS7gHs8TC8o9Sx7dau36VMWna5Lb8Jzq1x2Qe47qpjhfuqWfmm?cluster=devnet

https://explorer.solana.com/tx/5a4sbN14dmqwdCyTbiVb7zoytbkvvKsJggGLnRRf6Y7eXPFQutgY8GNXVyer4f99PB2HNcDYxhhPPsz29G76YcRC?cluster=devnet


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



https://book.anchor-lang.com/anchor_in_depth/PDAs.html
https://solana.stackexchange.com/questions/8829/on-calling-ctx-bumps-getmint-account-failing-with-error-method-get-not-fo
