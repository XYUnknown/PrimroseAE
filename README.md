# Getting Start

This is the research artifact for the paper *"Primrose: Selecting Container Data Types by their Properties"*.

For easy of evaluation, the artifact is provided as a VirtualBox virtual machine with all dependencies already pre-installed.
Below, we provide a guide on how to check that the claims made in the paper are supported by the artifact.
We encourage the evaluators to experiment themselves with the provided examples.


## Logging into the VM and locating the artifact
- user name: `user`
- password: `helloworld`
- artifact location: `/home/user/Documents/PrimroseAE/`

# Overview of the artifact
This artifact supports the claims made in the paper by containing:
- The property specifications introduced in section 4 of the paper.
- An implementation of a parser and type checker of the property specifications accompanied by a type-inference algorithm.
- An implementation of the container library studied in section 5 of the paper.
- Library specifications for each container discussed in section 5.
- An implementation of the selection process described in section 6 of the paper.
- An implementation of the code generation described in section 6.4 of the paper.
- Property based testing as described in the section 7.1 of the paper. 
- A script for measuring the solver times reported in section 7.2 of the paper. We do expect that runtimes measured in the virtual machine might differ from the numbers reported in the paper that have been measured outside the virtual machine. 

# Step-by-Step Instructions
## Walkthrough code related to features introduced in the paper
- The *syntactic properties* introduced in section 4.1 can be found in `./src/traits/mod.rs`
- The *semantic properties* introduced in section 4.2, can be found in:
  - the `unique` property specification can be found in `./spec_code/example_unique.rs`
  - the `ascending` property specification as well as the composition of `ascending` and `unique` can be found in `./spec_code/example_comp.rs`.
- The property specification of the stack example introduced in section 4.3 can be found in `./spec_code/example_stack.rs`
- The combinators used in property specifications are provided in `./racket_specs/combinators.rkt`
- All library specifications introduced in section 5 can be found in `./src/library/`
- All property based tests introduced in section 7 can also be found in `./src/library/`

## Selecting of valid container implementations and Rust code generation via the `primrose` tool
- To run the *unique container example* from the paper:
  - Make sure you are in the directory `PrimroseAE`
  - Run command: `cargo run example_unique.rs unique 3`
  - Generated code can be found in the directory `./gen_code/unique/`
  - For this example we expect that three files each with a different container implementation are generated:
    - `./gen_code/unique/unique0.rs`
    - `./gen_code/unique/unique1.rs`
    - `./gen_code/unique/unique2.rs`
  - To compile the generated Rust code, go to `Cargo.toml`, add at the end for the code you want to compile:
    ```
    [[bin]]
    name = "unique0"
    path = "gen_code/unique/unique0.rs"

    [[bin]]
    name = "unique1"
    path = "gen_code/unique/unique1.rs"

    [[bin]]
    name = "unique2"
    path = "gen_code/unique/unique2.rs"
    ```
  - Then you can compile and execute the generated file with:
    - `cargo run --bin unique0`
    - `cargo run --bin unique1`
    - `cargo run --bin unique2`

- To run the *unique and ascending (strictly ascending) container example* from the paper:
  - Make sure you are in the directory `PrimroseAE`
  - Run command: `cargo run example_comp.rs comp 3`
  - Generated code can be found under the directory `./gen_code/comp/`
  - To compile the generated Rust code, add it to `Cargo.toml` as above and then execute it via `cargo run`
- To run the *stack example* from the paper:
  - Make sure you are in the directory `PrimroseAE`
  - Run command: `cargo run example_stack.rs stack 3`
  - Generated code can be found under the directory `./gen_code/stack/`
  - - To compile the generated Rust code, add it to `Cargo.toml` as above and then execute it via `cargo run`

## Running property based testing from section 7.1
- Make sure you are in the directory `PrimroseAE`
- Run command: `cargo test`
- if you want to measure how long it takes to execute all tests: `./timetests.sh`

## Producing solver efficiency benchmarks form section 7.2
- Make sure you are in the directory `PrimroseAE`
- Run command: `./timeall.sh`
- **Please note:** we do not expect that the times measured inside the virtual machine and on different hardware will be exactly the same as the times presented in the paper.

# Technical Overview of the Artifact

## Overview of pre-installed dependencies
- Rust 1.67.0-nightly
- Racket 8.1 or later
- Rosette
  - We used the Z3 backend in this project
- These dependencies are all pre-installed, to check they are installed correctly:
  - Type command: `rustc --version` in terminal, you should get:
    ```
    rustc 1.67.0-nightly (01fbc6ae7 2022-12-07)
    ```
  - Type command: `racket --version` in terminal, you should get:
    ```
    Welcome to Racket v8.6 [cs].

## Execution of the `primrose` tool with arbitrary property specification
- Make sure you are in the `PrimroseAE` directory
- Make sure the Rust program with embedded property specifications (`input.rs`) is provided in the directory `./spec_code/`
- Run the tool with command:
  ```
  cargo run [input.rs] [output_dir_name] [model_size]
  ```
- For most properties, we recommend a model size of `3`
- The generated file will appear in the directory `[output_dir_name]` and can be compiled with `cargo` after an entry for it has been added at the end of `Cargo.toml` file:
    ```
    [[bin]]
    name = "name_you_like"
    path = "path/to/file.rs"
    ```
- To execute the generated Rust code run:
  ```
  cargo run --bin name_you_like
  ```

## File structure of the `PrimroseAE` directory
- `./benches/`: containing the code producing the runtime performance benchmarks in section 2
-  `./gen_code/`: containing generated code with selected container implementations
- `./mem_profiling/`: containing scripts producing the memory consumption benchmarks in section 2
- `./racket_specs/`: containing scripts for setting up and executing the selection process and generated code during the selection process
- `./scripts/`: containing the scripts for measuring the solver's selection time introduced in section 7.2
- `./spec_code/`: containing source code with property specifications introduced in section 4.
- `./src/library/`: containing container implementations used in this paper, library specifications introduced in section 5 and property based tests introduced in section 7.1
- `./src/proptest/`: containing code for setting up property based tests.
- `./src/tools/`: containing the code for generating dataset memory profiling.
- `./src/traits/`: container syntactic properties introduced in section 4.
- `./src/main.rs`: the entry point for executing the tool.
- All other files in the `./src/` directory are the detailed implementation of Primrose, including parsing, type checking and analyzing property specifications, extracting and processing library specifications, executing the selection and generating code.
- `./runall.sh` is the script for executing all examples
- `./timeall.sh` is the script running every script located in `./scripts/` measuring the solver's selection time introduced in section 7.2
- `./timetests.sh` is the script measuring the time for running all property based tests reported in section 7.1
- `./Cargo.toml`, `./Cargo.lock` and `rust-toolchain.toml` are Rust package and compiler configuration files.
