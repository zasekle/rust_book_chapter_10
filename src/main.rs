use std::fmt::Display;

fn main() {
    generic_data_types();
    traits_defining_shared_behavior();
}

fn generic_data_types() {

    //Generics can be used in functions. They have more restrictions than something like templates
    // in c++. But this allows the compiler to catch a lot more of the errors. This also requires
    // the PartialOrd trait to be explicitly stated. I assume this restriction is so the compiler
    // can catch errors with generics.
    fn smallest<T: PartialOrd>(one: T, two: T) {
        if one < two {
            println!("less");
        } else {
            println!("greater than or equal");
        }
    }

    smallest(1, 2);
    smallest('a', 'b');

    //Generics can also be used in structs.
    #[derive(Debug)]
    struct Rectangle<T> {
        height: T,
        width: T,
    }

    let rect_one = Rectangle {
        height: 1,
        width: 2,
    };
    let rect_two = Rectangle {
        height: 1.0,
        width: 2.0,
    };

    println!("rect_one: {:?} rect_two: {:?}", rect_one, rect_two);

    //Can use multiple generic type parameters.
    #[derive(Debug)]
    struct Triangle<T, U> {
        base: T,
        height: U,
    }

    let triangle = Triangle {
        base: 1,
        height: 2.0,
    };

    println!("triangle: {:?}", triangle);

    //Can use generics in enums.
    #[derive(Debug)]
    enum PrimaryColors<T> {
        Yellow(T),
        Blue(T),
        Red(T),
    }

    let color = PrimaryColors::Yellow("Or are they RGB?");

    println!("color: {:?}", color);

    //Generics can be used inside implemented methods on structs as well. The <T, U> defined after
    // the impl keyword are the values actually used inside the contained functions. They can be
    // different names than `Triangle` generic names.
    // impl<T, U> Triangle<T, U> {
    //     fn base(&self) -> &T {
    //         &self.base
    //     }
    //
    //     fn height(&self) -> &U {
    //         &self.height
    //     }
    // }

    //The generics can be set to specific types and the functions will only be used in the case
    // that those specific types are used.
    impl Triangle<isize, isize> {
        fn base(&self) -> &isize {
            &self.base
        }

        fn height(&self) -> &isize {
            &self.height
        }
    }

    //As far as the performance of generics, they seem to be determined at compile time and so they
    // don't make the program run any slower. There is a vocabulary word that I have never heard
    // before called `Monomorphization` which seems to mean filling in the types at compile time.
}

fn traits_defining_shared_behavior() {
    //A trait is similar to an interface.

    //An example of a trait.
    pub trait Shape {
        fn area(&self) -> isize;

        //Traits can have a default behavior for a method.
        fn default(&self) {
            println!("default called");
        }
    }

    pub struct Triangle {
        base: isize,
        height: isize,
    }

    pub struct Square {
        height: isize,
    }

    impl Shape for Triangle {
        fn area(&self) -> isize {
            (self.height * self.base)/2
        }

        fn default(&self) {
            println!("Triangle default called.");
        }
    }

    impl Shape for Square {
        fn area(&self) -> isize {
            self.height * self.height
        }
    }

    let my_triangle = Triangle{base: 5, height: 10};
    let my_square = Square{height: 10};

    println!("triangle area: {}", my_triangle.area());
    my_triangle.default();
    println!("square area: {}", my_square.area());
    my_square.default();

    //We cannot do something where we implement an external trait on an external struct. For
    // example the below is now allowed.
    // impl Display for Vec<T>
    // However the below two lines ARE allowed.
    // impl Display for Triangle
    // impl Shape for Vec<T>
    // This rules assures that other people's code cannot break my code and vice versa. Otherwise
    // two crates could implement the same trait for the same type and the compiler wouldn't know
    // which to use.

    //This would be how to use something like polymorphism with a trait.
    pub fn get_area(shape: &impl Shape) {
        println!("Area is {}", shape.area());
    }

    get_area(&my_triangle);
    get_area(&my_square);

    //The above is syntactic sugar for this method itself.
    pub fn get_area_long<T: Shape>(shape: &T) {
        println!("Area is {}", shape.area());
    }

    get_area_long(&my_triangle);
    get_area_long(&my_square);

    //It is possible to specify more than one trait at a time as required for the parameter.
    pub fn get_distance_and_area(shape: &(impl Shape + Display)) {}

    //Can also return a trait.
    pub fn return_area() -> impl Shape {
        Square{height: 15}
    }

    return_area().default();

    //However, cannot return different types. The below code does not compile. Apparently there is
    // a way to make this work, but it won't be covered in the book for a while.
    // pub fn return_area_switch(switch: bool) -> impl Shape {
    //     if switch {
    //         Square{height: 15}
    //     } else {
    //         Triangle{height: 4, base: 1}
    //     }
    // }

    return_area().default();

    //I can do something were I only implement the trait under certain conditions. I will use the
    // example directly from the book for this one.
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    //This will only implement Display if T implements PartialOrd.
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
