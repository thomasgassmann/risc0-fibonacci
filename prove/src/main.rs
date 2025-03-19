use std::time::Instant;

use risc0_zkvm::{get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext};

fn main() {
    run("./a");
    run("./b");
}

fn run(path: &str) {
    let elf = std::fs::read(path).unwrap();
    let env = ExecutorEnv::builder()
        .write::<u32>(&1000)
        .unwrap()
        .build()
        .unwrap();

    let mut exec = ExecutorImpl::from_elf(env, &elf).unwrap();
    let session = exec.run().unwrap();

    let opts = ProverOpts::default();
    let prover = get_prover_server(&opts).unwrap();
    let ctx = VerifierContext::default();
    let start = Instant::now();
    prover.prove_session(&ctx, &session).unwrap();
    let core_prove_duration = start.elapsed();

    println!("ELF {:?} took {:?}", path, core_prove_duration);
}