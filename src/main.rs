use triton_opcodes::program::Program;
use triton_vm::proof::Claim;
use triton_vm::stark::*;
use triton_vm::table::master_table::MasterBaseTable;
use triton_vm::vm;
use twenty_first::shared_math::b_field_element::BFieldElement;

fn main() {
    // The source of the program that is to be run in Triton VM. Written in Triton assembly.
    // The example program given here expects one public input `x` and one secret input `y`.
    // It asserts that (x·y)² = 17 and returns 1337.
    // Note that all arithmetic is in the prime field with 2^64 - 2^32 + 1 elements.
    let source_code = "read_io divine mul dup0 mul push 17 eq assert push 1337 write_io halt";

    // Parse the Triton assembly into a program.
    let program = Program::from_code(source_code).unwrap();

    // Define public and secret inputs.
    let public_input = [42].map(BFieldElement::new).to_vec();
    let secret_input = [16372729857439537988].map(BFieldElement::new).to_vec();

    // Generate
    // - the witness required for proof generation, i.e., the Algebraic Execution Trace (AET),
    // - the (public) output of the program, and
    // - an error, if the program crashes.
    let (aet, public_output, maybe_error) =
        vm::simulate(&program, public_input.clone(), secret_input);

    // Check for VM crashes, for example due to failing `assert` instructions or an out-of-bounds
    // instruction pointer. Crashes signify a buggy program being fed to Triton VM.
    // If the VM crashes, proof generation will fail.
    if let Some(error) = maybe_error {
        panic!("Simulation error: {error}");
    }

    // Set up the claim that is to be proven. The claim contains all public information. The
    // proof is zero-knowledge with respect to everything else.
    let claim = Claim {
        input: public_input,
        program: program.to_bwords(),
        output: public_output.clone(),
        padded_height: MasterBaseTable::padded_height(&aet),
    };

    // Construct a new STARK instance. The default parameters give a (conjectured) security level
    // of 160 bits.
    let stark = Stark::new(claim, StarkParameters::default());

    // Generate the proof.
    let proof = stark.prove(aet, &mut None);

    // Verify the proof.
    let verdict = stark.verify(proof, &mut None);
    if let Err(error) = verdict {
        panic!("Verification error: {error}");
    }
    assert!(verdict.unwrap());
    println!(
        "Success! Output: [{}]",
        public_output
            .into_iter()
            .map(|x| x.value().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}
