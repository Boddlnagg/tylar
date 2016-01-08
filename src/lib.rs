//! Type-Level Arithmetic in Rust (tylar).

use std::marker::PhantomData;

/// Basic trait implemented by all number types.
pub trait NumType: Into<i64> + Into<i32> + Into<i16> + Into<i8> + Into<isize> {
    /// Creates a new instance of this number type, which is actually a no-op, since
    /// number types are zero-sized. Instances are useful, however, to be converted
    /// into actual integer values, using implementations of the `Into` trait.
    #[inline(always)] fn new() -> Self;
}

/// Marker trait for positive numbers (including zero).
pub trait PosType: NumType + Into<u64> + Into<u32> + Into<u16> + Into<u8> + Into<usize> {}

/// Marker trait for negative numbers (including zero).
pub trait NegType: NumType {}

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

impl NumType for Zero {
    #[inline(always)] fn new() -> Self { Zero }
}

impl PosType for Zero {}
impl NegType for Zero {}

impl<N: NumType> NumType for Succ<N> {
    #[inline(always)] fn new() -> Self { Succ { phantom: PhantomData } }
}

impl<N: NumType> NumType for Pred<N> {
    #[inline(always)] fn new() -> Self { Pred { phantom: PhantomData } }
}

impl<N: PosType> PosType for Succ<N> {}
impl<N: NegType> NegType for Pred<N> {}

macro_rules! impl_into_signed {
    ($($ity:ty)+) => ($(
        impl<N: NumType> Into<$ity> for Succ<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) + 1 }
        }

        impl<N: NumType> Into<$ity> for Pred<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) - 1 }
        }

        impl Into<$ity> for Zero {
            #[inline(always)] fn into(self) -> $ity { 0 }
        }
    )+)
}

macro_rules! impl_into_unsigned {
    ($($ity:ty)+) => ($(
        impl<N: PosType> Into<$ity> for Succ<N> {
            #[inline(always)] fn into(self) -> $ity { Into::<$ity>::into(N::new()) + 1 }
        }

        impl Into<$ity> for Zero {
            #[inline(always)] fn into(self) -> $ity { 0 }
        }
    )+)
}

impl_into_signed!(i64 i32 i16 i8 isize);
impl_into_unsigned!(u64 u32 u16 u8 usize);

/// Negation of number types.
pub trait Neg: NumType {
    /// Result of the operation, i.e. `Out` = –`Self`.
    type Out: NumType;
}
impl Neg for Zero { type Out = Zero; }
impl<A: PosType, B: NegType> Neg for Succ<A> where A: Neg<Out=B> { type Out = Pred<B>; }
impl<A: NegType, B: PosType> Neg for Pred<A> where A: Neg<Out=B> { type Out = Succ<B>; }

/// Incrementation of number types.
pub trait Incr: NumType {
    /// Result of the operation, i.e. `Out` = `Self` + 1.
    type Out: NumType;
}
impl Incr for Zero { type Out = Succ<Zero>; }
impl<A: PosType> Incr for Succ<A> { type Out = Succ<Succ<A>>; }
impl<A: NegType> Incr for Pred<A> { type Out = A; }

/// Decrementation of number types.
pub trait Decr: NumType {
    /// Result of the operation, i.e. `Out` = `Self` – 1.
    type Out: NumType;
}
impl Decr for Zero { type Out = Pred<Zero>; }
impl<A: PosType> Decr for Succ<A> { type Out = A; }
impl<A: NegType> Decr for Pred<A> { type Out = Pred<Pred<A>>; }

/// Addition of number types.
pub trait Add<RHS>: NumType {
    /// Result of the operation, i.e. `Out` = `Self` + `RHS`.
    type Out: NumType;
}
impl<RHS: NumType> Add<RHS> for Zero { type Out = RHS; }
impl<A: PosType, RHS, B: NumType> Add<RHS> for Succ<A> where RHS: Incr<Out=B>, A: Add<B>  { type Out = A::Out; }
impl<A: NegType, RHS, B: NumType> Add<RHS> for Pred<A> where RHS: Decr<Out=B>, A: Add<B>  { type Out = A::Out; }

/// Subtraction of number types.
pub trait Sub<RHS>: NumType {
    /// Result of the operation, i.e. `Out` = `Self` – `RHS`.
    type Out: NumType;
}
impl<A, RHS, B: NumType> Sub<RHS> for A where RHS: Neg<Out=B>, A: Add<B> { type Out = A::Out; }

/// Halving of number types.
/// `Div<_,P2>` could be used instead of this, but `Div` stresses the typechecker more
/// than `Halve`, so that `Halve` can be used with larger numbers without running into
/// the recursion limit.
pub trait Halve: NumType {
    /// Result of the operation, i.e. `Out` = `Self` / 2.
    type Out: NumType;
}
impl Halve for Zero { type Out = Zero; }
impl<A: PosType, B: NumType> Halve for Succ<Succ<A>> where A: Halve<Out=B>  { type Out = Succ<B>; }
impl<A: NegType, B: NumType> Halve for Pred<Pred<A>> where A: Halve<Out=B>  { type Out = Pred<B>; }

/// Subtraction of number types.
pub trait Mul<RHS>: NumType {
    /// Result of the operation, i.e. `Out` = `Self` * `RHS`.
    type Out: NumType;
}
impl<N: NumType> Mul<N> for Zero { type Out = Zero; }
impl<A: PosType, RHS, B: NumType> Mul<RHS> for Succ<A> where A: Mul<RHS, Out=B>, RHS: Add<B> { type Out = RHS::Out; }
impl<A: NegType, RHS, B, C: NumType> Mul<RHS> for Pred<A> where A: Mul<RHS, Out=C>, RHS: Neg<Out=B>, B: Add<C> { type Out = B::Out; }

/// Division of number types.
pub trait Div<RHS>: NumType {
    /// Result of the operation, i.e. `Out` = `Self` / `RHS`.
    type Out: NumType;
}
impl<A: PosType> Div<Succ<A>> for Zero { type Out = Zero; }
impl<A: NegType> Div<Pred<A>> for Zero { type Out = Zero; }
impl<A: NumType, B: NumType, C: NumType> Div<Succ<B>> for Succ<A> where A: Sub<B, Out=C>, C: Div<Succ<B>> { type Out = Succ<C::Out>; }
impl<N: NegType, NN: NegType, P: PosType, PP: PosType> Div<Pred<NN>> for Pred<N>
    where N: Neg<Out=P>, NN: Neg<Out=PP>, Succ<P>: Div<Succ<PP>> { type Out = <Succ<P> as Div<Succ<PP>>>::Out; }
impl<P: NumType, N: NegType, PP: NumType, PPP: NumType> Div<Pred<N>> for Succ<P>
    where N: Neg<Out=PP>, Succ<P>: Div<Succ<PP>, Out=Succ<PPP>>, Succ<PPP>: Neg { type Out = <Succ<PPP> as Neg>::Out; }
impl<P: NumType, N: NegType, PP: NumType, PPP: NumType> Div<Succ<P>> for Pred<N>
    where N: Neg<Out=PP>, Succ<PP>: Div<Succ<P>, Out=Succ<PPP>>, Succ<PPP>: Neg { type Out = <Succ<PPP> as Neg>::Out; }

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
    fn neg<A: NumType, Out: NumType>() -> i32 where A: Neg<Out=Out> {
        Out::new().into()
    }
    
    fn add<A: NumType, B: NumType, Out: NumType>() -> i32 where A: Add<B, Out=Out> {
        Out::new().into()
    }
    
    fn sub<A: NumType, B: NumType, Out: NumType>() -> i32 where A: Sub<B, Out=Out> {
        Out::new().into()
    }
    
    fn halve<A: NumType, Out: NumType>() -> i32 where A: Halve<Out=Out> {
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
    fn div<A: NumType, B: NumType, Out: NumType>() -> i32 where A: Div<B, Out=Out> {
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
    
    fn mul<A: NumType, B: NumType, Out: NumType>() -> i32 where A: Mul<B, Out=Out> {
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