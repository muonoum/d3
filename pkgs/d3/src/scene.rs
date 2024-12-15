use crate::light::Light;
use crate::object::Object;

pub struct Scene {
	objects: Vec<Object>,
	lights: Vec<Light>,
}
