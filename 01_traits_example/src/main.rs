use std::fmt;

/// Use verbs for trait names by convention
trait Scream: Copy {
    /// If the trait has a single method, then it's usually named same as the trait.
    fn scream(self);
}

impl Scream for u32 {
    fn scream(self) {
        println!("I AM u32 AND MY VALUE IS {}!!!", self);
    }
}

impl Scream for f64 {
    fn scream(self) {
        println!("I AM f64 AND MY VALUE IS {}!!!", self);
    }
}

fn scream_three_times<T: Scream>(value: T) {
    value.scream();
    Scream::scream(value);
    <T as Scream>::scream(value);
}

#[derive(Copy, Clone)]
struct MyType {
    foo: u32,
}

impl Scream for MyType {
    fn scream(self) {
        println!("I AM MyType AND FOO IS {}!!!", self.foo);
    }
}

fn show_value<T: fmt::Display>(value: T) {
    println!("The value is: {}", value)
}

struct Point2D {
    x: f64,
    y: f64,
}

struct Vector2D {
    x: f64,
    y: f64,
}

impl std::ops::Add<Vector2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Vector2D;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() {
    let my_var = MyType { foo: 47 };

    scream_three_times(6.15);
    scream_three_times(my_var);
    show_value(42);
    show_value(my_var);

    let point = Point2D {
        x: 42.0,
        y: 47.0,
    };

    let vector = Vector2D {
        x: 5.0,
        y: 1.337,
    };

    let moved = point + vector;

    println!("[{}, {}]", moved.x, moved.y);
}


// impl fmt::Display for MyType  is written bellow, ignore it for now :)

































































impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I have {} foos", self.foo)
    }
}
