extern crate tylar;
use tylar::{NumType, Add, Sub, Mul, Div, P1, P2, P3, P5, N1, N2, N3, N4};

// Type-level function, calculating (N * (-4) + 2) / 2
trait Calculation1 { type Out; }
impl<N: NumType, Times4, Plus2> Calculation1 for N where N: Mul<N4, Out=Times4>, Times4: Add<P2, Out=Plus2>, Plus2: Div<P2> { type Out = Plus2::Out; }

// Type-level function, calculating (1 + N) * (1 - N)
trait Calculation2 { type Out; }
impl<N: NumType, OnePlusN, OneMinusN: NumType> Calculation2 for N where P1: Sub<N, Out=OneMinusN>, P1: Add<N, Out=OnePlusN>, OnePlusN: Mul<OneMinusN> { type Out = OnePlusN::Out; }

fn main() {
    // Unfortunately these first 5 examples don't work in Rust 1.0
    let result1: i32 = <N2 as Add<P5>>::Out::new().into();
    println!("-2 + 5 = {}", result1);
    
    let result2: i32 = <P3 as Calculation1>::Out::new().into();
    println!("(N * (-4) + 2) / 2 = {} (for N = 3)", result2);
    
    let result3: i32 = <N1 as Calculation1>::Out::new().into();
    println!("(N * (-4) + 2) / 2 = {} (for N = -1)", result3);
    
    let result4: i32 = <P3 as Calculation2>::Out::new().into();
    println!("(1 + N) * (1 - N) = {} (for N = 3)", result4);
    
    let result5: i32 = <N1 as Calculation2>::Out::new().into();
    println!("(1 + N) * (1 - N) = {} (for N = -1)", result5);
    
    fn do_something<A: NumType, B: NumType>() where A: Calculation2<Out=B> {
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