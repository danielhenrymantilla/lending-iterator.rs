pub
trait FnOnce1<Arg>
where
    Self : FnOnce(Arg) -> Self::Ret,
{
    type Ret;
}

impl<F : ?Sized, Arg, Ret> FnOnce1<Arg> for F
where
    Self : FnOnce(Arg) -> Ret,
{
    type Ret = Ret;
}

pub
trait FnMut1<Arg>
where
    Self : FnOnce1<Arg>,
    Self : FnMut(Arg) -> Self::Ret,
{}

impl<F : ?Sized, Arg, Ret> FnMut1<Arg> for F
where
    Self : FnMut(Arg) -> Ret,
{}

pub
trait FnMutOption<Arg>
where
    Self : FnMut(Arg) -> Option<Self::Ret>,
{
    type Ret;
}

impl<F : ?Sized, Arg, Ret> FnMutOption<Arg> for F
where
    Self : FnMut(Arg) -> Option<Ret>,
{
    type Ret = Ret;
}
