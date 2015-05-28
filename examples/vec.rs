#![cfg_attr(feature = "nightly", feature(collections))]

extern crate tylar;

use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Result};
use tylar::{NumType, PosType, Zero, Succ};

#[cfg(feature = "nightly")]
use tylar::Add;

struct TVec<T,N:PosType<N>> {
    vec: Vec<T>, // Here we could just store a pointer, because the length is statically determined by the type
    p: PhantomData<N>
}

impl<T,N:PosType<N>+NumType<N>> Debug for TVec<T,N> where T:Debug {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        try!(self.vec.fmt(formatter));
        let len: usize = N::new().into();
        try!(write!(formatter, " ({})", len));
        Ok(())
    }
}

impl<T> TVec<T,Zero> {
    pub fn new() -> Self {
        TVec {vec: vec![], p: PhantomData}
    }
}

impl<T,N:PosType<N>+NumType<N>> TVec<T,N> {
    pub fn push(self, v: T) ->  TVec<T,Succ<N>> {
        let mut vec = self.vec;
        vec.push(v);
        TVec {vec: vec, p: PhantomData}
    }
}

#[cfg(feature = "nightly")]
impl<T,N:PosType<N>+NumType<N>> TVec<T,N> {
    pub fn append<NR:PosType<NR>+NumType<NR>,NOut:PosType<NOut>>(self, other: TVec<T,NR>) ->  TVec<T,NOut> where N:Add<N,NR,Out=NOut>  {
        let mut vec = self.vec;
        let mut other = other;
        vec.append(&mut other.vec);
        TVec {vec: vec, p: PhantomData}
    }
}

fn main() {
    for i in 0..2 {
        let v = TVec::new();
        let r;
        if i % 2 == 0 {
            let v = v.push(1);
            r = v.push(2);
        } else {
            let v = v.push(3);
            r = v.push(4);
        }
        println!("{:?}", r);
    }
}