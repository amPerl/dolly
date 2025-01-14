use std::marker::PhantomData;

use glam::Vec3;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    ///
    pub offset: Vec3,
}

impl Arm {
    ///
    pub fn new(offset: Vec3) -> Self {
        Self { offset }
    }
}

impl<H: Handedness> RigDriver<H> for Arm {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            rotation: params.parent.rotation,
            position: params.parent.position + params.parent.rotation * self.offset,
            phantom: PhantomData,
        }
    }
}
