import * as anchor from "@coral-xyz/anchor";
import { WoofiContext } from "../src";
import { PYTH_FEED_ACCOUNT, PYTH_PRICE_UPDATE_ACCOUNT, TOKEN_MINTS, WOOFI_TOKENS } from '../src/utils/constants'
import { generatePoolParams, getWooPrice } from "../src/utils/contract";
import { Connection, PublicKey, Transaction } from "@solana/web3.js";
import { createTransferCheckedInstruction, getAssociatedTokenAddressSync } from "@solana/spl-token";

async function getTokenBalanceWeb3(connection: Connection, tokenAccount: PublicKey) {
  const info = await connection.getTokenAccountBalance(tokenAccount);
  if (info.value.uiAmount == null) throw new Error('No balance found');
  console.log('Balance (using Solana-Web3.js): ', info.value.uiAmount);
  return info.value.uiAmount;
}

describe("woofi_sdk", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const ctx = WoofiContext.from(provider.connection, provider.wallet);

    // increase to pool liquidity
    // const providerToTokenAccount = getAssociatedTokenAddressSync(tokenMint, provider.wallet.publicKey);
    // const beforeProviderToTokenBalance = await provider.connection.getTokenAccountBalance(providerToTokenAccount);
    // console.log("beforeProviderToTokenBalance amount:" + beforeProviderToTokenBalance.value.amount);
    // console.log("beforeProviderToTokenBalance decimals:" + beforeProviderToTokenBalance.value.decimals);

    // const transferToTranscation = new Transaction().add(
    //   // trasnfer USDC to to account
    //   createTransferCheckedInstruction(
    //     providerToTokenAccount,
    //     tokenMint,
    //     poolParams.tokenVault,
    //     provider.wallet.publicKey,
    //     200,
    //     woopool.baseDecimals
    //   ),
    // );

    // await provider.sendAndConfirm(transferToTranscation, [], { commitment: "confirmed" });

    // const afterToTokenBalance = await provider.connection.getTokenAccountBalance(toPoolParams.tokenVault);
    // console.log("afterProviderToTokenBalance amount:" + afterToTokenBalance.value.amount);
    // console.log("afterProviderToTokenBalance decimals:" + afterToTokenBalance.value.decimals);

    // Create transferChecked instruction
    // const instruction = createTransferCheckedInstruction(
    //   woopool.tokenVault, // source token account
    //   woopool.tokenMint, // mint address
    //   provider.wallet.publicKey, // destination token account
    //   poolParams.woopool, // source token account owner
    //   100, // amount (1 token)
    //   woopool.baseDecimals, // decimals of mint
    //   [poolParams.woopool]
    // );

    // const seeds = [Buffer.from('woopool'), poolParams.wooconfig.toBuffer(), tokenMint.toBuffer(), quoteTokenMint.toBuffer()]

    // const tx = new Transaction();
    // tx.add(instruction);
    // const sig = await provider.sendAndConfirm(tx, [], { commitment: "confirmed" });

    // let t = await provider.connection.getTransaction(sig, {
    //   commitment: "confirmed",
    // })!;
  //})
});
