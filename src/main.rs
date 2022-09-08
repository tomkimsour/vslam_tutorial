pub mod ch3;
use crate::ch3::nalgebra_matrix::nalgebra_matrix;
use core::fmt::Error;

fn main() -> Result<(),Error> {
    nalgebra_matrix();
    Ok(())
}
