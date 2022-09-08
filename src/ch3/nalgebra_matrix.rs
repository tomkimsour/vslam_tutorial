extern crate nalgebra as na;
use na::{Matrix2x3,Vector3, DMatrix, Matrix3};

pub fn nalgebra_matrix(){
    // First declare a static matrix and initialize it with zeros
    let mut matrix = Matrix2x3::<f32>::zeros();
    print!("{:}",matrix);

    // Then initialize a vector 
    let vector = Vector3::<f32>::zeros();
    print!("{:}",vector);

    // // it is also possible to use dynamic matrix if you are not sure about the size.
    let dmatrix = DMatrix::from_vec(2,2,vec![2.2,3.2,1.5,6.5]);
    print!("{:}",dmatrix);

    matrix = Matrix2x3::from_row_slice(&[
        1.2, 1.1, 1.2,
        3.1, 2.5, 1.5
    ]);
    
    print!("{:}",matrix);

    let dmatrix = DMatrix::from_vec(3,2,vec![5.5,6.3,2.2,3.2,1.5,6.5]);
    print!("{:}",matrix*dmatrix);

    // Let s give a look at all the operations possible
    let matrix_33 = Matrix3::<f32>::new_random();
    print!("{:}",matrix_33);
    print!("{:}",matrix_33.transpose());

}