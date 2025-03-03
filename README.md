# Running_App_Rust

[![Clippy](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/lint.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/lint.yml)
[![Build binary release](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/release.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/release.yml)
[![Rustfmt](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/rustfmt.yml)
[![Tests](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/tests.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/tests.yml)

## Description.

Running_App_Rust is a web application written in Rust that calculates various training parameters such as TRIMP (Training Impulse), TRIMP_LT (Lactate Threshold Training Impulse) and HRRS (Heart Rate Recovery Score).

## Installation.

1. Clone the repository:
    ``sh
    git clone https://github.com/AndreCanto00/Running_App_Rust.git
    cd Running_App_Rust
    ```

2. Build the project:
    ``sh
    cargo build
    ```

3. Run tests:
    ```sh
    cargo test
    ```

## Usage

To start the server:
```sh
cargo run
```

The server will be running at `http://0.0.0.0:8080`.

### Endpoint

- `GET /`: Returns a welcome message.
- `POST /trimp`: Calculates the TRIMP value.
- `POST /trimp_lt`: Calculates the TRIMP_LT value.
- `POST /hrrs`: Calculates the HRRS value.

### Example of a request.

To calculate the TRIMP value, send a POST request to `/trimp` with the following JSON payload:
``json
{
    “avg_hr": 140.0,
    “max_hr": 190.0,
    “rest_hr": 60.0,
    “workout_duration": 60.0,
    “lt_hr": 170.0
}
```

## Makefile commands

- `make rust-version`: Show the versions of the installed Rust tools.
- `make format`: Format the code.
- `make format-check`: Check code formatting.
- `make lint`: Lint the code.
- `make test`: Run tests.
- `make run`: Run the application.
- `make release`: Build the release version.
- `make build-release`: Build the release version.
- `make all`: Format, lint, test, and run the application.


## License.

This project is distributed under the MIT license. See the [LICENSE](./LICENSE) file for more details.
