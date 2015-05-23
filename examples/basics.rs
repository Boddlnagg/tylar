extern crate tylar;
use tylar::{NumType,Add,Sub,Mul,Div,P1,P2,P3,P5,N1,N2,N3,N4};

// Type-level function, calculating (N * (-4) + 2) / 2
trait Calculation1<N> { type Out; }
impl<N:NumType<N>,Times4,Plus2> Calculation1<N> for N where N:Mul<N,N4,Out=Times4>, Times4:Add<Times4,P2,Out=Plus2>, Plus2:Div<Plus2,P2> { type Out = Plus2::Out; }

// Type-level function, calculating (1 + N) * (1 - N)
trait Calculation2<N> { type Out; }
impl<N:NumType<N>,OnePlusN,OneMinusN> Calculation2<N> for N where P1:Sub<P1,N,Out=OneMinusN>, P1:Add<P1,N,Out=OnePlusN>, OnePlusN:Mul<OnePlusN,OneMinusN> { type Out = OnePlusN::Out; }

fn main() {
    // Unfortunately these first 5 examples don't work in Rust 1.0
    let result1: i32 = <N2 as Add<_,P5>>::Out::new().into();
    println!("-2 + 5 = {}", result1);
    
    let result2: i32 = <P3 as Calculation1<_>>::Out::new().into();
    println!("(N * (-4) + 2) / 2 = {} (for N = 3)", result2);
    
    let result3: i32 = <N1 as Calculation1<_>>::Out::new().into();
    println!("(N * (-4) + 2) / 2 = {} (for N = -1)", result3);
    
    let result4: i32 = <P3 as Calculation2<_>>::Out::new().into();
    println!("(1 + N) * (1 - N) = {} (for N = 3)", result4);
    
    let result5: i32 = <N1 as Calculation2<_>>::Out::new().into();
    println!("(1 + N) * (1 - N) = {} (for N = -1)", result5);
    
    fn do_something<A:NumType<A>,B:NumType<B>>() where A:Calculation2<A,Out=B> {
        // This function only exists for combinations of A and B where B = (1 + A) * (1 - A)
        let a: i32 = A::new().into();
        let b: i32 = B::new().into();
        assert_eq!(b, (1 + a) * (1 - a));
        println!("{1} = (1 + {0}) * (1 - {0})", a, b);
    }
    
    do_something::<P2, N3>();
    do_something::<P3, _>(); // here B is inferred to be P8
    
    // The following lines are rejected by the typechecker:
    
    //do_something::<P2, N2>(); // type mismatch
    //do_something::<_, N3>(); // can't infer the first parameter
    
}