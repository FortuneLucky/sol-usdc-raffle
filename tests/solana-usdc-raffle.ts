import * as anchor from '@coral-xyz/anchor';
import { Program, Wallet } from '@coral-xyz/anchor';
import { expect } from 'chai';
import {
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  transfer,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import {
  networkStateAccountAddress,
  Orao,
  randomnessAccountAddress,
  RANDOMNESS_ACCOUNT_SEED,
} from '@orao-network/solana-vrf';
import { SolanaUsdcRaffle } from '../target/types/solana_usdc_raffle';
import bs58 from 'bs58';
import { v4 as uuid } from 'uuid';
import secret from '/home/rrr/.config/solana/id.json';

describe('solana-usdc-raffle', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace
    .SolanaUsdcRaffle as Program<SolanaUsdcRaffle>;

  const connection = provider.connection;

  // Create a new oracle instance to handle random numbers
  const vrf = new Orao(anchor.getProvider() as any);

  const POOL_SEED = 'pool';
  const POOL_NATIVE_SEED = 'pool_native';
  const authKp = Keypair.fromSecretKey(new Uint8Array(secret));

  // const firstKp = new Keypair();
  // const secondKp = new Keypair();
  const firstKp = Keypair.fromSecretKey(Uint8Array.from(bs58.decode(''))); // 1sol-account
  const secondKp = Keypair.fromSecretKey(Uint8Array.from(bs58.decode(''))); // Test Account
  const treasuryKp = Keypair.fromSecretKey(Uint8Array.from(bs58.decode(''))); //  Account - 1 address

  let payTokenMint = new PublicKey(
    '7FctSfSZ9GonfMrybp45hzoQyU71CEjjZFxxoSzqKWT'
  ); // BPT mint address
  let treasury = new PublicKey('6CEUN4oMGbCQjNrzACaTKPuQKgCmqytgCDbtq5L4r6Em'); // Account - 1 address

  let decimal = 8;
  // let poolAddress: any;
  let poolAddress = new PublicKey(
    '5zZfxuJCWiPAPvwbzEYJvXB7DSQgKYoR57Qav1mXb3sT'
  );
  // let poolBump: any;
  let poolBump = 255;

  // let poolAddress: any;
  let poolNativeAddress = new PublicKey(
    '6RVxkCFwfZAKspSUNySpcMHWm12kU7aXCEojqpnE9euP'
  );
  // let poolBump: any;
  let poolNativeBump = 254;

  let buyersAddress = new PublicKey(
    'GN8tJHcKwjkei1MusTBhQJ4sv9wUwPCCw4Ey7hmEjRiz'
  );
  let authAta: any;
  let treasuryAta: any;
  let firstAta: any;
  let secondAta: any;
  let poolAta: any;

  it('Fetch all pools!', async () => {
    try {
      // Fetch the pool account and assert the values
      const allPoolAccount = await program.account.pool.all();

      console.log('allPoolAccount:', allPoolAccount);

      const allBuyers = await program.account.buyers.all();

      console.log('allBuyers:', allBuyers);
      // console.log("allBuyers:", allBuyers[0].account.buyers);
    } catch (error) {
      console.log('Error while fetching all pools:', error);
    }
  });

  it('Set Up!', async () => {
    try {
      authAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        authKp.publicKey
      );

      treasuryAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        treasury
      );

      poolAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        poolAddress,
        true
      );
      // 2GmgxnxfVJCkQCEZNtDtwSf3hgGtGP4NadKoyZRQKdwX
      console.log('poolAta', poolAta);

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   treasuryAta.address,
      //   authKp.publicKey,
      //   treasuryAmount
      // );

      firstAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        firstKp.publicKey
      );

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   firstAta.address,
      //   authKp.publicKey,
      //   firstAmount
      // );

      secondAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        secondKp,
        payTokenMint,
        secondKp.publicKey
      );

      // await transfer(
      //   program.provider.connection,
      //   authKp,
      //   authAta.address,
      //   secondAta.address,
      //   authKp.publicKey,
      //   secondAmount
      // );

      const authTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          authAta.address
        );

      console.log('authTokenAccount', authTokenAccount.value.amount);

      const treasuryTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          treasuryAta.address
        );

      console.log('treasuryTokenAccount', treasuryTokenAccount.value.amount);

      const firstTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          firstAta.address
        );

      console.log('firstTokenAccount', firstTokenAccount.value.amount);

      const secondTokenAccount =
        await program.provider.connection.getTokenAccountBalance(
          secondAta.address
        );

      console.log('secondTokenAccount', secondTokenAccount.value.amount);
    } catch (error) {
      console.log('Error while setting up:', error);
    }
  });

  // it("Create a new Raffle!", async () => {
  //   try {
  //     // Define the parameters for the create_raffle function //
  //     // Create a new uuid to use as a new raffle id
  //     const raffleId = uuid().slice(0, 8);

  //     // Display the new RaffleId
  //     console.log("newRaffleId:", raffleId);
  //     const startTime = Math.floor(Date.now() / 1000); // Current timestamp
  //     const price = Number(1); // 1 USDC
  //     const prize = Number(20); // 20 USDC
  //     const reserved = Number(0.2);
  //     const autoGenerate = Number(0);
  //     const multiplier = Number(1.5);

  //     const [pool, bump] = anchor.web3.PublicKey.findProgramAddressSync(
  //       [
  //         Buffer.from(anchor.utils.bytes.utf8.encode(POOL_SEED)),
  //         Buffer.from(anchor.utils.bytes.utf8.encode(raffleId)),
  //       ],
  //       // [Buffer.from(anchor.utils.bytes.utf8.encode(POOL_SEED)), raffleId.toArrayLike(Buffer, "le", 8)],
  //       program.programId
  //     );

  //     const [poolNativeAccount, poolNativeAccontBump] =
  //       anchor.web3.PublicKey.findProgramAddressSync(
  //         [
  //           Buffer.from(anchor.utils.bytes.utf8.encode(POOL_NATIVE_SEED)),
  //           Buffer.from(anchor.utils.bytes.utf8.encode(raffleId)),
  //         ],
  //         // [Buffer.from(anchor.utils.bytes.utf8.encode(POOL_SEED)), raffleId.toArrayLike(Buffer, "le", 8)],
  //         program.programId
  //       );

  //     poolAddress = pool;
  //     poolBump = bump;
  //     console.log("poolAddress", poolAddress);
  //     console.log("poolBump", poolBump);

  //     poolNativeAddress = poolNativeAccount;
  //     poolNativeBump = poolNativeAccontBump;
  //     console.log("poolNativeAccount", poolNativeAccount);
  //     console.log("poolNativeBump", poolNativeAccontBump);

  //     poolAta = await getAssociatedTokenAddressSync(
  //       payTokenMint,
  //       poolAddress,
  //       true
  //     );

  //     console.log("poolAta", poolAta);

  //     // 8 byte anchor discriminator + 10240 bytes for account data
  //     const space = 8 + 8 + 7200;

  //     // Calculate minimum lamports for space
  //     const rentLamports = await connection.getMinimumBalanceForRentExemption(
  //       space
  //     );

  //     console.log(rentLamports);
  //     // Generate a new random keypair to create a new account
  //     const newAccount = new Keypair();

  //     // Instruction to create new account
  //     const createAccountInstruction = SystemProgram.createAccount({
  //       fromPubkey: authKp.publicKey,
  //       newAccountPubkey: newAccount.publicKey,
  //       space: space,
  //       lamports: rentLamports,
  //       programId: program.programId, // transfers ownership to our program once created
  //     });

  //     const accountRentLamports = Number(rentLamports);

  //     // Call the create_raffle function
  //     const createRaffleInstruction = await program.methods
  //       .createRaffle(
  //         raffleId,
  //         startTime,
  //         reserved,
  //         price,
  //         prize,
  //         autoGenerate,
  //         multiplier,
  //         accountRentLamports
  //       )
  //       .accounts({
  //         admin: authKp.publicKey,
  //         pool,
  //         poolNativeAccount,
  //         buyers: newAccount.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         rent: SYSVAR_RENT_PUBKEY,
  //       })
  //       .instruction();

  //     const transaction = new Transaction().add(
  //       createAccountInstruction,
  //       createRaffleInstruction
  //     );

  //     const transactionSignature = await sendAndConfirmTransaction(
  //       connection,
  //       transaction,
  //       [authKp, newAccount]
  //     );

  //     console.log(
  //       "Your transaction signature for creating a new raffle",
  //       transactionSignature
  //     );

  //     // Fetch the pool account and assert the values
  //     const poolAccount = await program.account.pool.fetch(pool);

  //     console.log("new pool:", poolAccount);

  //     expect(poolAccount.raffleId).to.equal(raffleId);
  //     expect(poolAccount.startTime).to.equal(startTime);
  //     expect(poolAccount.ticketPrice).to.equal(price);
  //     expect(poolAccount.prize).to.equal(prize);
  //     expect(poolAccount.autoGenerate).to.equal(autoGenerate);
  //     expect(poolAccount.multiplier).to.equal(multiplier);
  //     expect(poolAccount.purchasedTicket).to.equal(0);
  //   } catch (error) {
  //     console.log("Error while creating a new raffle:", error);
  //   }
  // });

  // return;
  it('Buy tickets with Referral!', async () => {
    try {
      const totalTicket = Number(10); //
      const totalPrice = new anchor.BN(10 * Math.pow(10, decimal)); // 5 BPT token

      let poolAccount = await program.account.pool.fetch(poolAddress);

      console.log(
        new anchor.BN(
          (Number(poolAccount.accountFee.toString()) * totalTicket) /
            poolAccount.totalTicket
        ),
        poolAccount
      );

      // Call the buy_tickets function
      const tx = await program.methods
        .buyTickets(
          totalTicket,
          totalPrice,
          (Number(poolAccount.accountFee.toString()) * totalTicket) /
            poolAccount.totalTicket,
          [...poolAddress.toBuffer()]
        )
        .accounts({
          pool: poolAddress,
          poolNativeAccount: poolNativeAddress,
          payTokenMint,
          buyer: firstKp.publicKey,
          buyers: buyersAddress,
          receiver: null,
          referral: secondKp.publicKey,
          buyerAta: firstAta.address,
          adminAta: authAta.address,
          poolAta: poolAta.address,
          referralAta: secondAta.address,
          // oracle vrf related optional params //
          vrf: null,
          config: null,
          treasury: null,
          random: null,
          ////////////////////////////////
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([firstKp])
        .rpc({ skipPreflight: true });

      console.log(
        'Your transaction signature for buying tickets with referral:',
        tx
      );
      // 3f4EMSGojFop6CUTdJQARuyEadCEg4Vm4cnRaxQDuSZQLkJdc57dUvZs183BRscWjGXDFmVKhQxu79PML6VCqdLx
      // Fetch the pool account and assert the values
      poolAccount = await program.account.pool.fetch(poolAddress);

      console.log('new pool after buying tickets:', poolAccount);
    } catch (error) {
      console.log('Error while buying tickets with Referral!:', error);
    }
  });

  it('Buy tickets without Referral!', async () => {
    try {
      let poolAccount = await program.account.pool.fetch(poolAddress);

      // Get randomnessAccountAddress based on raffle account address pubkey
      const random = randomnessAccountAddress(
        poolAccount.newRandomAddress.toBuffer()
      );
      // const treasury = Keypair.generate().publicKey;
      const treasury = new PublicKey(
        '9ZTHWWZDpB36UFe1vszf2KEpt83vwi27jDqtHQ7NSXyR'
      );

      console.log({
        vrf: vrf.programId,
        config: networkStateAccountAddress(),
        treasury: treasury,
        random,
      });

      const totalTicket = Number(14); //
      const totalPrice = new anchor.BN(14 * Math.pow(10, decimal)); // 5 BPT token

      console.log(
        new anchor.BN(
          (Number(poolAccount.accountFee.toString()) * totalTicket) /
            poolAccount.totalTicket
        ),
        poolAccount
      );
      // Call the buy_tickets function
      const tx = await program.methods
        .buyTickets(
          totalTicket,
          totalPrice,
          (Number(poolAccount.accountFee.toString()) * totalTicket) /
            poolAccount.totalTicket,
          [...poolAccount.newRandomAddress.toBuffer()]
        )
        .accounts({
          pool: poolAddress,
          poolNativeAccount: poolNativeAddress,
          payTokenMint,
          buyer: secondKp.publicKey,
          buyers: buyersAddress,
          receiver: null,
          referral: null,
          buyerAta: secondAta.address,
          adminAta: authAta.address,
          poolAta: poolAta.address,
          referralAta: null,
          // oracle vrf related optional params //
          vrf: vrf.programId,
          config: networkStateAccountAddress(),
          treasury: treasury,
          random,
          ////////////////////////////////
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc({ skipPreflight: true });

      console.log(
        'Your transaction signature for buying tickets without referral:',
        tx
      );
      // 38K2a3tvigKACtGCmAA6GtJoVSjiJGYFU5BJSmXEpGTouMceZ21j4LL5Xfd9mTArgHeezuL1x1biKS7B9gX9yx3S

      // Fetch the pool account and assert the values
      poolAccount = await program.account.pool.fetch(poolAddress);

      console.log(
        'new pool after buying tickets withour referral:',
        poolAccount
      );
    } catch (error) {
      console.log('Error while buying tickets without Referral!:', error);
    }
  });

  // it("[Failure due to too many tickets] - Buy tickets without Referral!", async () => {
  //   try {
  //     let poolAccount = await program.account.pool.fetch(
  //       poolAddress
  //     );

  //     // Get randomnessAccountAddress based on raffle account address pubkey
  //     const random = randomnessAccountAddress(poolAccount.newRandomAddress.toBuffer());
  //     // const treasury = Keypair.generate().publicKey;
  //     const treasury = new PublicKey("9ZTHWWZDpB36UFe1vszf2KEpt83vwi27jDqtHQ7NSXyR");

  //     console.log({vrf: vrf.programId,
  //       config: networkStateAccountAddress(),
  //       treasury: treasury,
  //       random})

  //     const totalTicket = Number(poolAccount.totalTicket + 1); //
  //     const totalPrice = new anchor.BN((poolAccount.totalTicket + 1) * Math.pow(10, decimal)); // 5 BPT token

  //     // RangeError: The value of "value" is out of range. It must be >= 0 and <= 4294967295. Received 5_500_000_000 for u32
  //     console.log(new anchor.BN(Number(poolAccount.accountFee.toString()) *  totalTicket/ poolAccount.totalTicket), poolAccount);

  //     // Call the buy_tickets function
  //     const tx = await program.methods
  //       .buyTickets(
  //         totalTicket,
  //         totalPrice,
  //         Number(poolAccount.accountFee.toString()) *  totalTicket/ poolAccount.totalTicket,
  //         [...poolAccount.newRandomAddress.toBuffer()]
  //       )
  //       .accounts({
  //         pool: poolAddress,
  //         poolNativeAccount: poolNativeAddress,
  //         payTokenMint,
  //         buyer: secondKp.publicKey,
  //         buyers: buyersAddress,
  //         receiver: null,
  //         referral: null,
  //         buyerAta: secondAta.address,
  //         adminAta: authAta.address,
  //         poolAta: poolAta.address,
  //         referralAta: null,
  //         // oracle vrf related optional params //
  //         vrf: vrf.programId,
  //         config: networkStateAccountAddress(),
  //         treasury: treasury,
  //         random,
  //         ////////////////////////////////
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([secondKp])
  //       .rpc({ skipPreflight: true });

  //     console.log("Your transaction signature for buying tickets without referral:", tx);

  //     // Fetch the pool account and assert the values
  //     poolAccount = await program.account.pool.fetch(
  //       poolAddress
  //     );

  //     console.log("new pool after buying tickets withour referral:", poolAccount);
  //   } catch (error) {
  //     console.log("Error while buying tickets [failure due to too many tickets]", error);
  //     expect(error.error.errorMessage).to.equal("Too many ticket");
  //   }
  // });

  it('Randomness fulfilled', async () => {
    try {
      let poolAccount = await program.account.pool.fetch(poolAddress);

      let randomnessFulfilled = await vrf.waitFulfilled(
        poolAccount.newRandomAddress.toBuffer()
      );
      console.log(
        'Checking if randomness request fulfilled or not:',
        randomnessFulfilled
      );

      if (randomnessFulfilled.fulfilled) {
        // Extract the random number
        let randomness = randomnessFulfilled.randomness;
        let value = randomness.slice(0, 8); // First 8 bytes (size of u64)
        let randomNumber = Buffer.from(value).readUIntLE(0, 6);

        console.log('Random number result:', randomNumber);
        return randomNumber;
      } else {
        console.log('Randomness request not fulfilled');
        return 0;
      }
      console.log('Randomness is fulfilled, we can call the result function');
    } catch (error) {
      console.log('Error while getting fulfilled randomness:', error);
    }
  });

  it('Set winner!', async () => {
    try {
      let poolAccount = await program.account.pool.fetch(poolAddress);

      // Get randomnessAccountAddress based on raffle account address pubkey
      const random = randomnessAccountAddress(
        poolAccount.newRandomAddress.toBuffer()
      );

      // Call the claim raffle function
      const tx = await program.methods
        .setWinner([...poolAccount.newRandomAddress.toBuffer()])
        .accounts({
          signer: secondKp.publicKey,
          pool: poolAddress,
          buyers: buyersAddress,
          admin: authKp.publicKey,
          // oracle vrf related required params //
          vrf: vrf.programId,
          config: networkStateAccountAddress(),
          random,
          ////////////////////////////////
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc({ skipPreflight: true });

      console.log('Your transaction signature for setting winner:', tx);

      // Fetch the pool account and assert the values
      let poolAccountAfter = await program.account.pool.fetch(poolAddress);

      console.log('new pool after set winner:', poolAccountAfter);
    } catch (error) {
      console.log('Error while setting winner:', error);
    }
  });

  it('Claim raffle!', async () => {
    try {
      let poolAccount = await program.account.pool.fetch(poolAddress);

      const winnerAta = await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        authKp,
        payTokenMint,
        poolAccount.winner
      );

      // Call the claim raffle function
      const tx = await program.methods
        .claimPrize(poolBump, poolNativeBump)
        .accounts({
          signer: secondKp.publicKey,
          pool: poolAddress,
          poolNativeAccount: poolNativeAddress,
          buyers: buyersAddress,
          admin: authKp.publicKey,
          payTokenMint,
          winnerAta: winnerAta.address,
          adminAta: authAta.address,
          poolAta: poolAta.address,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondKp])
        .rpc({ skipPreflight: true });

      console.log('Your transaction signature for claim raffle:', tx);

      // Fetch the pool account and assert the values
      const poolAccountAfter = await program.account.pool.fetch(poolAddress);

      console.log('new pool after claim:', poolAccountAfter);
    } catch (error) {
      console.log('Error while claiming:', error);
    }
  });
});
