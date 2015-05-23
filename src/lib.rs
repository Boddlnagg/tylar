//! Type-Level Arithmetic in Rust (tylar).

use std::marker::PhantomData;

/// Basic trait implemented by all number types.
pub trait NumType<N>: Into<i64> + Into<i32> + Into<i16> + Into<i8> {
    /// Creates a new instance of this number type, which is actually a no-op, since
    /// number types are zero-sized. Instances are useful, however, to be converted
    /// into actual integer values, using implementations of the `Into` trait.
    #[inline(always)] fn new() -> Self;
}

/// Marker trait for positive numbers (including zero).
pub trait PosType<N:NumType<N>>: Into<u64> + Into<u32> + Into<u16> + Into<u8> {}

/// Marker trait for negative numbers (including zero).
pub trait NegType<N:NumType<N>> {}

/// The number type for zero (0).
#[allow(dead_code)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Zero;

/// The successor of `N`, i.e. a positive number.
#[allow(dead_code)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Succ<N> {
    phantom: PhantomData<N>
}

/// The predecessor of `N`, i.e. a negative number.
#[allow(dead_code)]
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Pred<N> {
    phantom: PhantomData<N>
}

impl NumType<Zero> for Zero {
    #[inline(always)] fn new() -> Self { Zero }
}

impl PosType<Zero> for Zero {}
impl NegType<Zero> for Zero {}

impl<N:NumType<N>> NumType<Succ<N>> for Succ<N> {
    #[inline(always)] fn new() -> Self { Succ { phantom: PhantomData } }
}

impl<N:NumType<N>> NumType<Pred<N>> for Pred<N> {
    #[inline(always)] fn new() -> Self { Pred { phantom: PhantomData } }
}

impl<N:PosType<N> + NumType<N>> PosType<Succ<N>> for Succ<N> {}
impl<N:NegType<N> + NumType<N>> NegType<Pred<N>> for Pred<N> {}

macro_rules! impl_into_signed {
    ($($ity:ty)+) => ($(
        impl<N:NumType<N>> Into<$ity> for Succ<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) + 1 }
        }

        impl<N:NumType<N>> Into<$ity> for Pred<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) - 1 }
        }

        impl Into<$ity> for Zero {
            #[inline(always)] fn into(self) -> $ity { 0 }
        }
    )+)
}

macro_rules! impl_into_unsigned {
    ($($ity:ty)+) => ($(
        impl<N:PosType<N> + NumType<N>> Into<$ity> for Succ<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) + 1 }
        }

        impl Into<$ity> for Zero {
            #[inline(always)] fn into(self) -> $ity { 0 }
        }
    )+)
}

impl_into_signed!(i64 i32 i16 i8);
impl_into_unsigned!(u64 u32 u16 u8);

/// Negation of number types.
pub trait Neg<A> {
    /// Result of the operation, i.e. `Out` = –`A`.
    type Out;
}
impl Neg<Zero> for Zero { type Out = Zero; }
impl<A:PosType<A>,B:NegType<B>> Neg<Succ<A>> for Succ<A> where A: Neg<A,Out=B> { type Out = Pred<B>; }
impl<A:NegType<A>,B:PosType<B>> Neg<Pred<A>> for Pred<A> where A: Neg<A,Out=B> { type Out = Succ<B>; }

/// Incrementation of number types.
pub trait Incr<A> {
    /// Result of the operation, i.e. `Out` = `A` + 1.
    type Out;
}
impl Incr<Zero> for Zero { type Out = Succ<Zero>; }
impl<A:PosType<A>> Incr<Succ<A>> for Succ<A> { type Out = Succ<Succ<A>>; }
impl<A:NegType<A>> Incr<Pred<A>> for Pred<A> { type Out = A; }

/// Decrementation of number types.
pub trait Decr<A> {
    /// Result of the operation, i.e. `Out` = `A` – 1.
    type Out;
}
impl Decr<Zero> for Zero { type Out = Pred<Zero>; }
impl<A:PosType<A>> Decr<Succ<A>> for Succ<A> { type Out = A; }
impl<A:NegType<A>> Decr<Pred<A>> for Pred<A> { type Out = Pred<Pred<A>>; }

/// Addition of number types.
pub trait Add<A,B> {
    /// Result of the operation, i.e. `Out` = `A` + `B`.
    type Out;
}
impl<A> Add<Zero,A> for Zero { type Out = A; }
impl<A:PosType<A>,B,C> Add<Succ<A>,B> for Succ<A> where B:Incr<B,Out=C>, A:Add<A,C>  { type Out = A::Out; }
impl<A:NegType<A>,B,C> Add<Pred<A>,B> for Pred<A> where B:Decr<B,Out=C>, A:Add<A,C>  { type Out = A::Out; }

/// Subtraction of number types.
pub trait Sub<A,B> {
    /// Result of the operation, i.e. `Out` = `A` – `B`.
    type Out;
}
impl<A:NumType<A>,B:NumType<B>,C,T:NumType<T>> Sub<A,B> for T where B:Neg<B,Out=C>, A:Add<A,C> { type Out = A::Out; }

/// Halving of number types.
/// `Div<_,P2>` could be used instead of this, but `Div` stresses the typechecker more
/// than `Halve`, so that `Halve` can be used with larger numbers without running into
/// the recursion limit.
pub trait Halve<A> {
    /// Result of the operation, i.e. `Out` = `A` / 2.
    type Out;
}
impl Halve<Zero> for Zero { type Out = Zero; }
impl<A:PosType<A>,B> Halve<Succ<Succ<A>>> for Succ<Succ<A>> where A:Halve<A,Out=B>  { type Out = Succ<B>; }
impl<A:NegType<A>,B> Halve<Pred<Pred<A>>> for Pred<Pred<A>> where A:Halve<A,Out=B>  { type Out = Pred<B>; }

/// Subtraction of number types.
pub trait Mul<A,B> {
    /// Result of the operation, i.e. `Out` = `A` * `B`.
    type Out;
}
impl<N:NumType<N>> Mul<Zero,N> for Zero { type Out = Zero; }
impl<A:PosType<A>,B,C> Mul<Succ<A>,B> for Succ<A> where A:Mul<A,B,Out=C>, B:Add<B,C> { type Out = B::Out; }
impl<A:NegType<A>,B,NB,C> Mul<Pred<A>,B> for Pred<A> where A:Mul<A,B,Out=C>, B:Neg<B,Out=NB>, NB:Add<NB,C> { type Out = NB::Out; }

/// Division of number types.
pub trait Div<A,B> {
    /// Result of the operation, i.e. `Out` = `A` / `B`.
    type Out;
}
impl<A:PosType<A>> Div<Zero,Succ<A>> for Zero { type Out = Zero; }
impl<A:NegType<A>> Div<Zero,Pred<A>> for Zero { type Out = Zero; }
impl<A:NumType<A>,B:NumType<B>,C:NumType<C>> Div<Succ<A>,Succ<B>> for Succ<A> where A:Sub<A,B,Out=C>, C:Div<C,Succ<B>> { type Out = Succ<C::Out>; }
impl<N:NegType<N>,NN:NegType<NN>,P:PosType<P>,PP:PosType<PP>> Div<Pred<N>,Pred<NN>> for Pred<N>
    where N:Neg<N,Out=P>, NN:Neg<NN,Out=PP>, Succ<P>:Div<Succ<P>,Succ<PP>> { type Out = <Succ<P> as Div<Succ<P>,Succ<PP>>>::Out; }
impl<P:NumType<P>, N:NegType<N>,PP:NumType<PP>,PPP:NumType<PPP>> Div<Succ<P>,Pred<N>> for Succ<P>
    where N:Neg<N,Out=PP>, Succ<P>:Div<Succ<P>,Succ<PP>,Out=Succ<PPP>>, Succ<PPP>:Neg<Succ<PPP>> { type Out = <Succ<PPP> as Neg<Succ<PPP>>>::Out; }
impl<P:NumType<P>, N:NegType<N>,PP:NumType<PP>,PPP:NumType<PPP>> Div<Pred<N>,Succ<P>> for Pred<N>
    where N:Neg<N,Out=PP>, Succ<PP>:Div<Succ<PP>,Succ<P>,Out=Succ<PPP>>, Succ<PPP>:Neg<Succ<PPP>> { type Out = <Succ<PPP> as Neg<Succ<PPP>>>::Out; }

/// Shorthand for the number 1 (the first successor of zero).
pub type P1 = Succ<Zero>;
/// Shorthand for the number 2 (the second successor of zero).
pub type P2 = Succ<P1>;
/// Shorthand for the number 3 (the third successor of zero).
pub type P3 = Succ<P2>;
/// Shorthand for the number 4 (the fourth successor of zero).
pub type P4 = Succ<P3>;
/// Shorthand for the number 5 (the fifth successor of zero).
pub type P5 = Succ<P4>;
/// Shorthand for the number 6 (the sixth successor of zero).
pub type P6 = Succ<P5>;
/// Shorthand for the number 7 (the seventh successor of zero).
pub type P7 = Succ<P6>;
/// Shorthand for the number 8 (the eighth successor of zero).
pub type P8 = Succ<P7>;
/// Shorthand for the number 9 (the nineth successor of zero).
pub type P9 = Succ<P8>;

/// Shorthand for the number –1 (the first predecessor of zero).
pub type N1 = Pred<Zero>;    
/// Shorthand for the number –2 (the second predecessor of zero).
pub type N2 = Pred<N1>;      
/// Shorthand for the number –3 (the third predecessor of zero).
pub type N3 = Pred<N2>;      
/// Shorthand for the number –4 (the fourth predecessor of zero).
pub type N4 = Pred<N3>;      
/// Shorthand for the number –5 (the fifth predecessor of zero).
pub type N5 = Pred<N4>;      
/// Shorthand for the number –6 (the sixth predecessor of zero).
pub type N6 = Pred<N5>;      
/// Shorthand for the number –7 (the seventh predecessor of zero).
pub type N7 = Pred<N6>;      
/// Shorthand for the number –8 (the eight predecessor of zero).
pub type N8 = Pred<N7>;     
/// Shorthand for the number –9 (the ninth predecessor of zero).
pub type N9 = Pred<N8>;

type Plus5<N> = Succ<Succ<Succ<Succ<Succ<N>>>>>;
type Plus10<N> = Plus5<Plus5<N>>;
type Plus50<N> = Plus10<Plus10<Plus10<Plus10<Plus10<N>>>>>;

#[test]
fn zero_sized() {
    use std::mem::size_of;
    assert_eq!(size_of::<Zero>(), 0);
    assert_eq!(size_of::<P1>(), 0);
    assert_eq!(size_of::<N1>(), 0);
    assert_eq!(size_of::<Plus50<Zero>>(), 0);
}

#[test]
fn into_number() {
    assert_eq!(0, Zero::new().into());
    assert_eq!(-3, N3::new().into());
    assert_eq!(2, P2::new().into());
    assert_eq!(2i8, P2::new().into());
    assert_eq!(2u64, P2::new().into());
    assert_eq!(2u8, P2::new().into());
    
    // 63 seems to be the maximal nesting depth acceptable to the compiler
    type P63 = Plus10<Plus50<P3>>;
    assert_eq!(63, P63::new().into());
}

#[test]
fn operations() {
    fn neg<A:NumType<A>,Out:NumType<Out>>() -> i32 where A:Neg<A,Out=Out> {
        Out::new().into()
    }
    
    fn add<A:NumType<A>,B:NumType<B>,Out:NumType<Out>>() -> i32 where A:Add<A,B,Out=Out> {
        Out::new().into()
    }
    
    fn sub<A:NumType<A>,B:NumType<B>,Out:NumType<Out>>() -> i32 where A:Sub<A,B,Out=Out> {
        Out::new().into()
    }
    
    fn halve<A:NumType<A>,Out:NumType<Out>>() -> i32 where A:Halve<A,Out=Out> {
        Out::new().into()
    }
    
    assert_eq!(-5, neg::<P5,_>());
    assert_eq!( 5, neg::<N5,_>());
    assert_eq!( 0, neg::<Zero,_>()); 
    
    assert_eq!( 5, add::<P2,P3,_>());
    assert_eq!(-1, sub::<P2,P3,_>());
    assert_eq!( 2, halve::<P4,_>());
    
    assert_eq!(-25, neg::<Plus5<Plus10<Plus10<Zero>>>,_>());
    assert_eq!( 45, sub::<Plus50<Zero>, P5,_>());
    assert_eq!( 50, halve::<Plus50<Plus50<Zero>>,_>());
}

#[test]
fn division() {
    fn div<A:NumType<A>,B:NumType<B>,Out:NumType<Out>>() -> i32 where A:Div<A,B,Out=Out> {
        Out::new().into()
    }
    
    assert_eq!(0, div::<Zero,P1,_>());
    
    assert_eq!(1, div::<P4,P4,_>());
    assert_eq!(2, div::<P4,P2,_>());
    assert_eq!(4, div::<P4,P1,_>());
    
    assert_eq!(1, div::<N4,N4,_>());
    assert_eq!(2, div::<N4,N2,_>());
    assert_eq!(4, div::<N4,N1,_>());
    
    assert_eq!(-1, div::<N4,P4,_>());
    assert_eq!(-2, div::<N4,P2,_>());
    assert_eq!(-4, div::<N4,P1,_>());
    
    assert_eq!(-1, div::<P4,N4,_>());
    assert_eq!(-2, div::<P4,N2,_>());
    assert_eq!(-4, div::<P4,N1,_>());
    
    assert_eq!( 2, div::<Plus10<Plus10<Zero>>,Plus10<Zero>,_>());
    assert_eq!(10, div::<Plus10<Plus10<Zero>>,P2,_>());
    assert_eq!( 4, div::<Plus10<Plus10<Zero>>,P5,_>());
}

#[test]
fn multiplication() {
    
    fn mul<A:NumType<A>,B:NumType<B>,Out:NumType<Out>>() -> i32 where A:Mul<A,B,Out=Out> {
        Out::new().into()
    }
    
    assert_eq!(0, mul::<Zero,Zero,_>());
    
    assert_eq!(0, mul::<P1,Zero,_>());
    assert_eq!(0, mul::<Zero,P1,_>());
    
    assert_eq!(1, mul::<P1,P1,_>());
    assert_eq!(2, mul::<P2,P1,_>());
    assert_eq!(2, mul::<P1,P2,_>());
    assert_eq!(4, mul::<P2,P2,_>());
    
    assert_eq!(-1, mul::<P1,N1,_>());
    assert_eq!(-2, mul::<P2,N1,_>());
    assert_eq!(-2, mul::<P1,N2,_>());
    assert_eq!(-4, mul::<P2,N2,_>());
    
    assert_eq!(-1, mul::<N1,P1,_>());
    assert_eq!(-2, mul::<N2,P1,_>());
    assert_eq!(-2, mul::<N1,P2,_>());
    assert_eq!(-4, mul::<N2,P2,_>());
    
    assert_eq!(1, mul::<N1,N1,_>());
    assert_eq!(2, mul::<N2,N1,_>());
    assert_eq!(2, mul::<N1,N2,_>());
    assert_eq!(4, mul::<N2,N2,_>());
    
    assert_eq!(25, mul::<P5,P5,_>());
    assert_eq!(25, mul::<N5,N5,_>());
}