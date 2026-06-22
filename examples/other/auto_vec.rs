use differential_equations::{prelude::*, traits::State};

#[derive(State)]
struct TestState<T, const N: usize> {
    a: Vec<T>,
}

// Differential equations are defined using structs that implement the ODE trait
struct ExponentialGrowth {
    k: f64,
}

impl ODE<f64, TestState<f64, 5>> for ExponentialGrowth {
    // Define the differential equation dy/dt = k*y
    fn diff(&self, _t: f64, y: &TestState<f64, 5>, dydt: &mut TestState<f64, 5>) {
        // println!("in ODE, state len: {}", y.len());

        for i in 0..y.len() {
            dydt.set_component(i, self.k * y.get_component(i));
        }
    }
}

fn main() {
    let y0 = TestState { a: vec![1.0] };
    let t0 = 0.0;
    let tf = 10.0;
    let ode = ExponentialGrowth { k: 1.0 };

    let solution = match IVP::ode(&ode, t0, tf, y0)
        .method(ExplicitRungeKutta::dop853().rtol(1e-12).atol(1e-12))
        .solve()
    {
        Ok(solution) => solution,
        Err(e) => panic!("Error: {:?}", e),
    };

    // Print the solution using the fields of the Solution struct
    println!(
        "Solution: ({:?}, {:?})",
        solution.t.last().unwrap(),
        solution.y.last().unwrap()
    );
    println!("Function evaluations: {}", solution.evals.function);
    println!("Steps: {}", solution.steps.total());
    println!("Rejected Steps: {}", solution.steps.rejected);
    println!("Accepted Steps: {}", solution.steps.accepted);
    println!("Status: {:?}", solution.status);
}
