#[macro_use]
mod macros;

/// Helper trait to allow expressing `F : FnMut(Arg) -> _` bounds (existential
/// return type).
pub
trait FnMut<Arg>
where
    Self : ::core::ops::FnMut(Arg) -> Self::Ret,
{
    type Ret;
}

impl<F : ?Sized, Arg, Ret>
    FnMut<Arg>
for
    F
where
    Self : ::core::ops::FnMut(Arg) -> Ret,
{
    type Ret = Ret;
}
