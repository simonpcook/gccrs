// { dg-output "Foo::A\nFoo::B\nFoo::C x\nFoo::D 20 80\n" }
extern "C" {
    fn printf(s: *const i8, ...);
}

enum Foo {
    A,
    B,
    C(char),
    D { x: i64, y: i64 },
}

fn inspect(f: Foo) {
    match f {
        Foo::A => unsafe {
            let a = "Foo::A\n\0";
            let b = a as *const str;
            let c = b as *const i8;

            printf(c);
        },
        Foo::B => unsafe {
            let a = "Foo::B\n\0";
            let b = a as *const str;
            let c = b as *const i8;

            printf(c);
        },
        Foo::C(x) => unsafe {
            let a = "Foo::C %c\n\0";
            let b = a as *const str;
            let c = b as *const i8;

            printf(c, x);
        },
        Foo::D { x, y } => unsafe {
            let a = "Foo::D %i %i\n\0";
            let b = a as *const str;
            let c = b as *const i8;

            printf(c, x, y);
        },
    }
}

fn main() -> i32 {
    let a = Foo::A;
    let b = Foo::B;
    let c = Foo::C('x');
    let d = Foo::D { x: 20, y: 80 };

    inspect(a);
    inspect(b);
    inspect(c);
    inspect(d);

    0
}
