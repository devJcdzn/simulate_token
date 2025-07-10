import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimulateToken } from "../target/types/simulate_token";

describe("token_simulado", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.simulate_token as Program<SimulateToken>;

  const tokenMetadata = anchor.web3.Keypair.generate();
  const userTokenAccount1 = anchor.web3.Keypair.generate();
  const userTokenAccount2 = anchor.web3.Keypair.generate();

  it("Cria um novo token", async () => {
    await program.methods
      .resgisterToken("FLIKK", "KK", new anchor.BN(1000))
      .accounts({
        tokenMetadata: tokenMetadata.publicKey,
        userTokenAccount: userTokenAccount1.publicKey,
        creator: provider.wallet.publicKey,
      })
      .signers([tokenMetadata, userTokenAccount1])
      .rpc();

    const token = await program.account.tokenMetadata.fetch(
      tokenMetadata.publicKey
    );
    const balance = await program.account.userTokenAccount.fetch(
      userTokenAccount1.publicKey
    );

    console.log("Token criado:", token);
    console.log("Saldo do criador:", balance.balance.toString());
  });

  it("Cria uma conta de token para outro usuário", async () => {
    await program.methods
      .createUserTokenAccount()
      .accounts({
        userTokenAccount: userTokenAccount2.publicKey,
        token: tokenMetadata.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([userTokenAccount2])
      .rpc();

    const userAccount = await program.account.userTokenAccount.fetch(
      userTokenAccount2.publicKey
    );
    console.log(
      "Conta de usuário criada com saldo:",
      userAccount.balance.toString()
    );
  });

  it("Transfere tokens entre contas", async () => {
    await program.methods
      .transfer(new anchor.BN(100))
      .accounts({
        senderAccount: userTokenAccount1.publicKey,
        receiverAccount: userTokenAccount2.publicKey,
      })
      .rpc();

    const sender = await program.account.userTokenAccount.fetch(
      userTokenAccount1.publicKey
    );
    const receiver = await program.account.userTokenAccount.fetch(
      userTokenAccount2.publicKey
    );

    console.log("Saldo do remetente:", sender.balance.toString());
    console.log("Saldo do destinatário:", receiver.balance.toString());
  });
});
