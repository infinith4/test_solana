https://examples.anchor-lang.com/docs/hello-world


cargo clean

rm /home/solanauser/.config/solana/id.json


anchor build

solana-keygen new -o /home/solanauser/.config/solana/id.json




solana airdrop 2 3942fWciFWTwiBdkK6to71m1X9PczbaViZdpZESwGGcD --url https://api.devnet.solana.com

solanauser ➜ /src/hello-world $ solana airdrop 2 3942fWciFWTwiBdkK6to71m1X9PczbaViZdpZESwGGcD --url https://api.devnet.solana.com
Requesting airdrop of 2 SOL

Signature: 4DZtLj8BKpPhi5aUUE9suRxc14PCjzd8BtpZdPW7HVMuCrdyfuyYzDMqnvj7tK479rjLN6eqkYCAimsCa3SXd5Fh

2 SOL

```
solanauser ➜ /src/hello-world $ anchor keys list                                                                                 
hello_world: 4CHruUMrLBHFDWYgTXUZMAzRLMM1qZnynHC3Hxv1o4bR
```

をlib.rs, Anchor.toml に設定する

anchor test


anchor deploy --provider.cluster devnet

```
solanauser ➜ /src/hello-world $ anchor deploy --provider.cluster devnet                                                          
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/solanauser/.config/solana/id.json
Deploying program "hello_world"...
Program path: /src/hello-world/target/deploy/hello_world.so...
Program Id: HmmTDoAmbNzXgXVCcHVrndGAVNSg6Ka2QjbZHF2oGrE9

Deploy success
```

https://explorer.solana.com/tx/2ojq4hG1fUJqxw4t4qBkbW2WFPyxoj2FkVvTuTvguVFLCwFgxTfJdAanhgQjLpNpuP7p1Hsy6E4f2G7u9ZZn9goB?cluster=devnet#ix-1


anchor test

------

We are going to deploy on devnet. Here is our deployment checklist 🚀

Run anchor build. Your program keypair is now in target/deploy. Keep this keypair secret 🤫.
Run anchor keys list to display the keypair's public key and copy it into your declare_id! macro at the top of lib.rs.
Run anchor build again. This step is necessary to include the new program id in the binary.
Change the provider.cluster variable in Anchor.toml to devnet.
Run anchor deploy
Run anchor test



anchor test