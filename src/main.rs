use triton_opcodes::program::Program;
use triton_vm::proof::Claim;
use triton_vm::stark::*;
use triton_vm::table::master_table::MasterBaseTable;
use triton_vm::vm;
use twenty_first::shared_math::b_field_element::BFieldElement;

fn main() {
    let source_code = "read_io divine mul dup0 mul push 17 eq assert push 1337 write_io halt";
    let program = Program::from_code(source_code).unwrap();
    let public_input = vec![BFieldElement::new(42)];
    let secret_input = vec![BFieldElement::new(16372729857439537988)];
    let (aet, public_output, maybe_error) =
        vm::simulate(&program, public_input.clone(), secret_input);
    if let Some(error) = maybe_error {
        panic!("Simulation error: {}", error);
    }

    let claim = Claim {
        input: public_input,
        program: program.to_bwords(),
        output: public_output.clone(),
        padded_height: MasterBaseTable::padded_height(&aet),
    };

    let stark = Stark::new(claim, StarkParameters::default());
    let proof = stark.prove(aet, &mut None);
    let verdict = stark.verify(proof, &mut None);
    if let Err(error) = verdict {
        panic!("Verification error: {error}");
    }
    assert!(verdict.unwrap());
    println!("Success! Output: {public_output:?}");
}
