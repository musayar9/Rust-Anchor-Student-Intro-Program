import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentIntroProgram } from "../target/types/student_intro_program";
import { expect } from "chai";

describe("student-intro-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.StudentIntroProgram as Program<StudentIntroProgram>;

 const student = {
  name:"Chris Evans",
  message:"I'm Captan America"

 } 

 const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
  program.programId

 )

 it("Student is added", async()=>{
  // add student

  const tx = await program.methods.addStudentIntro(student.name, student.message)
  .rpc();

  const account = await program.account.studentIntroState.fetch(studentPda);
    expect(student.name === account.name);
    expect(student.message === account.message);
    expect(account.student === provider.wallet.publicKey)
 });

 it("Student is updated", async()=>{

  const newMessage = "I'm Iron Man";
  const tx = await program.methods.updateStudentIntro(student.name, newMessage).rpc();


  const account = await program.account.studentIntroState.fetch(studentPda);
  expect(student.name === account.name);
  expect(newMessage === account.message);
  expect(account.student === provider.wallet.publicKey)

 })

 it("Delete studnet", async()=>{
  const tx= await program.methods.deleteStudentIntro(student.name).rpc();

 })
});
