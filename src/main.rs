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

}
