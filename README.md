# Running_App_Rust

[![Clippy](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/lint.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/lint.yml)
[![Build binary release](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/release.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/release.yml)
[![Rustfmt](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/rustfmt.yml)
[![Tests](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/tests.yml/badge.svg)](https://github.com/AndreCanto00/Running_App_Rust/actions/workflows/tests.yml)

## Descrizione

Running_App_Rust è un'applicazione web scritta in Rust che calcola vari parametri di allenamento come TRIMP (Training Impulse), TRIMP_LT (Lactate Threshold Training Impulse) e HRRS (Heart Rate Recovery Score).

## Installazione

1. Clona il repository:
    ```sh
    git clone https://github.com/AndreCanto00/Running_App_Rust.git
    cd Running_App_Rust
    ```

2. Costruisci il progetto:
    ```sh
    cargo build
    ```

3. Esegui i test:
    ```sh
    cargo test
    ```

## Uso

Per avviare il server:
```sh
cargo run
```

Il server sarà in esecuzione su `http://0.0.0.0:8080`.

### Endpoint

- `GET /`: Restituisce un messaggio di benvenuto.
- `POST /trimp`: Calcola il valore TRIMP.
- `POST /trimp_lt`: Calcola il valore TRIMP_LT.
- `POST /hrrs`: Calcola il valore HRRS.

### Esempio di richiesta

Per calcolare il valore TRIMP, invia una richiesta POST a `/trimp` con il seguente payload JSON:
```json
{
    "avg_hr": 140.0,
    "max_hr": 190.0,
    "rest_hr": 60.0,
    "workout_duration": 60.0,
    "lt_hr": 170.0
}
```

## Comandi Makefile

- `make rust-version`: Mostra le versioni degli strumenti Rust installati.
- `make format`: Format the code.
- `make format-check`: Check code formatting.
- `make lint`: Lint the code.
- `make test`: Run tests.
- `make run`: Run the application.
- `make release`: Build the release version.
- `make build-release`: Build the release version.
- `make all`: Format, lint, test, and run the application.

## Licenza

Questo progetto è distribuito sotto la licenza MIT. Vedi il file [LICENSE](./LICENSE) per maggiori dettagli.