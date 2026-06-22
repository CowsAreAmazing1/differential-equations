/// Aim is to allow the ability to easily change the number of objects in a system, say by
/// changing the number of elements in the state struct's arrays
///
/// Since Vecs can't impl `State`,
///
/// struct State<T> {
///     position: [T; N],
///     velocity: [T; N],
/// }
///
/// Currently, the automatic `State` derive macro only allows arrays of a compile time literal size
use differential_equations::derive::*;

#[derive(State)]
struct GoodState<T> {
    x: T,
    y: [T; 5], // Compile time "constant" or really a compile time literal, as was working before
}

// const NUM: usize = 10;
// #[derive(State)]
// struct BadState<T> {
//     x: T,
//     y: T,
//     z: [T; NUM], // `NUM` is a compile time constant, but not a literal, so this is not accepted
// }

// Inputs to macros like this are seen as Expr::Group by syn, see /derive/src/field_analysis.rs::analyze_field_type
macro_rules! make_state {
    ($n:expr) => {
        #[derive(State)]
        struct MacroState<T> {
            x: T,
            y: [T; $n],
        }
    };
}
// A literal works
make_state!(6);

// const NUM: usize = 10;
// make_state!(NUM); // Again a compile time constant does not.

fn assert_state<State>() {
    println!("`{}` impls `State`", std::any::type_name::<State>())
}

fn main() {
    assert_state::<GoodState<f64>>();
    assert_state::<MacroState<f64>>();
}
