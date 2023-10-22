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

### Dependencies

- **DevOps**: `black`, `click`, `pytest`, `pytest-cov`, `requests`
- **Rust Linter**: `ruff`
- **Data Processing**: `pandas`, `psutil`

## CI/CD

### Python CI/CD Workflow

```yaml
name: Python CI/CD

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v2

      - name: Sleep for 80 seconds
        run: sleep 80

      - name: Install Python packages
        run: make py_install

      - name: Lint Python code
        run: make py_lint

      - name: Run Python tests
        run: make py_test

      - name: Format Python code
        run: make py_format

      - name: Pull latest changes
        run: git pull

      - name: generate_and_push
        run: make generate_and_push

      - name: Deploy
        run: make py_deploy
```

### Makefile

A simplified version of the Makefile has been provided. This file contains both Rust and Python targets for various tasks including linting, testing, and deployment.

### Results for Python Lint and Test
![lint_test](https://github.com/nogibjj/Mini_Project8_Yabei/assets/143656459/bea6742b-6f2b-4f6f-a93e-824c48bef32e)

### Comparison between Python and Rust
#### Python Results
![py_test](https://github.com/nogibjj/Mini_Project8_Yabei/assets/143656459/73fc742e-635b-417a-b5b9-653598f2bc91)
The elapsed time for python is 0.0024 seconds
The CPU usage for python is 50%
The memory usage is 43.4%

#### Rust Results
The elapsed time for rust is 0.001106444 seconds
The memory usage is 4MB

## License

This project is licensed under the CC0 1.0 Universal license.


### Reference
https://github.com/kbknapp/rust-cli-template

https://github.com/nogibjj/rust-mlops-template
