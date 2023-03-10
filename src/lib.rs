use pyo3::prelude::*;
mod astar;
mod astar_v2;
mod dstar;
mod astar_test;
mod astar_v2_test;
mod dstar_test;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn aruw_pathfinding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<astar::AStar>()?;
    m.add_class::<dstar::DStar>()?;
    Ok(())
}
