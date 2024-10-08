import { AnchorProvider, BN, Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import PAYER from "/home/leo/.config/solana/id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import idl from "../target/idl/agrotree_marketplace.json";
import {
  getAssetWithProof,
  mplBubblegum,
} from "@metaplex-foundation/mpl-bubblegum";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { dasApi } from "@metaplex-foundation/digital-asset-standard-api";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { bufferToArray, craeteACnft, mapProof } from "./helper";
import { AgrotreeMarketplace } from "../target/types/agrotree_marketplace";
import dotenv from "dotenv";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
dotenv.config();

(async () => {
  const connection = new Connection(
    "https://devnet.helius-rpc.com/?api-key=07ab23dd-0e9b-4feb-bcd6-1ae9e0405db9",
    // "https://devnet-rpc.shyft.to/?api_key=037o0cpTSD8FBXv7",
    "confirmed"
  );

  const payer_wallet = Keypair.fromSecretKey(Uint8Array.from(PAYER));
  const buyer = Keypair.fromSecretKey(
    bs58.decode(process.env.BUYER_PRIVATE_KEY)
  );
  const provider = new AnchorProvider(connection, new Wallet(payer_wallet), {
    preflightCommitment: "confirmed",
  });

  const program = new Program(idl as AgrotreeMarketplace, provider);
  const FEE = 1000;

  const umi = createUmi(provider.connection.rpcEndpoint)
    .use(mplBubblegum())
    .use(mplTokenMetadata())
    .use(dasApi());
  const umiSigner = createSignerFromKeypair(
    umi,
    umi.eddsa.createKeypairFromSecretKey(Uint8Array.from(PAYER))
  );
  umi.use(signerIdentity(umiSigner));

  try {
    const rpcAssetList = await umi.rpc.getAssetsByOwner({
      owner: umiSigner.publicKey,
    });

    let assetId;
    if (rpcAssetList.items.length === 0) {
      const { assetId: _temp } = await craeteACnft(
        umi,
        "H8S1CPKVmx6PnQ61fFi6JH4eakpJ4A5F9rfS4ZZschFj"
      );
      // console.log({ asset });
      assetId = _temp;
    } else {
      const oneItem = rpcAssetList.items.filter(
        (item) => item.compression.tree
      );
      const randomItem = oneItem[Math.floor(Math.random() * oneItem.length)];
      // console.log({ oneItem });
      assetId = publicKey(randomItem.id);
    }

    try {
      const tx1 = await program.methods
        .initialize(FEE)
        .accounts({
          authority: provider.wallet.publicKey,
        })
        .rpc();

      console.log(`Init transaction: ${tx1}`);
    } catch (error) {
      // console.error({ error });
    }

    {
      const assetWithProof = await getAssetWithProof(umi, assetId);
      // console.log({ assetWithProof });

      const _mapRoof = mapProof({ proof: assetWithProof.proof });
      // console.log({ _mapRoof });

      const [newLeafOwner] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("c-listing"),
          new PublicKey(assetWithProof.merkleTree).toBuffer(),
          new PublicKey(assetId).toBuffer(),
        ],
        program.programId
      );

      console.log({
        listingAsset: assetId.toString(),
        newLeafOwner: newLeafOwner.toString(),
      });

      const tx2 = await program.methods
        .listingCnft({
          assetId: new PublicKey(assetId),
          price: new BN(10_000_000),
          cnftArgs: {
            root: bufferToArray(Buffer.from(assetWithProof.root)),
            creatorHash: bufferToArray(Buffer.from(assetWithProof.creatorHash)),
            dataHash: bufferToArray(Buffer.from(assetWithProof.dataHash)),
            index: assetWithProof.index,
            nonce: new BN(assetWithProof.nonce),
          },
        })
        .accounts({
          seller: provider.wallet.publicKey,
          leafOwner: provider.wallet.publicKey,
          leafDelegate: provider.wallet.publicKey,
          merkleTree: assetWithProof.merkleTree,
        })
        .remainingAccounts(_mapRoof)
        .rpc();

      console.log(`Listing transaction: ${tx2}`);
    }

    {
      // assetId = publicKey("CDvQqbFaridsM4etLGzcAqXypUkfxEvAasucg1keJ7LX");
      const assetWithProof = await getAssetWithProof(umi, assetId);

      const _mapRoof = mapProof({ proof: assetWithProof.proof });

      const [oldLeafOwner] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("c-listing"),
          new PublicKey(assetWithProof.merkleTree).toBuffer(),
          new PublicKey(assetId).toBuffer(),
        ],
        program.programId
      );

      console.log({
        buyingAsset: assetId.toString(),
        oldLeafOwner: oldLeafOwner.toString(),
      });

      const tx3 = await program.methods
        .buyCnft({
          assetId: new PublicKey(assetId),
          cnftArgs: {
            root: bufferToArray(Buffer.from(assetWithProof.root)),
            creatorHash: bufferToArray(Buffer.from(assetWithProof.creatorHash)),
            dataHash: bufferToArray(Buffer.from(assetWithProof.dataHash)),
            index: assetWithProof.index,
            nonce: new BN(assetWithProof.nonce),
          },
        })
        .accounts({
          buyer: buyer.publicKey,
          leafOwner: provider.wallet.publicKey,
          leafDelegate: provider.wallet.publicKey,
          merkleTree: assetWithProof.merkleTree,
          seller: provider.wallet.publicKey,
        })
        .remainingAccounts(_mapRoof)
        .signers([buyer])
        .rpc();

      console.log(`Buy transaction: ${tx3}`);
    }

    // delay 3s

    // {
    //   await new Promise((resolve) => setTimeout(resolve, 3000));
    //   const assetWithProof = await getAssetWithProof(umi, assetId);
    //   // console.log({ assetWithProof });

    //   const _mapRoof = mapProof({ proof: assetWithProof.proof });
    //   // console.log({ _mapRoof });

    //   const [oldLeafOwner] = PublicKey.findProgramAddressSync(
    //     [
    //       Buffer.from("c-listing"),
    //       new PublicKey(assetWithProof.merkleTree).toBuffer(),
    //       new PublicKey(assetId).toBuffer(),
    //     ],
    //     program.programId
    //   );

    //   console.log({
    //     unlistAsset: assetId.toString(),
    //     oldLeafOwner: oldLeafOwner.toString(),
    //   });

    //   const tx3 = await program.methods
    //     .unlistCnft({
    //       assetId: new PublicKey(assetId),
    //       cnftArgs: {
    //         root: bufferToArray(Buffer.from(assetWithProof.root)),
    //         creatorHash: bufferToArray(Buffer.from(assetWithProof.creatorHash)),
    //         dataHash: bufferToArray(Buffer.from(assetWithProof.dataHash)),
    //         index: assetWithProof.index,
    //         nonce: new BN(assetWithProof.nonce),
    //       },
    //     })
    //     .accounts({
    //       seller: provider.wallet.publicKey,
    //       leafOwner: provider.wallet.publicKey,
    //       leafDelegate: provider.wallet.publicKey,
    //       merkleTree: assetWithProof.merkleTree,
    //     })
    //     .remainingAccounts(_mapRoof)
    //     .rpc();

    //   console.log(`Unlist transaction: ${tx3}`);
    // }

    {
      const tx5 = await program.methods
        .collectFee()
        .accounts({
          destination: new PublicKey(
            "2zLzHr129EChjxt4HpFdBZuHbZKwNcoY6iCaF48vwCEU"
          ),
        })
        .rpc();

      console.log(`Collect Fee transaction: ${tx5}`);
    }
  } catch (error) {
    console.error({ error });
  }
})();
