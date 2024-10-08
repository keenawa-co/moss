#![no_implicit_prelude]

#[derive(::crater::Bundle)]
struct Foo {
    foo: (),
}

#[derive(::crater::Bundle)]
struct Bar<T> {
    foo: T,
}

#[derive(::crater::Bundle)]
struct Baz;

#[derive(::crater::Query)]
struct Quux<'a> {
    foo: &'a (),
}

fn main() {}
