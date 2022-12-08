# Getting Start
## Dependencies
- Rust 1.60.0-nightly
- Racket 8.1 or later
- Rosette
  - We used the Z3 backend in this project

## File structure
```
./
|-- benches
|   `-- ...
|-- gen_code
|   `-- ...
|-- mem_profiling
|   `-- ...
|-- racket_specs
|   |-- gen_lib_spec
|   |   `-- ...
|   |-- gen_match
|   |   `-- ...
|   |-- gen_prop_spec
|   |   `-- ...
|   |-- combinators.rkt
|   |-- container-setup.rkt
|   |-- randomaccess-setup.rkt
|   |-- stack-setup.rkt
|   `-- ...
|-- scripts
|   |-- b_asc_con_3.sh
|   |-- ...
|-- spec_code
|   |-- b_asc_con.rs
|   |-- ...
|-- src
|   |-- library
|   |   |-- eager_sorted_vector.rs
|   |   |-- eager_unique_vector.rs
|   |   |-- hashset.rs
|   |   |-- lazy_sorted_vector.rs
|   |   |-- lazy_unique_vector.rs
|   |   |-- list.rs
|   |   |-- mod.rs
|   |   |-- treeset.rs
|   |   `-- vector.rs
|   |-- proptest
|   |   |-- mod.rs
|   |   `-- strategies.rs
|   |-- tools
|   |   `-- mod.rs
|   |-- traits
|   |   |-- container_constructor.rs
|   |   `-- mod.rs
|   |-- analysis.rs
|   |-- bounded_ops.rs
|   |-- description.rs
|   |-- generator.rs
|   |-- inference.rs
|   |-- lib.rs
|   |-- lib_spec_processor.rs
|   |-- main.rs
|   |-- parser.rs
|   |-- run_matching.rs
|   |-- spec_map.rs
|   |-- type_check.rs
|   `-- types.rs
|-- Cargo.lock
|-- Cargo.toml
|-- README.md
|-- runall.sh
|-- rust-toolchain.toml
|-- timeall.sh
`-- timetests.sh
```
- `./benches/`: containing the code producing the runtime performance benchmarks in section 2
-  `./gen_code/`: containing generated code with selected container implementations
- `./mem_profiling/`: containing scripts producing the memeory cunsumption benchmarks in section 2
- `./racket_specs/`: containing scripts for setting up and executing the selection process and generated code during the selection process
- `./scripts/`: containing the scripts for measuring the solver's selection time introduced in section 7.2
- `./spec_code/`: containing source code with property specifications introduced in section 4.
- `./src/library/`: containing container implementations used in this paper, library specifications introduced in section 5 and property based tests introduced in section 7.1
- `./src/proptest/`: containing code for setting up property based tests.
- `./src/tools/`: containing the code for generating dataset memory profiling.
- `./src/traits/`: container syntatic properties introduced in section 4.
- `./src/main.rs`: the entry point for executing the tool.
- All other files in the `./src/` directory are the detailed implementation of Primrose, including parsing, type checking and analysing property specifications, extracting and processing library specifications, executing the selection and generating code.
- `./runall.sh` is the script for executing all examples
- `./timeall.sh` is the script running every script located in `./scripts/` measuring the solver's selection time introduced in section 7.2
- `./timetests.sh` is the script measuring the time for running all propert based tests reported in section 7.1
- `./Cargo.toml`, `./Cargo.lock` and `rust-toolchain.toml` are Rust package and compiler configuration files.


## Basic execution of the tool
- Make sure you are under the directory `PrimroseAE`
- Make sure the program with property specifications is under the directory `./spec_code/`
- Run the tool with command:
```
cargo run [input.rs] [output_dir_name] [model_size]
```
For example:
```
cargo run example_unique.rs unique 3
```
- The generated file will appear in the directory `./gen_code/`

# Overview of Claims
## Supported claims
- The property specifications introduced in the paper are demonstrated.
- A parser and type checker accompanied by the type inference algorithm are implemented.
- The container library studied in the paper is provided.
- Library specifications for each container are demostrated.
- The selection process is implemented.
- Code generation is implementated.
- Property based testing is conducted. Tests are included in this artifact.

## Unsupported claims
- The exact solver efficiency benchmarks introduced in the evaluation section cannot be reproduced in VM. Those benchmarks are produced from a specific machine as we explained in the paper.

# Step-by-Step Instructions
## Walkthough code related to features introduced in the paper
- In section 4.1, some syntatic properties are introduced, they can be found in `./src/traits/mod.rs`
- In section 4.2, some semantic properties are introduced, the `unique` property specification can be found in `./spec_code/example_unique.rs`. The `ascending` property specification as well as the composition of `ascending` and `unique` can be found in `./spec_code/example_comp.rs`.
- In section 4.3, the property specification of the stack example can be found in `./spec_code/example_stack.rs`
- The combinators used in property specifications are provided in `./racket_specs/combinators.rkt`
- All library specifications introduced in section 5 can be found in `./src/library/`
- All property based tests introduced in section 7 can also be found in `./src/library/`

## Example executions
- To run the unique container example:
  - Make sure you are under the directory `PrimroseAE`
  - Run command: `cargo run example_unique.rs unique 3`
  - Generated code can be found uner the directory `./gen_code/unique/`
  - To compile generated code, go to `Cargo.toml`, specify the code you want to compile with the format:
    ```
    [[bin]]
    name = "name_you_like"
    path = "path/to/file.rs"
    ```
  
    For example: 
    ```
    [[bin]]
    name = "unique"
    path = "gen_code/unique/unique0.rs"
    ```
  - Then you can compile and execute the generated file with command: `cargo run --bin [name_you_like]`
  for example: `cargo run --bin unique`
- To run the unique and ascending (strictly ascending) container example:
  - Make sure you are under the directory `PrimroseAE`
  - Run command: `cargo run example_comp.rs comp 3`
  - Generated code can be found uner the directory `./gen_code/comp/`
- To run the stack example:
  - Make sure you are under the directory `PrimroseAE`
  - Run command: `cargo run example_stack.rs stack 3`
  - Generated code can be found uner the directory `./gen_code/stack/`

## Running property based testing
- Make sure you are under the directory `PrimroseAE`
- Run command: `cargo test`
- if you want to measure how long it takes to execute all tests: `./timetests.sh`

## Producing solver effciency benchmarks
- Make sure you are under the directory `PrimroseAE`
- Run command: `./timeall.sh`