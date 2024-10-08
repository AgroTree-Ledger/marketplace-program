# Agrotree Ledger - Marketplace Program

## Key Features

- List a Tree NFT on marketplace for sale

- Buy a Tree NFT from marketplace

- Unlist a Tree NFT from marketplace


## How to use

1. Install dependencies:

- Install Rust: https://www.rust-lang.org/tools/install

- Install Solana: https://docs.solana.com/cli/install-solana-cli-tools

- Install Node.js: https://nodejs.org/en/download/

- Install Anchor: https://www.anchor-lang.com/

2. Clone the repository:

3. Build the program:

```bash
anchor build
```

4. Run tests:

```bash 
anchor test
```

5. Clean and redeploy the program:

```
anchor keys list
solana program close 7bpcNeTm68aQv7C2cnFZYoqtWQU5UMSNKcnwTPanhyTZ --bypass-warning
rm -rf target/deploy/agrotree_marketplace-keypair.json
anchor build
anchor keys sync
anchor build
anchor deploy --provider.cluster devnet
```
