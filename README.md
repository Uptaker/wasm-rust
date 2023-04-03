# Rust ja WebAssembly

# Mis on Rust?

Rust on noor programmeerimiskeel - alles tulnud välja viimase kümnendi jooksul. Aastast 2022 juba seitsmendat aastat järjest StackOverflow küsimustiku järgi arendajate kõige armastatuim keel [[1]](https://survey.stackoverflow.co/2022/), Rust on järjest kogunud populaarsust oma elu jooksul.

Rust on üldiselt süsteemiprogrammeerimiskeel - ta on kiire, ohutu mäluhaldusega ning kuna ta on uus, teda ei aeglusta viimase sajandi eel tehtud programmeerimiskeele disaini vead.

### Võimas kompilaator

Rustis täidab kompilaator koodi "valvaja" rolli. Ta tuvastab ära levinuid ja raskesti tabavaid vigu ja keeldub kompileerimast, kui ta neid leidub. Selline lähenemine annab garantii, et mäluga seotud probleemid ja vead on võimatu Rust keeles.

Nii saab arendaja keskenduda süsteemi loogikale ja panustada kiire arendusele ning arendaja suure tõenäosusega ei pea tagantjärgi parandama vigu.

### Lihtne sõltuvuste haldus

Rustiga tuleb kaasa **Cargo** sõltuvuste haldur, mis laseb arendajatel lihtsasti kasutada teiste arendajate Rust liideseid või jagada enda koostatud liides.
[[2]](https://rustwiki.org/en/book/ch00-00-introduction.html)

### Testideks valmis

Rust keelega tuleb kaasa nende enda testimisraamistik. Lihtsad näited tulevad kaasa ka siis, kui tekitada läbi Cargo uue projekti. Nii saavad arendajad lihtsasti sättida ülesse testimise oma integratsioonide töövoogu, hoida koodikvaliteedi kõrgel tasemel ning kindlustada koodi töötavust juhul, kui tekivad suured koodibaasi muudatused.

### Suurettevõtete toetus

Rust keel sai oma vaikse alguse läbi Mozilla arendajatelt. Nüüdseks ajaks Rust keele haldab **Rust Foundation** organisatsioon, mille asutajaliikmeteks on suurettevõtted nagu Google, Microsoft, Huawei, AWS ja Mozilla. [[3]](https://foundation.rust-lang.org/news/2021-02-08-hello-world/)

# Miks Rust koos WebAssemblyga?

Kui vaadata, kuidas WebAssemblyga arendatakse, siis juba mitmendat aastat järjest kõige tihemini arendatakse WebAssembly rakendusi läbi Rusti [[4]](https://blog.scottlogic.com/2022/06/20/state-of-wasm-2022.html).

Mitmed WebAssembly-sisesed tööriistad, nagu WASI (WebAssembly System Interface) on juba kirjutatud Rust keeles [[5]](https://github.com/WebAssembly/WASI). See tähendab, et Rust ei ole võõras WebAssembly maailmas ning naudib head esmaklassilist tuge.

Võrreldes JavaScriptiga, Rusti tüübisüsteem on palju tugevam ja naudib turvalisema arendustsükli oma tüübi süsteemi tõttu.

## Rust WebAssembly eelised

Eelnevalt mainitud tüübisüsteemi eelis on tegelikult olemas ka teistes keeltes. Rustil on hetkel mitu teist eelist konkureerivate keeltega.

### .wasm failid on väiksed

Süsteemi programmeerimiskeeled nagu C, C++ ja Rust ei tule koos enda käivituskeskkonnaga (runtime environment). See tähendab, et võrreldes keeltega nagu Java või C#, siis pakitud **.wasm** fail ei ole nii suur.

Näitena võib tuua Go keele WebAssembly toetust. Kõige väiksem võimalik Go rakenduse `.wasm` fail on hetkel 2MB [[6]](https://github.com/golang/go/wiki/WebAssembly#reducing-the-size-of-wasm-files), kuid lihtne Rust "Hello World" näidisrakendus, mis on genereeritud läbi WebPacki ning on optimeerimata, on ainult 117kB. Teised on aga saavutanud kuni 13kB suuruse faili [[7]](https://dev.to/sahilgarg/why-rust-is-good-for-web-assembly-and-path-to-learning-it-2njf).

### Töötab koos levinud veebitehnoloogia tööriistadega

JavaScript maailmas on populaarseks tööriistaks osutunud WebPack, mis pakib TypeScripti/JavaScripti rakenduse kokku. Rust keelega on lihtne integreerida nende tööriistadega, mis kiirendab ja hõlpsustab arendust. [[8]](https://rustwasm.github.io/book/why-rust-and-webassembly.html)

## Kasutusvõimalused

Rust WebAssembly saab kasutada mitmet moodi. Kasutusvõimalusi saab liigitada kaheks liigiks - terve rakendus on kas osaliselt või täiesti kirjutatud Rustiga. [[9]](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm). Repositoorium ja õpetus keskendub osalise Rust WebAssembly rakenduse ehitamiseks.

### Osaliselt Rustiga

Veebirakenduse ehitamine osaliselt Rustiga tähendab seda, et mingi spetsiifiline osa rakendusest koosneb Rust funktsioonidest. Selline lähenemine on kasulik olemasolevatele veebirakendustele, kus leidub jõudlusega seotud puudusi. Integreerimine sel juhul ei oleks raske, kuid siiski peab ümberkirjutama funktsionaalsust võõras keeles.

Selle tagajärjena on loomulikult see, et rakenduse arendamine, testimine ja üldine töövoog on tehtud keerulisemaks. Õnneks nagu eelmainitud, on Rust võrreldes teiste keeltega lihtsasti integreerud olemasoleva veebitehnoloogia rööriistadega.

### Täieikult Rustiga

On tulnud välja mitu veebiraamistiku Rustile, kus terve kood on kirjutatud täiuslikus ulatuses Rustis. **Yew** on kõige populaarsem Rusti veebiraamistik, kuid on tulnud välja ka teisi, nagu **Leptos**,**Zaplib** ja teised.

Kõik väljatoodud veebiraamistikud põhinevad reaktiivsele paradigmale, mis leidub teistes populaarsetes veebiraamistikes nagu React, Vue ja Svelte - seega. Siin on näide Yew rakenduse komponendist:

```rust
// Allikas - https://yew.rs/docs/0.19.0/tutorial

#[function_component(App)]
fn app() -> Html {
    // ...
-    let videos = videos.iter().map(|video| html! {
-        <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
-    }).collect::<Html>();
-
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
-               { videos }
+               <VideosList videos={videos} />
            </div>
            // ...
        </>
    }
}
```

Nende eelis on ilmselgelt see, et kõik on kirjutatud ühe keelega - kuid selle taga on palju negatiivseid tagajärgi. Reaalsuses on hetkel täielike Rust rakendusi raskem kirjutada. Lisaks, DOM manipulatsiooni tõttu ja toores veebiraamistikudena nad ei ole tootmisvalmis (production-ready) reaalseks maailmaks, kuna tavalised JavaScript rakendused osutuvad olla kiiremad nii tavakasutaja jaoks kui ka arenduses.

# Rust paigaldamine

Rust installeerimiseks kasutame `rustup`. See on Rust paigaldamiseks ametlikult soovitatud viis. [[10]](https://www.rust-lang.org/learn/get-started)


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

# WebAssembly arendus

Järgnevalt tutvume erinevate Rust WebAssembly arendusvõimalustega. On olemas kaks peamist liiki - klassikaline arendus läbi `wasm-pack` tööriista või läbi bundleri, mis tekitab meile arenduseks optimaalse töövoogu.

## 1. Klassikaline arendus - wasm-pack

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

## 2. Arendus läbi bundleri - WebPack

WebPack võimaldab meil saada mugavama arenduskeskkonna läbi asju nagu kiire ümberlaadimise (hot refresh) ja arendusserveri läbi.

Selleks peab olema paigaldatud `node`, mille läbi tuleb `npm` pakettihaldus (package manager)

Genereerime uue Rust WebAssembly näidisprojekti läbi WebPacki:
```bash
npm init rust-webpack webpack && cd webpack
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

2. Lisada järgmine kood `webpack.config.js` faili algusesse [[11]](https://stackoverflow.com/questions/69394632/webpack-build-failing-with-err-ossl-evp-unsupported):
```js
const crypto = require("crypto")
const crypto_orig_createHash = crypto.createHash
crypto.createHash = algorithm => crypto_orig_createHash(algorithm == "md4" ? "sha256" : algorithm)
```

---

Mõlemad näited on saadaval õpetuse repositooriumis.

# Funktsioonide importimine/eksportimine

## Rust koodis

Vaatame järgmist näidisfaili, `main.rs`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn error(err: &str) {
    alert(&format!("Error: please try reloading the page,\n {}", err));
}
```

Algusest pihta on kohe imporditud kõik `wasm_bindgen` funktsioonid. `wasm_bindgen` on üks kõige olulisematest atribuutidest, mis on kasutuses kõikides funktsioonides, mida tahame kas importida või eksportida.

```rust
use wasm_bindgen::prelude::*;
```

Järgmisena, me eksportime lihtsa `add()` funktsiooni.

Enne funktsiooni defineerimist anname talle `#[wasm_bindgen]` atribuuti, kuid see pole veel kõik - peame tegema funktsiooni avatud kõikidele. Selleks kasutame `pub` märksõna funktsiooni sees. See on sarnane `public` funktsioonidega näiteks Java või C# keeles.

```rust
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

Järgmine `alert()` funktsioon teeb vastupidist - me importime funktsiooni JavaScriptist. Selleks teeme teda `extern` funktsiooniks, mis tähistab, et tegemist on välise funktsiooniga (external). Lisaks, peame kopeerima ka tema originaalsed sisendparameetrid.

```rust
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
```

Nüüd saame kutsuda sellesama `alert()` funktsiooni meie Rust koodis. Lisaks, eksportime selle välja, et JavaScript saaks selle kasutada:

```rust
#[wasm_bindgen]
pub fn error(err: &str) {
    alert(&format!("Error: please try reloading the page,\n {}", err));
}
```

Kui arendus käib ilma bundlerita, siis on vaja kompileerida:
```bash
wasm-pack build --target web
```

## JavaScript koodis

Vaatame nüüd JavaScript koodi:

```html
<script type="module">
    import init, {add, error} from './pkg/example.js'

    await init()

    console.log(add(5, 10))

    error('Some error! Oh no!')
</script>
```

WebAssembly funktsioonide väljakutsumiseks peame kasutama JavaScript moodulid, seejärel importime neid funktsioone nimepidi:

```js
import init, {add, error} from './pkg/example.js'
```

`init` on vaja välja kutsuda juba rakenduse alguses - see teeb valmis meie koostatud mooduli kasutamiseks.

```js
await init()
```

Kuna meil on defineeritud ka `add()` ja `alert()` funktsioonid, siis importime neid ja võtame kasutusele.

```js
console.log(add(5, 10))

error('Some error! Oh no!')
```

Nüüd, kui avame veebilehe, siis konsooli väljundisse tekib number 15 ning saame hüppakna, et midagi on läinud valesti.

## Loe rohkem

Materjal loodud järgnevast allika põhjast:

- https://rustwasm.github.io/wasm-pack/book/tutorials/hybrid-applications-with-webpack/using-your-library.html

# Allikad
1. https://survey.stackoverflow.co/2022/
2. https://rustwiki.org/en/book/ch00-00-introduction.html
3. https://foundation.rust-lang.org/news/2021-02-08-hello-world/
4. https://blog.scottlogic.com/2022/06/20/state-of-wasm-2022.html
5. https://blog.scottlogic.com/2022/06/20/state-of-wasm-2022.html
6. https://github.com/golang/go/wiki/WebAssembly#reducing-the-size-of-wasm-files
7. https://dev.to/sahilgarg/why-rust-is-good-for-web-assembly-and-path-to-learning-it-2njf
8. https://rustwasm.github.io/book/why-rust-and-webassembly.html
9. https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
10. https://www.rust-lang.org/learn/get-started
11. https://stackoverflow.com/questions/69394632/webpack-build-failing-with-err-ossl-evp-unsupported
