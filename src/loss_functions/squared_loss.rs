use crate::maths::{
    types::MatrixD
};
/// Squared loss
pub fn squared_loss(output: &MatrixD<f32>, target: &MatrixD<f32>) -> MatrixD<f32> {
    //0.5 * ((output - target).exp2());
    let mut out = output.clone();
    out.zip_apply(&target, |o,t| 0.5 * ((o - t).exp2()));

    out
}

/// Squared loss gradient
pub fn squared_loss_gradient(errors: MatrixD<f32>) -> MatrixD<f32> {
    // f(x) = 0.5 * ((output - target).exp2()) = 0.5 * x.exp2(); 
    // dt/dx = x
    errors
}