#![forbid(unsafe_code)]

use {
    ::core::{
        cell::RefCell,
    },
    ::lending_iterator::{
        higher_kinded_types::{HKT, Apply},
    },
};

struct Person {
    name: String,
    surname: String,
    age: u8,
}

impl Person {
    fn full_name (self: &'_ Person)
      -> String
    {
        format!(
            "{}{sep}{}",
            self.name,
            self.surname,
            sep = if self.name.is_empty() { "" } else { " " },
        )
    }

    fn name (self: &'_ Person)
      -> ::std::borrow::Cow<'_, str>
    {
        if self.name.is_empty() {
            format!("Mr/Ms {}", self.surname).into()
        } else {
            self.name.as_str().into()
        }
    }
}

fn debug_each<R : HKT, F> (
    elems: &'_ [RefCell<Person>],
    f: F,
)
where
    F : Fn(&'_ Person) -> Apply!(R<'_>),
    for<'any>
        Apply!(R<'any>) : ::core::fmt::Debug
    ,
{
    elems
        .iter()
        .for_each(|refcell: &'_ RefCell<Person>| {
            let guard: ::core::cell::Ref<'_, Person> = refcell.borrow();
            let person: &'_ Person = &*guard;
            let to_debug: Apply!(R<'_>) = f(person);
            eprintln!("{:?}", to_debug);
        })
}

fn main ()
{
    let array = [
        RefCell::new(Person {
            name: "".into(),
            surname: "Globby".into(),
            age: 0xff,
        }),
    ];
    let elems = &array[..];

    // OK
    debug_each::<HKT!(u8), _>(
        elems,
        |person: &'_ Person| -> u8 {
            person.age
        },
    );

    // OK
    debug_each::<HKT!(String), _>(
        elems,
        Person::full_name,
    );

    // OK
    debug_each::<HKT!(::std::borrow::Cow<'_, str>), _>(
        elems,
        Person::name,
    );

    // OK as well!
    debug_each::<HKT!(&str), _>(
        elems,
        |person: &Person| -> &str {
            &person.surname
        },
    );
}
