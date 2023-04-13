# Triton VM Scaffold

If you want to try out and tinker with [Triton VM](https://triton-vm.org), this repository is a great place to start.
The `main` function
- defines a program in [Triton assembly](https://triton-vm.org/spec/instructions.html),
- generates the witness that's necessary for zk-STARK proving,
- generates the proof, and
- verifies the proof.

Additionally, this repository helps answer the question:
How easy is it to use Triton VM as a dependency?
It helps identifying unintuitive code flow, overly complex structures, unnecessarily exposed internals, _et cetera_.
