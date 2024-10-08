import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { AgrotreeMarketplace } from "../target/types/agrotree_marketplace";
import { assert } from "chai";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createTree,
  fetchTreeConfigFromSeeds,
  findLeafAssetIdPda,
  mintV1,
  mplBubblegum,
} from "@metaplex-foundation/mpl-bubblegum";
import {
  createSignerFromKeypair,
  generateSigner,
  none,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import { WrappedConnection } from "./WrappedConnection";

describe("agrotree-marketplace", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = new WrappedConnection(provider.connection.rpcEndpoint);

  const program = anchor.workspace
    .AgrotreeMarketplace as Program<AgrotreeMarketplace>;
  const authority = provider.wallet.publicKey;
  const FEE = 300;

  const [user1, user2] = [web3.Keypair.generate(), web3.Keypair.generate()];

  const umi = createUmi(provider.connection.rpcEndpoint)
    .use(mplBubblegum())
    .use(mplTokenMetadata());
  //.use(dasApi())

  const umiSigner = createSignerFromKeypair(
    umi,
    umi.eddsa.createKeypairFromSecretKey(new Uint8Array(user1.secretKey))
  );
  umi.use(signerIdentity(umiSigner));

  const merkleTree = generateSigner(umi);

  before(async () => {
    {
      await provider.connection.confirmTransaction({
        signature: await provider.connection.requestAirdrop(
          user1.publicKey,
          10 * web3.LAMPORTS_PER_SOL
        ),
        ...(await provider.connection.getLatestBlockhash()),
      });
      await provider.connection.confirmTransaction({
        signature: await provider.connection.requestAirdrop(
          user2.publicKey,
          10 * web3.LAMPORTS_PER_SOL
        ),
        ...(await provider.connection.getLatestBlockhash()),
      });
    }
    {
      const builder = await createTree(umi, {
        merkleTree,
        maxDepth: 3,
        maxBufferSize: 8,
        treeCreator: umiSigner,
      });
      await builder.sendAndConfirm(umi, {
        send: { commitment: "finalized", skipPreflight: true },
      });

      await mintV1(umi, {
        leafOwner: umiSigner.publicKey,
        merkleTree: merkleTree.publicKey,
        metadata: {
          name: "My Compressed NFT",
          uri: "https://example.com/my-cnft.json",
          sellerFeeBasisPoints: 500, // 5%
          collection: none(),
          creators: [
            { address: umi.identity.publicKey, verified: false, share: 100 },
          ],
        },
      }).sendAndConfirm(umi, {
        send: { commitment: "finalized", skipPreflight: true },
      });

      await mintV1(umi, {
        leafOwner: umiSigner.publicKey,
        merkleTree: merkleTree.publicKey,
        metadata: {
          name: "My Compressed NFT",
          uri: "https://example.com/my-cnft.json",
          sellerFeeBasisPoints: 500, // 5%
          collection: none(),
          creators: [
            { address: umi.identity.publicKey, verified: false, share: 100 },
          ],
        },
      }).sendAndConfirm(umi, {
        send: { commitment: "finalized", skipPreflight: true },
      });

      const treeConfig = await fetchTreeConfigFromSeeds(umi, {
        merkleTree: merkleTree.publicKey,
      });
      console.log({ treeConfig });

      const [assetId] = await findLeafAssetIdPda(umi, {
        merkleTree: merkleTree.publicKey,
        leafIndex: 0,
      });
      console.log({ assetId });

      const rpcAsset = await umi.rpc.getAsset(assetId);
      console.log({ rpcAsset });

      // const assetWithProof = await connection.getAssetWithProof(assetId);

      // console.log({ assetWithProof });
    }
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(FEE)
      .accounts({
        authority: authority,
      })
      .rpc();

    assert.ok(tx);
    console.log("Your transaction signature", tx);

    const [configAddress, configBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("m-config")],
      program.programId
    );
    const configAccount = await program.account.marketConfig.fetch(
      configAddress
    );

    assert(configAccount.authority.equals(authority));
    assert(configAccount.fee == FEE);
    assert(configAccount.bump == configBump);
  });
});
