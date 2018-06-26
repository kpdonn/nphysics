use na::{DVectorSliceMut, Real};

use joint::Joint;

use math::{Isometry, JacobianSliceMut, Vector, Velocity, SPATIAL_DIM};
use solver::IntegrationParameters;

/// A joint that allows all the relative degrees of freedom between two multibody links.
///
/// This joint can only be added between a `Ground` body (as parent) and any other body.
#[derive(Copy, Clone, Debug)]
pub struct FreeJoint<N: Real> {
    position: Isometry<N>,
}

impl<N: Real> FreeJoint<N> {
    /// Creates a free joint with the given initial position of the descendent, relative to the ground.
    pub fn new(position: Isometry<N>) -> Self {
        FreeJoint { position }
    }

    fn apply_displacement(&mut self, disp: &Velocity<N>) {
        let disp = Isometry::new(disp.linear, disp.angular);
        self.position = Isometry::from_parts(
            disp.translation * self.position.translation,
            disp.rotation * self.position.rotation,
        )
    }
}

impl<N: Real> Joint<N> for FreeJoint<N> {
    fn ndofs(&self) -> usize {
        SPATIAL_DIM
    }

    fn body_to_parent(&self, _: &Vector<N>, _: &Vector<N>) -> Isometry<N> {
        self.position
    }

    fn update_jacobians(&mut self, _: &Vector<N>, _: &[N]) {}

    fn jacobian(&self, _: &Isometry<N>, out: &mut JacobianSliceMut<N>) {
        out.fill_diagonal(N::one());
    }

    fn jacobian_dot(&self, _: &Isometry<N>, _: &mut JacobianSliceMut<N>) {}

    fn jacobian_dot_veldiff_mul_coordinates(
        &self,
        _: &Isometry<N>,
        _: &[N],
        _: &mut JacobianSliceMut<N>,
    ) {
    }

    fn integrate(&mut self, params: &IntegrationParameters<N>, vels: &[N]) {
        #[cfg(target_arch = "wasm32")]
        use log;
        #[cfg(target_arch = "wasm32")]
        log("integrate calling from_slice");
        let disp = Velocity::from_slice(vels) * params.dt;
        self.apply_displacement(&disp);
    }

    fn apply_displacement(&mut self, disp: &[N]) {
        #[cfg(target_arch = "wasm32")]
        use log;
        #[cfg(target_arch = "wasm32")]
        log("apply_displacement calling from_slice");
        let disp = Velocity::from_slice(disp);
        self.apply_displacement(&disp);
    }

    fn jacobian_mul_coordinates(&self, vels: &[N]) -> Velocity<N> {
        #[cfg(target_arch = "wasm32")]
        use log;
        #[cfg(target_arch = "wasm32")]
        log("jacobian_mul_coordinates calling from_slice");
        Velocity::from_slice(vels)
    }

    fn jacobian_dot_mul_coordinates(&self, _: &[N]) -> Velocity<N> {
        // The jacobian derivative is zero.
        Velocity::zero()
    }

    fn default_damping(&self, _: &mut DVectorSliceMut<N>) {}
}
