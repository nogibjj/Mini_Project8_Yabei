[![Python CI/CD](https://github.com/nogibjj/Mini_Project8_Yabei/actions/workflows/py_ci.yml/badge.svg)](https://github.com/nogibjj/Mini_Project8_Yabei/actions/workflows/py_ci.yml) [![Rust CI/CD](https://github.com/nogibjj/Mini_Project8_Yabei/actions/workflows/rs_ci.yml/badge.svg)](https://github.com/nogibjj/Mini_Project8_Yabei/actions/workflows/rs_ci.yml)
# Mini Project 8: Rewrite a Python Script in Rust

## Introduction
In this mini project, we take a Python script meant for data processing and rewrite it in Rust. This project aims to showcase the improvements in speed and resource usage when migrating from Python to Rust for data processing tasks.

## Requirements

- Start with an existing Python script for data processing.
- Rewrite the same script in Rust.
- Highlight the improvements in speed and resource usage between the Python and Rust implementations.


## Repository Contents

- **YML Files**: Contains workflow definitions for continuous integration and deployment, one for Rust and another for Python.
- **lib**: A shared Python library with functions used in `main.py`. This library calculates summary statistics (mean, median, standard deviation) for the dataset `cars.csv`.
- **main.py** and **test.py**: Python scripts for the main application and tests respectively.
- **Rust Code**: Contains `main.rs` and `lib.rs`, which are Rust equivalents of the Python code.
- **requirements.txt**: Lists the dependencies for the Python project.

## Dependencies

The project has the following dependencies:

### Main Dependencies

- **csv**: `1.1`
  - A fast CSV reading/writing library for Rust.
  
- **rusqlite**: `0.24`
  - Rust bindings for SQLite.
  
- **statrs**: `0.14`
  - Provides statistics and numerical routines for Rust.
  
- **reqwest**: `0.11`
  - An easy and powerful Rust HTTP Client, with `blocking` feature enabled.
  
- **sys-info**: `0.7`
  - A library to get system information in Rust.

### Development Dependencies

- **pretty_assertions**: `0.7`
  - Overwrite panics with pretty assertions in Rust.



### Preparation
- **Linting:**

Linting helps identify potential issues in the code. Run the lint command to check for any such problems:

```bash
make py_lint
```

- **Running Tests:**
Before finalizing any changes, ensure all tests pass. Run the test suite using:
```bash
make py_test
```
- **Formating:**
To ensure the code adheres to the project's style guidelines
```bash
make py_format
```


### Results for Python Lint and Test
![lint_test](https://github.com/nogibjj/Mini_Project8_Yabei/assets/143656459/bea6742b-6f2b-4f6f-a93e-824c48bef32e)

### Comparison between Python and Rust
#### Python Results
![py_test](https://github.com/nogibjj/Mini_Project8_Yabei/assets/143656459/73fc742e-635b-417a-b5b9-653598f2bc91)

The elapsed time for python is 0.0024 seconds

The CPU usage for python is 50%

The memory usage is 43.4%

#### Rust Results
![rust_test](https://github.com/nogibjj/Mini_Project8_Yabei/assets/143656459/8411dfda-2ad2-4bd2-84a6-23420daec580)

The elapsed time for rust is 0.001078813 seconds

The CPU usage is 15%

The memory usage is 40.91223%

Overall, Rust looks to be more efficient in this test, both in terms of execution time and CPU consumption. Memory utilization is equivalent between the two, with Rust having a little advantage. One possible reason could be that: Rust, a compiled language with aggressive optimizations and a unique ownership system, frequently outperforms Python, an interpreted language, in raw computational tasks. While Rust's design emphasizes zero-cost abstractions and efficient memory management without a garbage collector, Python prioritizes developer productivity and readability. As a result, Python may introduce more runtime overhead. However, test specifics, library optimizations, and the nature of tasks can all influence performance differences.

### License. 

This project is licensed under the CC0 1.0 Universal license.


### Reference
https://github.com/kbknapp/rust-cli-template

https://github.com/nogibjj/rust-mlops-template
