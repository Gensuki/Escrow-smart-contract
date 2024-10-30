#   Smart contract for escrowing cw721 & cw20

A Rust-based smart contract on the Sei blockchain that functions as an escrow for managing assets in raffle applications. This contract supports native tokens, CW20 tokens, and CW721 NFTs, enabling secure storage and distribution for raffle events. It is designed to allow authorized raffle operators to handle funds and distribute winnings efficiently.

## Multi-Asset Escrow

Native Tokens: Accepts and securely holds Sei's native tokens.
CW20 Tokens: Supports escrow for CW20-compliant tokens.
CW721 NFTs: Allows storage of CW721 NFTs for raffles or other events.

## Raffle and Distribution Management
Bulk Distributions: Optimized for gas-efficient bulk transactions:
Bulk Native Send: Distribute native tokens to multiple winners in one call.
Bulk CW20 Send: Send CW20 tokens to multiple addresses at once.
Bulk CW721 Send: Transfer NFTs in batch, allowing winners to claim their prizes seamlessly.

## Access Control
Raffle Operator: Only the raffle smart contract operator can call bulk send functions, ensuring authorized handling of escrowed assets.
Escrow Security: Ensures funds are securely held until predefined conditions, like raffle completion, are met.

## Contract Functions

Deposit:

Allows raffle hosts or authorized users to deposit native tokens, CW20 tokens, or CW721 NFTs into the escrow account.
Withdraw:

Provides a secure withdrawal method for raffle hosts in case of raffle cancellation or other valid conditions.
Bulk Distribution:

Bulk Distribute Native: Sends native tokens to multiple recipients in one transaction.
Bulk Distribute CW20: Transfers CW20 tokens to multiple winners simultaneously.
Bulk Distribute CW721: Batch transfers NFTs to raffle winners.

#Usage
Setup
Install Rust and Sei Dependencies:

Ensure you have Rust installed. Follow the Rust installation guide.
Install Sei CLI and any required dependencies.
Clone the Repository:

bash
Copy code
`git clone https://github.com/yourusername/sei-escrow-raffle.git
cd sei-escrow-raffle`
Build the Contract:

bash
Copy code
`cargo build --release`
Deploy:

Deploy the contract to Sei following the Sei deployment guide.
Example Calls
Deposit Tokens:

json
Copy code
`{
  "deposit": {
    "amount": "1000000",
    "denom": "usei"
  }
}`
Bulk Distribute Native:

json
Copy code
`{
  "bulk_distribute_native": {
    "recipients": [
      {"address": "sei1...", "amount": "500000"},
      {"address": "sei1...", "amount": "500000"}
    ]
  }
}`
Bulk Distribute CW20 Tokens:

json
Copy code
`{
  "bulk_distribute_cw20": {
    "contract_addr": "sei1...",
    "recipients": [
      {"address": "sei1...", "amount": "100"},
      {"address": "sei1...", "amount": "200"}
    ]
  }
}`
Bulk Distribute CW721 NFTs:

json
Copy code
`{
  "transferNfts": {
    "contract_addr": "sei1...",
    "recipients": [
      {"address": "sei1...", "token_id": "nft123"},
      {"address": "sei1...", "token_id": "nft456"}
    ]
  }
}`

## Sei Atlantic contract Address

`https://www.seiscan.app/atlantic-2/contracts/sei1m4ex7c7mlny7azcffgng5gy8ehxtrpwet479ve4t03ux46sw60zsd70cnj`

Security and Compliance
Access Control: Only the raffle operator can call distribution functions.
Asset Tracking: Deposits are tracked by type, ensuring they are securely held until distribution conditions are met.
Sei Standards: Fully compliant with Sei’s best practices for smart contract security and efficiency.
License
This project is licensed under the MIT License. See the LICENSE file for details.
