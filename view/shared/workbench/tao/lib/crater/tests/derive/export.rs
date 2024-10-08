mod inner {
    use crater::{Bundle, Query};

    #[derive(Bundle)]
    pub struct Foo;

    #[derive(Query)]
    pub struct Bar<'a> {
        foo: &'a i32,
    }
}

type Foo = inner::Foo;
type Bar<'a> = inner::Bar<'a>;

fn main() {}
