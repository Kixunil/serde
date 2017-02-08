#[macro_use]
extern crate serde_derive;

#[derive(Serialize)] //~ ERROR: proc-macro derive panicked
struct S {
    #[serde(rename(serialize="x", serialize="y"))] //~^^ HELP: duplicate serde attribute `rename`
    x: (),
}

fn main() {}
