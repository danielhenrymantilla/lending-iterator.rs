
## The what, the why and the how of Higher Kinded Types (HKTs)

  - Feel free to also check up <https://docs.rs/polonius-the-crab>'s extensive
    documentation, since it also has to deal with Higher-Kinded Types (even
    though, in practice, it hides them under the ~~rug~~ macros 🙃)

### What are Higher-Kinded Types?

<details><summary>Click to see</summary>

A higher-kinded type is an actual / **full / standalone** type which is, itself,
"generic", or rather, to which we can further feed generic parameters (such as
lifetime parameters or type parameters) to obtain further types.

  - [ ] "is generic" / can be fed generic parameters to construct a type ❓
  - [ ] is a type in and of itself ❓
      - For instance, `type Standalone = YourHktType;` has to compile.

One way to illustrate this difference, for instance, would be to consider:

```rust
use ::lending_iterator::higher_kinded_types::HKT;

type StringRefNaïve<'lt> = &'lt str;
// and
type StringRef = HKT!(<'lt> => &'lt str);
```

Both `StringRefNaïve` and `StringRef` can be fed a generic parameter (in this
instance, a lifetime parameter) so as to get or construct a type:

```rust
use ::lending_iterator::higher_kinded_types::{Feed, HKT};

# type StringRefNaïve<'lt> = &'lt str;
# type StringRef = HKT!(<'lt> => &'lt str);
#
const _: StringRefNaïve<'static> = "This is a `&'static str`";
const _: Feed<'static, StringRef> = "This is a `&'static str`";
```

  - [x] "is generic" / can be fed generic parameters to construct a type ✅

But what of:

  - [ ] is a type in and of itself ❓

Well, while `StringRef` is indeed a standalone type:

```rust
use ::lending_iterator::higher_kinded_types::HKT;

type StringRef = HKT!(<'lt> => &'lt str);

type Standalone = StringRef; // ✅
```

it turns out that `StringRefNaïve` is not:

```rust ,compile_fail
use ::lending_iterator::higher_kinded_types::HKT;

type StringRefNaïve<'lt> = &'lt str;

type Standalone = StringRefNaïve; // ❌ Error
```

This errors with:

```console
error[E0106]: missing lifetime specifier
 --> src/higher_kinded_types.rs:70:19
  |
8 | type Standalone = StringRefNaïve; // ❌ Error
  |                   ^^^^^^^^^^^^^^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
8 | type Standalone<'a> = StringRefNaïve<'a>; // ❌ Error
  |                ++++   ~~~~~~~~~~~~~~~~~~
```

That is, in Rust **a generic "type" is actually not a type**. It's just a path
(grammar-wise), a name, to which we can feed the generic parameters so as to
obtain types in return.

A HKT would be the proper solution to this: not only can such an "entity" be
fed generic parameters (thence "acting like" a generic "type" above), it can
also _not be fed any parameters and still be a type_. That is,

> <span style="font-size: large;">a HKT is an _actual_ **type** which is generic / can be fed parameters.</span>

Another definition, which will make more sense in the following section, is that
HKTs come into play the moment we need "generic generics".


</details>

### Why? What are Higher-Kinded Types _for_?

<details><summary>Click to see</summary>

  - #### Type-HKTs

    <details open><summary>Click to hide</summary>

    Consider the following pseudo-code:

    ```rust ,ignore
    struct StructOfArrays<ArrayKind : <T>> {
        array_of_i32s: ArrayKind<i32>,
        array_of_strings: ArrayKind<String>,
    }

    /** ```rust
    struct StructOfVecs {
        array_of_i32s: Vec<i32>,
        array_of_strings: Vec<String>,
    }
    ``` */
    type StructOfVecs = StructOfArrays<Vec>;

    /** ```rust
    struct StructOfVecDeques {
        array_of_i32s: VecDeque<i32>,
        array_of_strings: VecDeque<String>,
    }
    ``` */
    type StructOfVecDeques = StructOfArrays<VecDeque>;
    ```

    Within this pseudo-code, `<ArrayKind>` is a generic parameter, _but one
    which would be able to be fed a generic type parameter `<T>` itself_, so as
    to obtain each field type. So, we have a generic type parameter,
    `<ArrayKind>`, which is expected to be, itself, `<T>`-generic as well!

    Thence the term a "generic generic" (parameter).

    And we could push that flexibility even further with idea of being able to
    produce and feed _ad-hoc_ / on-demand "generic **types**" to these generic
    structs. Something along the lines of:

    - ```rust ,ignore
      /** ```rust
      struct StructOfPairs {
          array_of_i32s: [i32; 2],
          array_of_strings: [String; 2],
      }
      ``` */
      type StructOfPairs = StructOfArrays< HKT!(<T> => [T; 2]) >;
      ```

    In some cases, that level of genericity can lead to very s(l)ick designs.

    The example above was kind of contrived, but a more classic need would be to
    try and be generic over the thread-safety of a reference-counted pointer:

    ```rust ,ignore
    type Arc = HKT!(<T> => ::std::sync::Arc<T>);
    type Rc = HKT!(<T> => ::std::rc::Rc<T>);

    type MyHandle<RefCountedPtr : <T>>(
        RefCounterPtr<Inner>,
    );

    type MyHandleFast = MyHandle<Rc>;
    type MyHandleSync = MyHandle<Arc>;
    ```

    </details>

  - #### Lifetime HKTs

    <details open><summary>Click to hide</summary>

    Another use case can be around lifetimes, when dealing with higher-order
    lifetimes (_e.g._, when exposing borrows of callee local variables to a
    caller-chosen generic parameter).

    To illustrate, let's consider the following example:

    First the following type definition:

    ```rust
    struct Person {
        name: String,
        surname: String,
        age: u8,
    }
    ```

    and now consider an API being able to locally produce a borrow to a
    `Person`, (by _locally_ it is meant that such borrow cannot escape the
    function call / is tied to the _callee_), which thus requires some form of
    callback:

    ```rust
    use ::core::cell::RefCell;
    # struct Person { name: String, surname: String, age: u8 }

    fn for_each (
        elems: &[RefCell<Person>],
        // this could be `FnMut`, but let's stick to `Fn` for the sake of simplicity.
        f: impl Fn(&Person),
    )
    {
        elems
            .iter()
            .for_each(|refcell| {
                let local = refcell.borrow();
                f(&*local);
            })
    }
    ```

    Now, let's spice things a little. For starters, let's consider that rather
    than a `Fn(&Person) /* -> () */` kind of callback, we're gonna expect the
    user to _map_ to one of its fields or dependent data so that we can `Debug`
    it:

    ```rust
    use ::core::{
        cell::RefCell,
        fmt::Debug,
    };
    # struct Person { name: String, surname: String, age: u8 }

    fn debug_each<R> (
        elems: &[RefCell<Person>],
        f: impl Fn(&Person) -> R,
    )
    where
        R : Debug,
    {
        elems
            .iter()
            .for_each(|refcell| {
                let local = refcell.borrow();
                let field_to_debug = f(&*local);
                eprintln!("{:?}", field_to_debug);
            })
    }
    ```

    With this,

    ```rust ,ignore
    debug_each(elems, |person: &Person| -> u8 { person.age });
    ```

    works Just Fine™.

    But what of:

    ```rust ,compile_fail
    debug_each(elems, |person: &Person| -> &str { person.name });
    ```

    This will fail with a bunch of "conflicting lifetime requirements" error
    messages; we are now having to deal with higher-order lifetimes!

    The gist of the issue is that here, we'd like to say that `R = &str`, right?

    But such statement is wave-handed and overlooking a crucial aspect here:
    **what is the lifetime in that `&str`?**

    It turns out that we can't really answer it: if we unsugar the `impl Fn`
    required signature, we have:

    ```rust ,ignore
    impl Fn(&Person) -> R
    // is the same as:
    impl for<'local> Fn(&'local Person) -> R
    ```

    and so we would have liked to pick `R = &'local str`.

    Let's see the function signature that would have resulted from that:

    ```rust ,ignore
    fn debug_each<R /* = &'local str */> (
        elems: &[RefCell<Person>],
        f: impl for<'local> Fn(&'local Person) -> R,
    )
    where
        R : Debug,
    ```

    Can you spot the issue?

    `'local` is not in scope when picking `R`!

      - If you have followed the HKT introduction above, you may now start
        to see where I am going: we'd like to have a `R : <'lt>` HKT-y
        type (`R = HKT!(<'lt> => &'lt str)`), so as to then use:

        ```rust ,ignore
        f: impl for<'local> Fn(&'local Person) -> R<'local>
        ```

        But you may consider this overly convoluted / overkill.

    Indeed, there is a simpler way to make the signature compile: "just" slap a
    `&` in front of that `R` in the return type of the closure!

    ```rust ,ignore
    fn debug_each<R : ?Sized /* = str */> (
        elems: &'_ [RefCell<Person>],
        f: impl for<'local> Fn(&'local Person) -> &'local R,
    )
    where
        R : Debug,
    ```

    which, for better or for worse, can be further reduced down to:

    ```rust ,ignore
    fn debug_each<R : ?Sized /* = str */> (
        elems: &[RefCell<Person>],
        f: impl Fn(&Person) -> &R,
    )
    where
        R : Debug,
    ```

    And when doing so, then yes,

    ```rust ,ignore
    |person: &Person| -> &str { &person.name }
    ```

    will be a valid callback to feed to `debug_each` ✅

    Except… that our previous:

    ```rust ,compile_fail
    debug_each(elems, |person: &Person| -> u8 { person.age });
    ```

    doesn't compile anymore!

    Easy —you may say— use `-> &u8` instead! (and `&person.age`)

    And okay, that will work, but at this point we should start noticing that
    Rust is now dictating the rules, slowly narrowing down our intended API.

    And this narrowing is no small thing. Indeed, now let's consider that
    `Person` has some fancy getters:

    ```rust
    use ::std::borrow::Cow;

    pub
    struct Person {
        name: String,
        surname: String,
        age: u8,
    }

    impl Person {
        /// Case 1: getter that returns something _owned_.
        pub
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

        /// Case 2: getter that returns a borrowing / dependent type which is
        /// not exactly a Rust reference.
        pub
        fn name_not_empty (self: &'_ Person)
          -> Cow<'_, str>
        {
            if self.name.is_empty() {
                format!("Mr/Ms {}", self.surname).into()
            } else {
                self.name.as_str().into()
            }
        }
    }
    ```

    Now try to use either of `person.full_name()` or `person.name_not_empty()`
    with this `debug_each()` API. You'll see that "slap a `&` on the return
    type" approach no longer works, you'll get "borrow of dropped temporary"
    errors.

    Granted, you _could_ duplicate `debug_each` into a `-> R` API, and a `-> &R`
    API (let's call the latter `debug_each_ref`) —which incidentally is
    something that does happen in the Rust ecosystem. The other option is to
    forgo that `-> &R` one, and require callers to `.clone()` stuff to meet the
    `-> R` requirement.

    <span style="font-size: x-large;">😕</span>

    Neither of these things is satisfactory. We'd like to be _generic_ over all
    the possible return types, whether they be borrowing/dependent or not, and
    whether they be exactly a Rust reference or a more complex type such as a
    `Cow`.

    Hence the need for HKTs, here.

    The fully generic API would then thus be:

    ```rust ,ignore
    //! In pseudo-code
    fn debug_each<R : <'lt>>(
        elems: &[RefCell<Person>],
        f: impl for<'local> Fn(&'local Person) -> R<'local>,
    )
    where
        for<'lt>
            R<'lt> : Debug
        ,
    ```

    A fully working example of this API using HKTs will be showcased below, once
    we've tackled replacing the pseudo-code with actual Rust.

    </details>

</details>

## Back to real Rust

<details><summary>Click to see</summary>

This, in real Rust, comes with three challenges:

  - Expressing that `ArrayKind : <T>` constraint. In other words, encoding
    the `<T>`-ness property into a trait.

    ```rust ,ignore
    trait HKT : /* magic */ { /* magic */ }
    ```

  - Applying / feeding a `<T>` type parameter to it to query the resulting
    type.

    Given our base idea of using a trait, this last aspect will involve
    querying an associated type; either through a generic helper trait, or
    with the associated type being, itself, generic:

      - With `generic_associated_types`:

         1. We could envision using:
            ```rust ,ignore
            trait TypeHKT { type Assoc<T>; }
            ```

         1. And then the querying of the type would be done with:
            ```rust ,ignore
            <ArrayKind as TypeHKT>::Assoc<T>
            ```

        for the `<T>` type-HKT case, and, similarly,

         1. ```rust ,ignore
            trait HKT { type Assoc<'lt>; }
            ```

         1. ```rust ,ignore
            <R as HKT>::Assoc<'local>
            ```

        for the `<'lt>` lifetime-HKT case.

      - Without `generic_associated_types`:

        Alas, **there is no way to express the `<T>`-over-types HKT-ness**.

        But the good news is that the `<'lt>`-over-lifetimes HKT-ness can still
        be expressed, since [lifetime GATs can be emulated in stable Rust](
        https://docs.rs/nougat):

         1. We define:
            ```rust ,ignore
            trait WithLifetime<'lt> { type Assoc; }
            ```

            which encodes a _single_ `'lt => Self::Assoc` mapping for the `Self`
            HKT.

         1. And then we alias:

            ```rust ,ignore
            trait HKT = for<'any> WithLifetime<'any>;
            ```

            to encode the idea of having the aforementioned mapping exist `for`
            "all" / `'any` possible choice of the `'lt`.

              - This `for`all / universal quantification of a trait bound is the
                magic that allows us to express the same as a GAT. In other
                words, GATs, can be viewed as "just" sugar for a universal trait
                quantification.

                Since `for<'lt>` is expressible in stable Rust, lifetime GATs
                can be emulated in stable Rust, and thus, lifetime-HKTs too, as
                showcased by this very post.

                But since `for<T>` is not yet a thing, neither type GATs nor
                type HKTs can be expressed in stable Rust 😔.

         1. Querying the type is then done with:

            ```rust ,ignore
            <Type as WithLifetime<'lt>>::Assoc
            ```

  - Providing implementors or implementations of that trait:

      - neither `Vec` nor `VecDeque` are, _alone_, types.
        They're "syntactical type paths" which can be fed a type parameter to
        then refer to one of the `Vec{,Deque}<T>` types.

          - See the previous sections for more info about this.

      - So we'll need to define _ad-hoc_ implementors of this genericity, as
        hinted by the aforementioned `HKT!(<T> => [T; 2])` example syntax.

    So, while it would be possible to manually implement:

      - the `generic_associated_types`-based trait:

        `impl {Type,}HKT for … { type Assoc<…> = …; }`,

      - or even the without-`generic_associated_types` polyfill:

        `impl<'lt> WithLifetime<'lt> for … { type Assoc = …; }`

    , the truth is that we don't necessarily need to write all these
    implementations if we are able to somehow magically produce appropriate
    type implementors "on demand" (in an _ad-hoc_ fashion).

    And it turns out there is! `dyn` to the rescue!

    Indeed, `dyn Trait<…>` is a standalone / "on demand"-queryable type,
    which does implement `Trait<…>`.

    From here, we come up with `dyn for<T> TypeHKT<Assoc<T> = …>` in the general
    case, and `dyn for<'lt> WithLifetime<'lt, Assoc = …>` in the polyfilled
    case.

    #### A convenience macro shorthand

    Since remembering it all is hard, writing it in full, cumbersome, and
    reading it, noisy, this crate offers a convenience macro shorthand:

    ```rust
    use ::lending_iterator::higher_kinded_types::HKT;

    type StringRef = HKT!(<'lt> => &'lt str);
    ```

    More on this below.

### The HKT API of this crate

This crate needs HKTs to express some of the APIs involved with the iterator
adaptors.

Given that `'next` lifetime involved in the key signature of a
`LendingIterator`:

```rust ,ignore
trait LendingIterator {
    type Item<'next>
    where
        Self : 'next,
    ;

    fn next<'next> (
        self: &'next mut Self,
    ) -> Self::Item<'next>
    ;
}
```

it's easy to guess that we'll be dealing with `<'next>`-one-lifetime-generic
kind of HKTs.

And luckily, this is the one expressible in stable Rust:

```rust ,ignore
trait WithLifetime<'lt> {
    type Assoc;
}
trait HKT = for<'any> WithLifetime<'any>;
```

From there, here are the following key ideas to keep in mind when using this
crate.

</details>

# The three important things to work with these HKT APIs

<details open><summary>Click to hide</summary>

 1. **APIs use `T : HKT` to express `T : <'lt>`**

    So, back to our aforementioned `debug_each` example API, that `R : <'lt>`
    bound would be expressed using `R : HKT`:

    ```rust ,ignore
    fn debug_each<R : HKT> (
    # /*
        …
    # */ )
    ```

 1. **Given a `R : HKT` type, use <code>[Feed]\<\'lt, R\></code> or
    <code>[Apply!]\(R\<\'lt\>\)</code> to feed it a lifetime** `'lt`.

    ```rust ,ignore
    fn debug_each<R : HKT> (
        elems: &[RefCell<Person>],
        f: impl for<'local> Fn(&'local Person) -> Apply!(R<'local>),
        // or:
        f: impl Fn(&Person) -> Apply!(R<'_>),
    )
    where
        for<'any>
            Apply!(R<'any>) : Debug
        ,
    ```

    For those curious, <code>[Apply!]\(R\<\'lt\>\)</code> is just sugar for
     <code>[Feed]\<\'lt, R\></code>, which in turn is an alias for:

    ```rust ,ignore
    <R as WithLifetime<'lt>>::T
    ```

 1. And last but totally not least,

    **use <code>[HKT!]\(\<\'lt\> =\> Type…\)</code> to define and provide an
    ad-hoc HKT / generic-lifetime-to-type association**.

    ```rust ,ignore
    debug_each::<HKT!(<'local> => &'local str)>(
    # /*
        …
    # */)
    ```

      - 💡 The macro supports lifetime elision rules: you can directly feed a
        type with elided lifetimes, as in <code>[HKT!]\(Type\<\'_\>\)</code>,
        and the macro will automagically replace it with
        `HKT!(<'a> => Type<'a>)` 💡

    ```rust ,ignore
    // This works too!
    debug_each::<HKT!(&str)>( // or `HKT!(&'_ str)`.
    # /*
        …
    # */)
    ```

      - Note that nothing requires that these `HKT!` invocations be inlined in
        their turbofished sites; instead, you can easily define type aliases
        using them:

        ```rust ,ignore
        type HKTRefStr = HKT!(<'lt> => &'lt str);

        debug_each::<HKTRefStr>(
        # /*
            …
        # */)
        ```

        or even make the HKT, itself, be generic!

        ```rust ,ignore
        type HKTRef<T /* : ?Sized */> = HKT!(<'lt> => &'lt T);

        debug_each::<HKTRef<str>>(
        # /*
            …
        # */)
        ```

    For those curious, the [`HKT!`] macro expands to a
    <code>dyn for\<\'lt\> [WithLifetime]\<\'lt, T = …\></code> type, but wrapped
    in a `PhantomData` (thanks to a blanket impl on them), so as to be `Sized`
    (thus allowing the callees / the called APIs to skip the noisy `?Sized`
    unbounds on already heavy signatures).

</details>

## Illustration: fully working code for the `debug_each` example

<details><summary>Click to see</summary>

```rust
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
            age: 255,
        }),
    ];
    let elems = &array[..];

    // OK
    debug_each::<HKT!(<'lt> => u8), _>(
        elems,
        |person: &'_ Person| -> u8 {
            person.age
        },
    );

    // OK
    debug_each::<HKT!(String), _>( /* using the lifetime-elision syntax */
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
```

</details>
