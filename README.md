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
- `/src` - lähtekoodi failid
- `/target` - kompileeritud failid

*Kui kasutusel on git versioonihalduseks, siis soovituslik on panna `**/target` **.gitignore** faili.*

Loe rohkem:
- https://rustup.rs/

# Paigaldame WebAssembly teegid

Rusti WebAssembly funktsioone saab mitmet moodi käsitleda. Kõige klassikaline töövoog on tekitada uue Rust moodulteeki (library).


Enne seda, peame installima [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) tööriista.

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

`wasm-pack` aitab teil luua Rustiga loodud WebAssembly raamistikke. Sellega tuleb kaasa ka `wasm-bindgen` sõtluvus, mis aitab meil nii importida Rusti kompileeritud koodi brauserisse kui ka eksportida JavaScript funktsioone.


Järgmisena, tekima uue Rust moodulteeki kasutades `--lib` parameetri:
```bash
cargo init --name add_example --lib && cd test
```


Lisame `wasm-bindgen` sõltuvuse meie `Cargo.toml` faili ning ütleme kompilaatorile, et tekitame dünaamilise süsteemi teeki/mooduli kasutades `cdylib`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.84" # Versiooni saab siit: https://crates.io/crates/wasm-bindgen
```

Meie `lib.rs` failis kasutame juba olemasoleva koodinäidet, kuid me lisame sinna `wasm_bindgen` makrot, et genereeritud funktsiooni oleks võimalik importida.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

Seejärel terminalis kasutame `wasm-pack` tööriista, et kompileerida veebiraamistikuna:
```bash
wasm-pack build --target web
```

Nüüdseks peab tekkima ma uus `pkg` kaust. JavaScript ja TypeScript arendajatele tulevad ette tuttavad failid, nagu `package.json` ja TypeScript definitsiooni failid arenduseks. Lisaks tekib ka meie `.wasm` laiendiga kompileeritud binaarne fail, mida saab käivitada brauseris.

Laadime WASM mooduli JavaScriptist. Selleks tekimate uue `index.html` faili järgneva sisuga:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WASM-Rust</title>
</head>
<body>
    <script type="module">
        import init, {add} from './pkg/add_example.js'

        await init()

        console.log(add(5, 10))
    </script>
</body>
</html>
```

Serveerime `index.html` läbi HTTP serveri. Selleks sobib ükskõik mis server. Lihtsuse pärast saab kasutada ka Python http serveri (Python3 olemasolul):

```bash
python3 -m http.server

# serveerib localhost:8000
```

Kui avad serveeritud HTML faili, siis konsooliaknas peab olema '15' väljund.

**Seega kõik toimub!**

# Arendus läbi WebPacki

WebPack võimaldab meil saada mugavama arenduskeskkonna läbi asju nagu kiire ümberlaadimise (hot refresh) ja arendusserveri läbi.

Selleks peab olema paigaldatud `node`, mille läbi tuleb `npm` pakettihaldus (package manager)

Genereerime uue Rust WebAssembly näidisprojekti läbi WebPacki:
```bash
cd webpack && npm init rust-webpack webpack
```

Paneme käima arenduserveri:
```bash
npm start
```

Avades brauseri konsooli peaksid nägema sõnumi: "Hello world!".

**See tähendab, et kõik toimis!** - uued projektid saab koostada samal viisil.

---
*Mis teha, kui tuleb `digital envelope routines::unsupported` viga?*

Vea lahenduseks on kaks võimalust:

1. Installida vanem Node 16 versioon

või

2. Lisada järgmine kood `webpack.config.js` faili algusesse:
```js
const crypto = require("crypto");
const crypto_orig_createHash = crypto.createHash;
crypto.createHash = algorithm => crypto_orig_createHash(algorithm == "md4" ? "sha256" : algorithm);
```

[(src)](https://rustwasm.github.io/wasm-pack/book/tutorials/hybrid-applications-with-webpack/using-your-library.html)




