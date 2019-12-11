enum Simple {
    First,
    Second,
    Third,
}

fn print_simple(simple: Simple) {
    match simple {
        Simple::First => println!("First"),
        Simple::Second => println!("Second"),
        Simple::Third => println!("Third"),
    }
}

enum Complex {
    First(u32),
    Second(f64),
}

fn print_complex(complex: Complex) {
    match complex {
        Complex::First(val) => println!("First: {}", val),
        Complex::Second(val) => println!("Second: {}", val),
    }
}

#[derive(Copy, Clone)]
enum Generic<T> {
    Foo(T),
    Bar,
}

trait Show: Copy {
    fn show(self);
}

impl<T: Show> Show for Generic<T> {
    fn show(self) {
        match self {
            Generic::Foo(foo) => {
                print!("Foo: ");
                foo.show();
                println!();
            },
            Generic::Bar => println!("Bar"),
        }
    }
}

impl Show for u32 {
    fn show(self) {
        println!("u32: {}", self);
    }
}

fn main() {
    print_simple(Simple::First);
    print_complex(Complex::Second(6.15));
    let generic = Generic::Foo(42);
    generic.show();

    // Look at error handling

}
