import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {
  createTree,
  fetchMerkleTree,
  fetchTreeConfigFromSeeds,
  findLeafAssetIdPda,
  LeafSchema,
  MetadataArgsArgs,
  mintV1,
  parseLeafFromMintV1Transaction,
} from "@metaplex-foundation/mpl-bubblegum";
import {
  generateSigner,
  none,
  publicKey,
  Signer,
  Umi,
} from "@metaplex-foundation/umi";
import { AccountMeta, PublicKey } from "@solana/web3.js";

import dotenv from "dotenv";
dotenv.config();

export async function craeteACnft(umi: Umi, merkleTreeKey?: string) {
  try {
    let merkleTreePubkey;
    if (!merkleTreeKey) {
      const merkleTree = generateSigner(umi);

      const builder = await createTree(umi, {
        merkleTree,
        maxDepth: 3,
        maxBufferSize: 8,
      });
      await builder.sendAndConfirm(umi, {
        send: { commitment: "confirmed", skipPreflight: true },
      });

      await postToTele(
        `Merkle tree created: ${merkleTree.publicKey.toString()}`
      );
      merkleTreePubkey = merkleTree.publicKey;
    } else {
      merkleTreePubkey = publicKey(merkleTreeKey);
    }

    const currentMetadata: MetadataArgsArgs = {
      name: "My Compressed NFT",
      uri: "https://raw.githubusercontent.com/priyanshuveb/solana-cnft/main/assets/collection.json",
      sellerFeeBasisPoints: 500, // 5%
      collection: none(),
      creators: [
        { address: umi.identity.publicKey, verified: false, share: 100 },
      ],
    };

    const { signature } = await mintV1(umi, {
      leafOwner: umi.identity.publicKey,
      merkleTree: merkleTreePubkey,
      metadata: currentMetadata,
    }).sendAndConfirm(umi, {
      send: { commitment: "finalized", skipPreflight: true },
    });

    const leaf: LeafSchema = await parseLeafFromMintV1Transaction(
      umi,
      signature
    );

    // const treeConfig = await fetchTreeConfigFromSeeds(umi, {
    //   merkleTree: merkleTree.publicKey,
    // });
    const [assetId, bump] = await findLeafAssetIdPda(umi, {
      merkleTree: merkleTreePubkey,
      leafIndex: leaf.nonce,
    });

    await postToTele(`Asset created: ${assetId.toString()}`);

    const rpcAsset = await umi.rpc.getAsset(assetId);

    // const merkleTreeAccount = await fetchMerkleTree(umi, merkleTree.publicKey);

    return {
      // merkleTreeAccount,
      merkleTree: merkleTreePubkey,
      // treeConfig,
      assetId,
      asset: rpcAsset,
      currentMetadata,
    };
  } catch (error) {
    console.error({ helperError: error });
  }
}

// const updateArgs: UpdateArgsArgs = {
//   name: some("New name"),
//   uri: some("https://updated-example.com/my-nft.json"),
// };

// await updateMetadata(umi, {
//   leafOwner: umiSigner.publicKey,
//   merkleTree: merkleTree.publicKey,
//   root: getCurrentRoot(merkleTreeAccount.tree),
//   nonce: assetWithProof.nonce,
//   index: assetWithProof.index,
//   currentMetadata,
//   updateArgs,
//   // collectionMint: publicKey("FLpdLuHWN6YQXXVT4BtbsXBfVc7U8F5fNeNXF1av4861"),
// }).sendAndConfirm(umi);

// assetWithProof = await getAssetWithProof(umi, assetId);
// console.log({ assetWithProof });
//   await transfer(umi, {
//     ...assetWithProof,
//     leafOwner: umiSigner.publicKey,
//     newLeafOwner: publicKey("DeL8XmsiLonPGzKEefRpcyyMXGwuxrmVR56eDHYqBhpi"),
//   }).sendAndConfirm(umi);

export async function postToTele(text: string) {
  const url = `https://api.telegram.org/bot${process.env.BOT_API}/sendMessage?chat_id=@yhjghjghj&text=${text}`;
  // console.log(url);
  const res = await fetch(url);
}

export function decode(stuff: string) {
  return bufferToArray(bs58.decode(stuff));
}
export function bufferToArray(buffer: Buffer): number[] {
  const nums: number[] = [];
  for (let i = 0; i < buffer.length; i++) {
    nums.push(buffer[i]);
  }
  return nums;
}
export const mapProof = (assetProof: { proof: string[] }): AccountMeta[] => {
  if (!assetProof.proof || assetProof.proof.length === 0) {
    throw new Error("Proof is empty");
  }
  return assetProof.proof.map((node) => ({
    pubkey: new PublicKey(node),
    isSigner: false,
    isWritable: false,
  }));
};
