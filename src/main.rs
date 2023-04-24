fn main() {
    // The source of the program that is to be run in Triton VM. Written in Triton assembly.
    // The example program given here expects one public input `x` and one secret input `y`.
    // It asserts that (x·y)² = 17 and returns 1337.
    // Note that all arithmetic is in the prime field with 2^64 - 2^32 + 1 elements.
    let source_code = "read_io divine mul dup 0 mul push 17 eq assert push 1337 write_io halt";

    // Define public and secret inputs.
    // Since arithmetic is in the prime field, the inputs must be in canonical representation,
    // i.e., smaller than the prime field's modulus 2^64 - 2^32 + 1. Otherwise, proof generation
    // will be aborted.
    let public_input = [42];
    let secret_input = [16372729857439537988];

    // Generate the claim that is to be proven, as well as the corresponding proof.
    // The claim contains all public information:
    //   - the program's public input,
    //   - the program's hash digest under hash function Tip5,
    //   - the program's public output, and
    //   - an upper bound for the number of steps the program was running for.
    // Triton VM is zero-knowledge with respect to everything else.
    // The proof contains the cryptographic information asserting the claim's correctness.
    // Triton VM's default parameters give a (conjectured) security level of 160 bits.
    let (claim, proof) = triton_vm::prove(source_code, &public_input, &secret_input);
    let public_output = claim.output.to_owned();

    // Verify the proof.
    let verdict = triton_vm::verify(claim, proof);
    assert!(verdict);
    println!("Success! Output: {:?}", public_output);
}
