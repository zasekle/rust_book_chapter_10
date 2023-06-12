use std::fmt::Display;

fn main() {
    generic_data_types();
    traits_defining_shared_behavior();
    validating_references_with_lifetimes();
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


fn validating_references_with_lifetimes() {
    //Every reference has a lifetime, which the the scope for which that reference is valid.
    // lifetimes can be annotated as a method of specifying relationships between lifetimes.
    // I am new to this feature (never seen it in another programming language), and quite frankly
    // the only value I can see to it is in passing a variable through a function without making
    // a copy.

    //Essentially the lifetimes are communicated using the lifetime annotation below. This `says`
    // that there is a relationship between the lifetimes of x, y and the return value.
    fn largest_int<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
        if x > y {
            &x
        } else {
            &y
        }
    }

    let int1 = 1;
    let int2 = 2;

    let res = largest_int(&int1, &int2);
    println!("result: {res}");

    //Because of the above function definition, all lifetimes of the parameters must be valid for
    // the compiler to allow res to be used. For example, the below code will not compile.
    // let int1 = 2;
    // let res;
    // {
    //     let int2 = 1;
    //     res = largest_int(&int1, &int2);
    // }
    // println!("result: {res}");

    //They go through that you cannot return a reference to an object that will be deallocated at
    // the end of a function. But coming from c++ this is the norm so I am not going to focus too
    // much on it.

    //structs also require lifetime annotations in order to hold references.
    struct MyStuff<'a> {
        my_str: &'a str
    }

    let my_str = String::from("the string");
    let stuff = MyStuff{my_str: my_str.as_str()};

    println!("stuff string: {}", stuff.my_str);

    //There are also certain exceptions that happen so frequently the compiler can fill in the
    // lifetime annotations for me. These exceptions are called `lifetime elision rules`.
    // There are currently three rules.

    //Rule #1: The compiler assigns a lifetime parameter to each parameter that's a reference.
    // For example
    // fn foo<'a>(a: &<'a>i32) { ...
    //  can be written as
    // fn foo(a: &i32) { ...

    //Rule #2: If there is exactly one input lifetime parameter, that lifetime is assigned to all
    // output lifetime parameters.
    // For example
    fn single_param(x: &i32) -> &i32 {
        x
    }

    println!("single_param: {}", single_param(&int1));

    //Rule #3: If there are multiple input parameters, but one of them is &self or &mut self
    // (meaning that this is a method) then the lifetime of self is assigned to all output lifetime
    // parameters.

    struct World {
        happy: bool,
        sad: bool,
    }

    impl World {
        fn winner(&self, happiness: &i32) -> &bool {
            if *happiness > 5 {
                &self.happy
            } else {
                &self.sad
            }
        }
    }

    let world = World{happy: true, sad: false};

    let happiness = 10;
    println!("Happy? {}", world.winner(&happiness));

    //As a side note I didn't realise that Rust had different definitions of `Method` and
    // `Function`. I have been using them interchangeably. A Method is a function declared on a
    // struct, enum or trait with the first parameter of self. A function seems to be everything
    // else starting with the `fn` keyword that is not a method.

    //The lifetime annotation for impl must be used if a reference type is stored inside the
    // object.
    impl<'a> MyStuff<'a> {
        fn sad(&self) -> i32 {
            3
        }
    }

    println!("sad: {}", stuff.sad());

    //`'static` is a special lifetime. It allows for the variable to be directly written into the
    // programs binary. When &str is used for a string literal, by default it is set to the static
    // lifetime annotation.
    let hello_world = "Hello world!";
    let static_int: &'static i32 = &123; //A manual example of 'static.

    println!("hello_world: {hello_world} static_int: {static_int}");

    //A lifetime can be used with a generic as follows.
    // fn largest_int<'a, T>(x: &'a i32, y: &'a i32, z: T) -> &'a i32
    // where
    //     T: Display,
    // { ...

}