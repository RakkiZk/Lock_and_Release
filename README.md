# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.




CtkLqsoKpZVjI9LARGuVWbzsebAZj9Up.A13idbcUCw9XYnTu
https://soroban-rpc.mainnet.stellar.gateway.fm	
https://rpc.eu-central-1.gateway.fm/v4/soroban/non-archival/mainnet

stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet

  stellar network add \
  --global mainnet2 \
  --rpc-url https://soroban-rpc.mainnet.stellar.gateway.fm \
  --network-passphrase "Public Global Stellar Network ; September 2015"



deploy
~/s/soroban-hello-world ❯ stellar keys address alice            18s 12:59:45 PM
GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76
~/s/soroban-hello-world ❯

~/s/soroban-hello-world ❯ stellar keys show alice                   12:59:05 PM
SBPXVV5EN4N4UXNTG5UIN7JSL637TU7FUJ7HL4BZ7KV2NYUDAPHHJY4K

my frighter wallet - 
GB7PT7AHQCYWA6RWXRGEX66DQSOFJZO45O3VPDZXLCUFZPHH7CZPCUNU

1. cargo clean

2. stellar contract build

3. cargo build --target wasm32-unknown-unknown --release

4. 
~/s/soroban-hello-world ❯ stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet

  stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network mainnet2

CBTWKH2SZKZAT3AWASDL4GPMGLA4IVGO6ALNVAXERC57GWLQKVW426T7   [Mainnet2]

5. 
stellar contract invoke \
    --id CBTWKH2SZKZAT3AWASDL4GPMGLA4IVGO6ALNVAXERC57GWLQKVW426T7 \
    --source alice \
    --network mainnet2 \
    -- initialize \
    --owner GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76

6. 
    stellar contract invoke \
    --id CA4XJSKDUZZ7AVDL6L6PBD6VTP2T7YQD55YNQ7GI2ZOKL2DJZD6G2Q57 \
    --source alice \
    --network testnet \
    -- set_admin \
    --admin GDHHGGRA7DM7CLXXS2U6SPHKHO45J2Z7Y6LWA57XNWIAZPHB5G6Q23PI

7. lock
CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75
CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA

    stellar contract invoke \
    --id CA4XJSKDUZZ7AVDL6L6PBD6VTP2T7YQD55YNQ7GI2ZOKL2DJZD6G2Q57 \
    --source alice \
    --network testnet \
    -- lock \
    --user_address GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76 \
    --from_token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC\
    --dest_token CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA \
    --in_amount 3 \
    --dest_chain 657468 \
    --recipient_address GDU66QMFGHZ74N4D5ZRSCEL4DBD3O6MZQ536ACH6MRGPHJSAWLXS3G3H



    stellar contract invoke \
    --id CA4XJSKDUZZ7AVDL6L6PBD6VTP2T7YQD55YNQ7GI2ZOKL2DJZD6G2Q57 \
    --source-account SBMN2JZ42YBJYQ4YV5ANVEQXJXUYLU2YGCQUGDFR4SS444SFMPMD45QP \
    --network testnet \
    -- release \
    --amount 4950000 \
    --user GCKKVKDAZDDIPNYF4L3YI7M7Q7I37M3B2NLX3UDON5CLXJYQEXTTEAIA \
    --destination_token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

    __________________


    stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source-account SCHKQXATI7TPUZN5XQUXEZYNIWV6RZF7OJNZ2OPCXRPNYBDTJ6LA6ILA \
  --network testnet


  Mainnet deployment

1. cargo clean

2. adding node with name mainnet3

~/stellar/soroban-hello-world ❯ stellar network add \                                 12:27:49 PM
  --global mainnet3 \
  --rpc-url https://soroban-rpc.mainnet.stellar.gateway.fm \
  --network-passphrase "Public Global Stellar Network ; September 2015"
~/stellar/soroban-hello-world ❯   

3. OWNER DETAILS

stellar keys show alice                                                                                                         
SBPXVV5EN4N4UXNTG5UIN7JSL637TU7FUJ7HL4BZ7KV2NYUDAPHHJY4K

stellar keys address alice                                                                                                      
GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76

4. stellar contract build

5. cargo build --target wasm32-unknown-unknown --release

6. 
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/lock_release.wasm \
  --source alice \
  --network mainnet3

  CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC

7.  initialize and provide owner address, he has permission to remove admin
stellar contract invoke \
    --id CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC \
    --source alice \
    --network mainnet3 \
    -- initialize \
    --owner GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76

8. set admin
admin details
SCHKQXATI7TPUZN5XQUXEZYNIWV6RZF7OJNZ2OPCXRPNYBDTJ6LA6ILA

stellar contract invoke \
    --id CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC \
    --source alice \
    --network mainnet3 \
    -- set_admin \
    --admin GBJPLNOYJPHTPEOCNOTCHQND4DAPB5TAZQGD6RNIDZNRTN2YUH4BZXGS

9. LOCK, for cli testing using xlm

stellar contract invoke \
    --id CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC \
    --source alice \
    --network mainnet3 \
    -- lock \
    --user_address GDA7TLDVQUAELH2WWQMYPB7M6MAMYBOOW7VB7VLPM3ECRJVNNCLGGA76 \
    --from_token CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA\
    --dest_token CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA \
    --in_amount 5 \
    --dest_chain 657468 \
    --recipient_address GDU66QMFGHZ74N4D5ZRSCEL4DBD3O6MZQ536ACH6MRGPHJSAWLXS3G3H

10. release

stellar contract invoke \
    --id CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC \
    --source-account SCHKQXATI7TPUZN5XQUXEZYNIWV6RZF7OJNZ2OPCXRPNYBDTJ6LA6ILA \
    --network mainnet3 \
    -- release \
    --amount 2 \
    --user GDVLK4OOH3UTG74AYVKAI4IVYRHWFR4LYEIPCHQFUN5H7UPEKDW56GZB \
    --destination_token CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA

    stellar contract invoke \
    --id CAZB2F2GXXDTDZHY55K736BIEJVUYV6UYMKPZUW2VTSIJI3JL4NV42FC \
    --source-account SCHKQXATI7TPUZN5XQUXEZYNIWV6RZF7OJNZ2OPCXRPNYBDTJ6LA6ILA \
    --network mainnet3 \
    -- release \
    --amount  \
    --user GDVLK4OOH3UTG74AYVKAI4IVYRHWFR4LYEIPCHQFUN5H7UPEKDW56GZB \
    --destination_token CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75