import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentIntro } from "../target/types/anchor_student_intro";
import { expect } from "chai";

describe("anchor-student-intro", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorStudentIntro as Program<AnchorStudentIntro>;

  const studentIntro = {
    student_name: "John Doe",
    intro: "Hello everybody, my name is John Doe",
  }

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(studentIntro.student_name),
    provider.wallet.publicKey.toBuffer()],
    program.programId);

  it("Student Intro created", async () => {
    const tx = await program.methods.addStudentIntro(studentIntro.student_name, studentIntro.intro).rpc();

    const account = await program.account.studentIntroState.fetch(studentPda);
    expect(studentIntro.student_name === account.studentName);
    expect(studentIntro.intro === account.intro);
    expect(account.student === provider.wallet.publicKey);
  });

  it("Student Intro updated", async () => {
    const newIntro = "Hi fellows, I'm John Doe."
    const tx = await program.methods.updateStudentIntro(studentIntro.student_name, newIntro).rpc();

    const account = await program.account.studentIntroState.fetch(studentPda);
    expect(studentIntro.student_name === account.studentName);
    expect(newIntro === account.intro);
  });

  it("Student Intro delete", async () => {
    const tx = await program.methods.deleteStudentIntro(studentIntro.student_name).rpc();

    try {
      const account = await program.account.studentIntroState.fetch(studentPda)
    } catch (error) {
      console.log("\nFailed fetching account: ", error);
    }
  })
});
