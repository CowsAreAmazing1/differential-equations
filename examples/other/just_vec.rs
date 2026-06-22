use differential_equations::prelude::*;

struct ExponentialGrowth {
    k: f64,
}

impl ODE<f64, Vec<f64>> for ExponentialGrowth {
    fn diff(&self, _t: f64, y: &Vec<f64>, dydt: &mut Vec<f64>) {
        for i in 0..y.len() {
            dydt[i] = self.k * y[i];
        }
    }
}

fn main() {
    let y0 = vec![0.0, 1.0, 2.0, 3.0, 4.0];
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
