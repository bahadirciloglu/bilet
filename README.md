
# NEAR Smart Contract for Event Tickets Project Overview

The NEAR Smart Contract for Event Tickets allows decentralized event creation and ticket management on the NEAR blockchain. This project utilizes the power of blockchain technology to mint non-fungible tickets (NFTs) that are securely linked to specific events, preventing forgery and ensuring transparent ownership.

## Key Features

### Decentralized Event Management
Organizers can create events, define available tickets, and manage them transparently.

### NFT-Based Tickets
Each ticket is a unique NFT (Non-Fungible Token) that represents ownership and is securely stored on the NEAR blockchain.

### Forgery Protection
By leveraging the immutable nature of blockchain, ticket counterfeiting is eliminated.

### Ownership Verification
Event participants can easily verify their ticket ownership through the blockchain.

This project provides a foundation for building decentralized event systems, where users can purchase, sell, or transfer event tickets securely without intermediaries. The smart contract handles event creation, ticket minting, and ownership verification through NEAR's blockchain infrastructure, ensuring high scalability and low transaction costs.

## Features

### Mint Tickets for Specific Events
This project allows event organizers to create events on the NEAR blockchain and mint unique NFT (Non-Fungible Token) tickets for each event. Unlike traditional digital tickets, these NFT tickets are immutable and resistant to counterfeiting. For each event, a set number of tickets is minted, and users can purchase these available tickets.

### Verify Ticket Ownership via Blockchain
Using blockchain technology, ticket ownership can be easily verified. Each ticket is linked to the owner's NEAR wallet, ensuring authenticity and preventing fraudulent tickets. Event organizers can verify ticket ownership on the blockchain at the entrance, allowing only valid ticket holders to attend the event.

## Installation Guide

### Install the NEAR CLI
Clone the repository and install the NEAR CLI:
##
    npm install -g near-cli


The NEAR Command Line Interface (CLI) is used to interact with the NEAR blockchain. Install it globally using npm:
Login to NEAR CLI:
Authenticate with your NEAR account to interact with the blockchain:

##
    near login
        
Use code with caution.

This will open a browser window asking you to authorize the CLI to use your NEAR account.

Clone the Repository:

Clone this GitHub repository to your local machine:

##
    git clone https://github.com/bahadirciloglu/bilet.git
    cd bilet

Use code with caution.

Build the Smart Contract:

Use cargo (for Rust-based contracts) to build the smart contract:

##
    cargo build --target wasm32-unknown-unknown --release
        
Use code with caution.

The compiled contract will be located in the target/wasm32-unknown-unknown/release directory.

Deploy the Smart Contract:

Deploy the smart contract to your NEAR account:

##
    near deploy --accountId YOUR_ACCOUNT --wasmFile ./target/wasm32-unknown-unknown/release/event_contract.wasm

Use code with caution.

Replace YOUR_ACCOUNT with your NEAR testnet account ID.

Initialize the Contract (Optional):

If your smart contract requires initialization, you can call an initialization function:

##
    near call YOUR_ACCOUNT init '{}' --accountId YOUR_ACCOUNT
        
Use code with caution.

Run Tests (Optional):

To ensure everything is working as expected, run the tests:

##
    cargo test
        
Use code with caution.

About

Near Protocol Bilet
Resources
Readme
License (GPL-3.0 license)
Activity

Stars: 0
Watchers: 1
Forks: 0
Releases

No releases published.

Packages

No packages published.

Contributors

encody (Jacob Lindahl)
bahadirciloglu (Bahadir Ciloglu)
Languages

Rust (100.0%)






