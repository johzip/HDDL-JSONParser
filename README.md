An open-source tool for validating planning models specified in the Hierarchical Domain Definition Language (HDDL)[^1] format based on the official HDDL syntax specification and concerete tests (including those pointing to *potential* issues) as proposed by Sleath and Bercher (2023)[^2]. HDDLAnalyzer aims to help domain modelers catch bugs by providing strong type checking, error localization, and ensuring syntactic correctness.
This tool is validated against all 33 IPC 2023 domains, and even found some errors in the benchmark set. In particular, We have found that:
1. the partially ordered version of the Barman_BDI domain diverges from the HDDL grammar on multiple occasions in empty method definitions, and
2. the Ultralight-Cockpit domain does not adhere to its defined type hierarchy in multiple instances.

## Detected Errors
In what follows, we provide the currently supported list of errors (for further details, see the paper by Sleath and Bercher (2023)[^2]). Once detected, actionable error messages in conjunction with the exact error location are returned to help modelers fix problems quickly.
* **Inconsistent Parameter Usage**: Catches type mismatches and incorrect parameter counts in task/predicate usage.
* **Undefined Entities**: Identifies undefined predicates, types, objects, and tasks.
* **Basic Syntax Issues**: Detects deviations from the HDDL grammar.
* **Duplicate Definitions**: Finds duplicate tasks, methods, predicates, and constants.
* **Cyclic Type Declarations**: Identifies cycles in the type hierarchy.
* **Undeclared Parameters**: Catches use of parameters not declared in tasks or methods.
* **Task Network Issues**: Detects cyclic ordering declaration.
* **Inconsistent Preconditions**: Flags preconditions that can never be satisfied (i.e., the formula is inconsistent).
* **Unrefinable Tasks**: Catches compound tasks without primitive refinements.

## Build Instruction
The following steps must be taken to compile the project. Wherever we mention "project_directory", we mean the root folder where the ```cargo.toml``` file is located.
1. This project was written in the Rust programming language, and requires its compiler (and cargo package manager) to be built.
If you do not have it installed, follow the official installation guide ([link](https://www.rust-lang.org/tools/install)).
2. The project depends on parts of [CreuSAT](https://github.com/sarsko/CreuSAT), a formally verified DPLL solver. In order to add this dependency, copy the ```Robinson``` directory from CreuSAT to the project_directory (i.e., you should have src, tests, and Robinson in the project_directory). This folder is automatically build with the rest of the project. However, this requires the nightly build of the Rust compiler. For instructions on how to achieve this, visit [here](https://rust-lang.github.io/rustup/concepts/channels.html).
3. Open a terminal in the project_directory, and execute ```cargo build --release```.
4. If all steps are done successfully, the executable file can be located in ```/project_directory/target/release/hddl_analyzer.exe```.

# Usage
Once you have successfully built the project and obtained ```hddl_analyzer.exe``` (the ".exe" part might differ based on your OS), you can execute the following commands. 
* To verify a domain, use ```/path/to/hddl_analyzer.exe verify /path/to/domain.hddl```
* To verify a problem, use ```/path/to/hddl_analyzer.exe verify /path/to/domain.hddl -p /path/to/problem.hddl```
* To get general information about the domain (e.g., hierarchy class), use ```/path/to/hddl_analyzer.exe metadata /path/to/domain.hddl```
* For a complete list of commands, use ```/path/to/hddl_analyzer.exe verify --help```

## Contribution
We welcome contributions and feedback from the planning community. The tool is designed to be extensible for adding new error checks as domain modeling best practices evolve. If you have spotted any bugs, please report it using the "Issues" tab in this repository.

[^1]: Höller, Daniel, Gregor Behnke, Pascal Bercher, Susanne Biundo, Humbert Fiorino, Damien Pellier, and Ron Alford. "HDDL: An Extension to PDDL for Expressing Hierarchical Planning Problems". In Proceedings of the 34th Association for the Advancement of Artificial Intelligence (AAAI) Conference, 9883–9891. AAAI Press, 2020.
[^2]: Sleath, Kayleigh, and Pascal Bercher. "Detecting AI Planning Modelling Mistakes -- Potential Errors and Benchmark Domains". In Proceedings of the 20th Pacific Rim International Conference on Artificial Intelligence (PRICAI), 448–454. Springer, 2023.