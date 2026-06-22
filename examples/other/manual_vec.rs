use differential_equations::{
    prelude::*,
    traits::{Real, State},
};

#[derive(Clone, Debug)]
struct TestState<T, const N: usize> {
    a: [T; N],
    b: Vec<T>,
}

impl<T, const N: usize> State<T> for TestState<T, N>
where
    T: Real,
{
    fn len(&self) -> usize {
        N + self.b.len()
    }

    fn get_component(&self, index: usize) -> T {
        if index < N {
            self.a[index]
        } else {
            self.b[index - N]
        }
    }

    fn set_component(&mut self, index: usize, value: T) {
        if index < N {
            self.a[index] = value;
        } else {
            self.b[index - N] = value;
        }
    }

    fn map_components_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, &mut T),
    {
        for (i, val) in self.a.iter_mut().chain(self.b.iter_mut()).enumerate() {
            f(i, val);
        }
    }

    fn zeros_like(&self) -> Self {
        Self {
            a: [T::zero(); N],
            b: vec![T::zero(); self.b.len()],
        }
    }

    fn zeros() -> Self {
        Self {
            a: [T::zero(); N],
            b: Vec::new(),
        }
    }

    fn mul_add_assign(&mut self, alpha: T, other: &Self) {
        assert_eq!(self.len(), other.len(), "State length mismatch");
        // for (s, o) in self.a.iter_mut().zip(other.a.iter()) {
        //     *s += alpha * *o;
        // }
        // for (s, o) in self.b.iter_mut().zip(other.b.iter()) {
        //     *s += alpha * *o;
        // }
        for (s, o) in self
            .a
            .iter_mut()
            .zip(other.a.iter())
            .chain(self.b.iter_mut().zip(other.b.iter()))
        {
            *s += alpha * *o;
        }
    }

    fn scale_mut(&mut self, alpha: T) {
        for s in self.a.iter_mut().chain(self.b.iter_mut()) {
            *s *= alpha;
        }
    }
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
    let y0 = TestState {
        a: [1.0; 5],
        b: vec![1.0],
    };
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
