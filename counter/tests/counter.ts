import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Creates a reference to the program just built
  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    // Call the initialize method defined in our program
    const tx = await program.methods.initialize().rpc(); // .rpc is like an api call
    console.log("Your transaction signature", tx);
  });
});
