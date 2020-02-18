# Portable Reasoning in Web Assembly

This repository is currently a compilation of my experiments from my internship
at [LIRIS](https://liris.cnrs.fr/) in the team
[TWEAK](https://liris.cnrs.fr/equipe/tweak).

The project I'm working on is to reasoning using a RDF API in Javascript
that resorts to
[the Sophia library written in Rust](https://github.com/pchampin/sophia_rs).

See [Project REPID](https://projet.liris.cnrs.fr/repid/).

As I am still exploring Rust / Web Assembly / tryining to learn web
technologies, this repository does not currently expose a clear tool with a
proper README.

## Sophia interface that matches rdF.js.org specification

The first goal is to enable javascript users to use *Sophia* as a backend.

### How to "use"

*Normally*

- To start a web server that resorts to the Sophia backend :
`./wasm_example/run_server.sh`

*Unit Testing*

I am currently working on integrating Unit Tests

- `npm i @rdfjs/namespace`

- `npm run test`

---

## Temporary1 link heap

1 This word is often a lie

### Iterators / Symbol.Iterator

- https://github.com/rustwasm/wasm-bindgen/issues/1036

- https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Iterator.html



---

## Links that might be useful or not

### Rust / Web Assembly

#### Official Documentation / Links I should stop losing

| Link    | Description |
| ------- | ----------- |
| https://doc.rust-lang.org/book/ | Rust book |
| https://doc.rust-lang.org/std/ | `std` intensifies |
| https://rustwasm.github.io/docs/wasm-bindgen/ | wasm_bindgen |



#### Wasm bindgen

| Link    | Description |
| ------- | ----------- |
| https://dev.to/sendilkumarn/rust-and-webassembly-for-the-masses-wasm-bindgen-57fl | A basic tutorial on wasm_bindgen |
| https://rustwasm.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html | A potential way to make the code faster |
| https://rustwasm.github.io/wasm-bi:bindgen/reference/cli.html | We are actually not supposed to use the command wasm_bindgen but wasm-pack |
| https://github.com/rustwasm/wasm-pack | wasm pack repository |

#### Other documentation

| Link    | Description |
| ------- | ----------- |
| https://play.rust-lang.org/ | Rust Online Compiler. Doesn't offer suggestion |
| https://learnxinyminutes.com/docs/rust/ | Could be used as a quick cheatsheet but I prefer using the documentation with the search feature |
| https://github.com/pchampin/rust-iut/ | A Rust course teached to 2 years students. |



### RDF


| Link    | Description |
| ------- | ----------- |
| https://www.w3.org/TeamSubmission/turtle/ | Turtle Spec |
| https://github.com/rubensworks/jest-rdf   | How to build a Jest test suite for RDF |
| https://www.w3.org/community/rdfjs/       | RDF JS Work group |
| http://iswc2011.semanticweb.org/fileadmin/iswc/Papers/Workshops/SSWS/Emmons-et-all-SSWS2011.pdf | Article on RDF Literal Data Types |

### RDF JS interface


|   Title    |            Specification            |           Basic implementation           |
| ---------- | ----------------------------------- | ---------------------------------------- |
| Data Model | https://rdf.js.org/data-model-spec/ | https://github.com/rdfjs-base/data-model |
| Dataset    | https://rdf.js.org/dataset-spec/    | https://github.com/rdfjs-base/dataset    |
| Stream     | https://rdf.js.org/stream-spec/     |                                          |



### Internship

- https://projet.liris.cnrs.fr/repid/2019/stage-raisonnement-portable/fr/

- HyLAR : https://github.com/ucbl/HyLAR-Reasoner#readme
    - https://hal.archives-ouvertes.fr/hal-01154549/file/hylar.pdf
    - https://hal.archives-ouvertes.fr/hal-01276558/file/Demo_www2016.pdf
    - http://mmrissa.perso.univ-pau.fr/pub/Accepted-papers/2018-TheWebConf-RoD.pdf

- Sophia :
    - Master : https://github.com/pchampin/sophia_rs
    - Clone : https://github.com/bruju/sophia_rs

- wasm_example : https://github.com/davidavzP/wasm_example

- rflib.js : http://linkeddata.github.io/rdflib.js/doc/

### Other people that do related things


| Link | Description |
| ---- | ----------- |
| https://github.com/Tpt/oxigraph/ | Another rdf implementation in rust : Oxygraph |
| https://karthikkaranth.me/blog/my-experience-with-rust-plus-wasm/ | A feedback on rust + wasm |

---

## Rust snippets / Observations


> Condition on genetic terms

```rust
impl BJQuad {
    pub fn new<T>(cloned_quad: &T) -> BJQuad 
    where T: Graph<TermData = RcTerm> {
        /* blabla */
    }
}
```

or

```rust
fn toto<T>(...) where
T : Graph + Foo,
  <T as Graph>::TermData
```

To answer the question "where does TermData comes from ?"


> As Rust is expression oriented, we can always refactor code like this

```rust
    pub fn quad(&self, subject: JssTerm, predicate: JssTerm, object: JssTerm, graph: Option<JssTerm>) -> BJQuad {
        match graph {
            None => BJQuad::new_by_move(
                build_rcterm_from_jss_term(&subject).unwrap(),
                build_rcterm_from_jss_term(&predicate).unwrap(),
                build_rcterm_from_jss_term(&object).unwrap(),
                None
            ),
            Some(g) => BJQuad::new_by_move(
                build_rcterm_from_jss_term(&subject).unwrap(),
                build_rcterm_from_jss_term(&predicate).unwrap(),
                build_rcterm_from_jss_term(&object).unwrap(),
                build_rcterm_from_jss_term(&graph));
            )
        }
    }
```

into

```rust
    pub fn quad(&self, subject: JssTerm, predicate: JssTerm, object: JssTerm, graph: Option<JssTerm>) -> BJQuad {
        BJQuad::new_by_move(
            build_rcterm_from_jss_term(&subject).unwrap(),
            build_rcterm_from_jss_term(&predicate).unwrap(),
            build_rcterm_from_jss_term(&object).unwrap(),
            match graph {
                None => None,
                Some(g) => build_rcterm_from_jss_term(&g)
            }
        )
    }
```

> I would like to optimize the operations on JsImport if the received object is
a RustExport

> We could also change the type of dataset depending on if we are currently
are doing a lot of adds or a lot of matches

> In some function, I'd like to return self to let the user chain its call

*Problem* : wasm_bindgen can't return references.

*Doesn't work* :

- `pub fn add(self) -> MyClass { self }` because if we don't assign we lose
the instance

- `pub fn add(self) -> *MyClass { *self }` returns a number that have no
sense for the user and that he can't use.

*Bad* ;

- Python script that modifies the generated javascript code : we can just
manually add the `return this;` in the ~4 functions that requires it
