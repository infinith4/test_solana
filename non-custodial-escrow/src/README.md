anchor init noncustodialescrow



cargo clean

rm /home/solanauser/.config/solana/id.json

anchor build


solana-keygen new -o /home/solanauser/.config/solana/id.json

pepper reward boss rose copper bring element fire cool camp under mule

anchor keys list

```
solanauser ➜ /src $ anchor keys list
self_custodial_facebook: CEzPoUAPLvwMHhADUjaxxyPXvfY7xCvsxNwsQdPEbY2y
```

lib.rs の declare_id! macro に書く

Anchor.toml のprovider.cluster の値を devnet に変更する。

```
[provider]
cluster = "Localnet"
```


solana airdrop 3 FdLadAHvHEAYjmCQJWNi7niGQG3XQFHyRqbs7FHEdEZ --url https://api.devnet.solana.com

solana balance FdLadAHvHEAYjmCQJWNi7niGQG3XQFHyRqbs7FHEdEZ --url https://api.devnet.solana.com


solanauser ➜ /src $ solana-keygen new -o /home/solanauser/.config/solana/id.json                             
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none): 

Wrote new keypair to /home/solanauser/.config/solana/id.json
====================================================================================
pubkey: FdLadAHvHEAYjmCQJWNi7niGQG3XQFHyRqbs7FHEdEZ
====================================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
bean ginger marriage away evidence sudden like citizen diagram program journey alter
====================================================================================

anchor deploy



```
solanauser ➜ /src $ anchor deploy
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/solanauser/.config/solana/id.json
Deploying program "self_custodial_facebook"...
Program path: /src/target/deploy/self_custodial_facebook.so...
Program Id: Fco3EAgPY7TEh7rCdacgvovSRv6Na89JvjAAoTkD8Knu

Deploy success
```

https://explorer.solana.com/address/Fco3EAgPY7TEh7rCdacgvovSRv6Na89JvjAAoTkD8Knu?cluster=devnet
https://explorer.solana.com/tx/5C4odbVwSPDqqi2fQKKGMdWDpKpL8EvTgt1QPNqv9FZbpYBV9iNbxEmrhHgTnkY5e3Lwndh5g1PRvboJ3oGA3hov?cluster=devnet



anchor test

solanauser ➜ /src $ anchor test     
   Compiling noncustodialescrow v0.1.0 (/src/programs/noncustodialescrow)
    Finished release [optimized] target(s) in 4.84s
   Compiling noncustodialescrow v0.1.0 (/src/programs/noncustodialescrow)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.60s
     Running unittests src/lib.rs (/src/target/debug/deps/self_custodial_facebook-50d4a1e4d9f9ce67)
Deploying cluster: https://api.devnet.solana.com
Upgrade authority: /home/solanauser/.config/solana/id.json
Deploying program "self_custodial_facebook"...
Program path: /src/target/deploy/self_custodial_facebook.so...
Program Id: EQ4UQF6WMf52Cmm4a4v9iE2y4EGYaQSNFt39vjrymiNP

Deploy success

Found a 'test' script in the Anchor.toml. Running it as a test suite!

Running test suite: "/src/Anchor.toml"

yarn run v1.22.22
warning package.json: No license field
$ /src/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'


  noncustodialescrow
User facebook address ::  BRwenrUbXYhAMutXSm4tQxtEHL3GsDDqXXsK1WEm2XSV
Your transaction signature 5f8J2ZBzeQxz8uYKywTrQzGHDJLAUstgmRGwYzcnXMrAieshBrmT2dSJQCpv9gm9tejAUy48RomJAQ5gQHZkiSTE
Created a new account with following details 
 Name :: Deep 
 Status :: always tinkring 
 Twitter :: 0xdeep
    ✔ Creating a new account for user (1329ms)
usrFaceBook Address ::  BRwenrUbXYhAMutXSm4tQxtEHL3GsDDqXXsK1WEm2XSV
Your transaction signature 5NYjyYsawsm9e9yHZjjiRrFAZB5xp7aqPks338KyY8Q1jqxyV2fUKWChZCx3hvfTY8DXLR131yNTGoqw1BiUN47g
Created a new account with following details 
 Name :: Deep 
 Status :: &mut self :crab 
 Twitter :: 0xdeep
    ✔ Update My Status (995ms)
Users account does not exist ::  Error: Account does not exist or has no data iKZN8HeC17UP3X8RFYDvt7iyEFRff2bMqR8pebBM8sw
    at AccountClient.fetch (/src/node_modules/@coral-xyz/anchor/src/program/namespace/account.ts:168:13)
    at processTicksAndRejections (node:internal/process/task_queues:95:5)
    ✔ Find Someone's Facebook (152ms)
user facebook address ::  BRwenrUbXYhAMutXSm4tQxtEHL3GsDDqXXsK1WEm2XSV
Your transaction signature 2yNiLZDS9Mf5i1gH3fiwFhS1aWr95Q4HkttRc6gi5Tt44qotrNNV63TdHedZTGpGnuYyrJc3Aa6Yhw1UXaoh7RdA
User Details Not found, 'cuz we close the account
    ✔ Close My Facebook Account (1486ms)


  4 passing (4s)

Done in 6.14s.


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
