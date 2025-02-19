pub mod float_point_data;
pub mod quaternion_point_data;
pub mod vector3_point_data;
pub mod vector4_point_data;

use crate::easings::functions::Functions;
use crate::values::base_provider_context::BaseProviderContext;
use std::any::Any;

// Generic trait for point data
pub trait BasePointData<T> {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self, context: &BaseProviderContext) -> T;
    fn get_time(&self) -> f32;
}
