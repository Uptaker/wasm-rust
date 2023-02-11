# Rust ja WebAssembly

# TODO Mis on Rust?

# TODO Miks Rust koos WebAssemblyga?

# Rust paigaldamine

Rust installeerimiseks kasutame `rustup`. See on Rust paigaldamiseks ametlikult soovitatud viis. [(src)](https://www.rust-lang.org/learn/get-started)


### 1. Paigalda Rust

Linux:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Pärast installeerimist võib olla vajalik süsteemi restartida või terminali sees värskendada bashi parameetrid:
```bash
source "$HOME/.cargo/env"
```

Lisaks, tekib mitu terminali tööriista, mida saab kasutusele võtta:

- `rustup` - tööriist Rusti erinevate versioonide haldamiseks
- `rustc` - Rust kompilaator
- `cargo` - tööriist Rust pakettide, sõltuvuste, projektide haldamiseks ja projekti kompileerimiseks läbi `rustc`

### 2. Kontrolli installatsiooni

Teeme uue projekti

```bash
mkdir hello_world && cd hello_world && cargo init
```

Tekib lihtne "Hello World" näidisprogramm. Kompileerime selle:

```bash
cargo run
```

`Cargo` kompileerib uue projekti ning kohe käivitab saadud programmi.

Programmi failid asuvad `target` kaustas ning selle saab käivitada uuesti ka sealt:

```
./target/debug/hello_world
```

Või läbi `cargo` tööriista

```bash
cargo run -q
# -q - ära näita kompileerimise staatust
```

Programmi käivitamisel väljundiks on tekst:
```
Hello, World!
```

**Paigaldatud!**

Kasulikud käsud:
- `cargo run` - Kompileeri ja käivita programmi
- `cargo build` - Kompileeri programmi
- `rustup update` - uuenda Rust. Kuna Rust on kiiresti arenev programmeerimiskeel, siis selle tasub vahepeal käivitada.

### Mida teevad erinevad projekti failid?

**hello_world** kaustas on tekkinud paar faili:
- `Cargo.toml` - konfiguratsioonifail Cargo tööriistale. See on võrreldav NodeJS-põhiste projektide `package.json`-iga.
- `Cargo.lock` - automaatselt koostatud fail pärast kompileerimist. Haldab sõltuvuste versioone ning ei ole mõeldud kasutajapoolsete muutusteks.
- `/src/` - lähtekoodi failid

*Kui kasutusel on git versioonihalduseks, siis soovituslik on panna `**/target` **.gitignore** faili.*

Loe rohkem:
- https://rustup.rs/

# TODO Paidalga WebAssembly




