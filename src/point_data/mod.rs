pub mod float_point_data;
use std::any::Any;

use crate::easings::functions::Functions;

// Generic trait for point data
pub trait BasePointData<T> {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self) -> T;
    fn get_time(&self) -> f32;
    fn as_any(&self) -> &dyn Any;
}
