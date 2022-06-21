
# The what, the why and the how of Higher Kinded Types (HKTs)

  - Feel free to also check up <https://docs.rs/polonius-the-crab>'s extensive
    documentation, since it also has to deal with Higher-Kinded Types (even
    though, in practice, it hides them under the ~~rug~~ macros 🙃)

### What are Higher-Kinded Types?

<details>

A higher-kinded type is an actual / **full / standalone** type which is, itself,
"generic", or rather, to which we can further feed generic parameters (such as
lifetime parameters or type parameters) to obtain further types.

  - [ ] ❓ "is generic" / can be fed generic parameters to construct a type.
  - [ ] ❓ is a type in and of itself.
      - For instance, `type Standalone = YourHktType;` has to compile.

One way to illustrate this difference, for instance, would be to consider:

```rust
use ::lending_iterator::higher_kinded_types::HKT;

type StringRefNaïve<'lt> = &'lt str;
// and
type StringRef = HKT!(<'lt> &'lt str);
```

Both `StringRefNaïve` and `StringRef` can be fed a generic parameter (in this
instance, a lifetime parameter) so as to get or construct a type:

```rust
use ::lending_iterator::higher_kinded_types::{Feed, HKT};

# type StringRefNaïve<'lt> = &'lt str;
# type StringRef = HKT!(<'lt> &'lt str);
#
const _: StringRefNaïve<'static> = "This is a `&'static str`";
const _: Feed<'static, StringRef> = "This is a `&'static str`";
```

  - [x] "is generic" / can be fed generic parameters to construct a type.

But what of:

  - [ ] ❓ is a type in and of itself.

Well, while `StringRef` is indeed a standalone type:

```rust
use ::lending_iterator::higher_kinded_types::HKT;

type StringRef = HKT!(<'lt> &'lt str);

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
also _not be fed any parameters_ and still be a type_. That is,

> a HKT is actually an _actual_ **type** which is generic / can be fed
> parameters.

Another definition, which will make more sense in the following section, is that
HKTs come into play the moment we need "generic generics".


</details>

### Why? What are Higher-Kinded Types _for_?

  - #### Type-HKTs

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

    - <code>type StructOfPairs = StructOfArrays\< [HKT!]\(\<T\> \[T; 2\]\) \></code>;

        ```rust
        struct StructOfPairs {
            array_of_i32s: [i32; 2],
            array_of_strings: [String; 2],
        }
        ```

    In some cases, that level of genericity could lead to very s(l)ick designs.
    The example above was kind of simple, but a more classic need would be to
    try to be generic over the thread-safety of a reference-counted pointer:

    ```rust ,ignore
    type Arc = HKT!(<T> ::std::sync::Arc<T>);
    type Rc = HKT!(<T> ::std::rc::Rc<T>);

    type MyHandle<RefCountedPtr : <T>>(
        RefCounterPtr<Inner>,
    );

    type MyHandleSync = MyHandle<Arc>;
    type MyHandleFast = MyHandle<Rc>;
    ```

  - #### Lifetime HKTs

    Another use case can be around lifetimes, when dealing with higher-order
    lifetimes (_e.g._, when exposing borrows of callee local variables to a
    caller-chosen generic parameter).

## Back to real Rust

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
            trait HKT { type Assoc<T>; }
            ```

         1. And then querying the type would be done with:
            ```rust ,ignore
            <ArrayKind as HKT>::Assoc<T>
            ```

      - Without `generic_associated_types`:

        There is still an option in the case of a generic lifetime parameter
        (`<'lt>` instead of `<T>`):

         1. We define:
            ```rust ,ignore
            trait WithLifetime<'lt> { type Assoc; }
            ```

         1. And then we alias `trait HKT = for<'any> WithLifetime<'any>;`

         1. Querying the type is then done with:
            ```rust ,ignore
            <Type as WithLifetime<'lt>>::Assoc
            ```

  - Providing implementors or implementations of that trait:

      - neither `Vec` nor `VecDeque` are,
        _alone_, types. They're "syntactical type paths" which can be fed a
        type parameter to then refer to one of the `Vec{,Deque}<T>`
        types.

      - So we'll need to define _ad-hoc_ implementors of this genericity, as
        hinted by the `HKT!(<T> [T; 2])` example syntax.

    So, while it would be possible to manually implement:

      - the `generic_associated_type`-based trait:

        `impl HKT for … { type Assoc<'lt> = …; }`,

      - or even the without-`generic_…_types` polyfill:

        `impl<'lt> WithLifetime<'lt> for … { type Assoc = …; }`

    , the truth is that we don't necessarily need to write all these
    implementations if we are able to somehow magically produce appropriate
    type implementors "on demand" (in an _ad-hoc_ fashion).

    And it turns out there is! `dyn` to the rescue! Indeed, `dyn Trait<…>`
    is a standalone / "on demand"-queryable type, which does implement
    `Trait<…>`.

    This yields `dyn for<T> HKT<Assoc<T> = …>` in the general case, and
    `dyn for<'lt> WithLifetime<'lt, Assoc = …>` in the polyfill case.

    #### A convenience macro shorthand

    Since remembering it all is hard, writing it in full, cumbersome, and
    reading it, noisy, this crate offers a convenience macro shorthand:

    ```rust
    use ::lending_iterator::higher_kinded_types::HKT;

    type StringRef = HKT!(<'lt> &'lt str);
    ```
