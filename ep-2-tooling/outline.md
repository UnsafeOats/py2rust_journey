# Python to Rust
## General Tooling

- Creating a new project
- Basic project boilerplate
  - Rust:
    * Cargo.toml
  - Python:
    * setup.py
    * pyproject.toml
    * requirements.txt
    * virtualenv
- Unit testing:
  - Rust
    * built-in testing framework that works well
    * supports both inline unit tests and separate unit test directories
  - Python:
    * built-in testing framework sucks
    * `pytest` (an external dependency) is the most commonly used
- General Code Structure:
  - Rust:
    * Show main loop, lib.rs structure
  - Python:
    * Compare to `__name__ == "__main__":` and creating utils modules to import into main script
- Formatting
  - Rust:
    * Built in `cargo fmt` which works well and provides standardized output
  - Python:
    * No officially supported formatter.  `Black` (an external dependency) is most commonly used
- Documentation
  * `pydoc` vs `rustdoc`
- Linting
  - Rust:
    * `cargo clippy`
  - Python:
    * `pylint`/`flake8` (external dependencies)
- LSP
  - Rust:
    * `rust-analyzer`
  - Python:
    * `pyright`
- Type Hints
  - Rust:
    * Statically typed
    * Types can be explicit or implicit, but are known 100% before runtime
  - Python:
    * Dynamically typed
    * Types not enforced by Python. Type hints are just that--hints.  Can use external typing library to enforce type hints like `mypy`, but even then using type hints themselves aren't strict
- Library Ecosystem:
  - Rust:
    * Small standard library
    * Almost everything that isn't considered bare minimum has to be pulled in from an external crate
    * Even things as simple as basic async usage usually requires an external crate (or a lot of work to roll your own)
    * This helps to maintain backwards compatibility via the Rust Edition system and avoid disasters like the Python 2 -> Python 3 update
  - Python:
    * Batteries included standard library
    * Comes with much more than needed for most basic tasks
    * Strong third party library ecosystem--particularly for data science and scientific computing
  - Both:
    * Great C foreign function interface (FFI) in Rust makes it extremely easy to create Rust bindings for existing C/C++ libraries and code
    * C/C++ bindings in Python are less intuitive and require a bit more work, but are still relatively easy to implement and have been the backbone of any computation-heavy Python task
  - ChatGPT says
  ```
    Python Type System:

    Python has a dynamically typed system, which means that the type of a variable is determined at runtime rather than compile-time. In Python, variables can be assigned values of any type, and the type can change throughout the lifetime of the program. For example, a variable might be assigned an integer value initially, and later on, it might be assigned a string value. This makes Python a highly flexible and expressive language, but it can also make it harder to catch certain types of errors during development.

    Python also supports duck typing, which means that a variable's type is determined based on its behavior rather than its actual type. This allows Python to be highly dynamic and flexible, but it can also make it harder to reason about code and to catch errors.

    Rust Type System:

    Rust has a statically typed system, which means that the type of a variable is determined at compile-time rather than runtime. Rust's type system is designed to be highly expressive and safe, allowing developers to catch a wide range of errors at compile-time rather than at runtime. For example, Rust will catch errors such as null pointer dereferences, buffer overflows, and data races at compile-time rather than during execution.

    Rust's type system also supports algebraic data types and pattern matching, which make it easy to define complex data structures and to handle them in a type-safe way. Rust's type system is also designed to make it easy to reason about code and to catch errors before they occur.

    Overall, the difference between Python and Rust's type systems is that Python has a dynamically typed system that allows for flexibility and expressiveness, while Rust has a statically typed system that is designed to be highly expressive and safe. Both languages have their strengths and weaknesses, and the choice of language dep
    ```


