///module for camera motion by wrapping a render camera
///and additional property interpolators to be used for motion stepping

extern crate mazth;

use std::collections::HashMap;
use interface::i_interpolate::*;
use interface::i_camera::ICamera;

use self::mazth::mat::*;
// use implement::math::frustum::*;
use implement::render::camera;

pub struct CamGuide {
    ///interpolators of various properties of camera. todo: extract them out to a separate module
    pub _prop_interp: HashMap< camera::PropKey, Box< IInterpolate< Mat4x1< f64 >, Item = Mat4x1< f64 > > > >,
    ///values to be applied to interpolators
    pub _prop_next_update: HashMap< camera::PropKey, camera::PropVal >,
    ///camera for rendering
    pub _cam: camera::Cam,
}

impl Default for CamGuide {
    fn default() -> CamGuide {
        let fov = 120f32;
        let aspect = 1f32;
        let near = 0.001f32;
        let far = 1000f32;
        let cam_foc_pos = Mat3x1 { _val: [0f32, 0f32, 10f32] };
        let cam_up = Mat3x1 { _val: [0f32, 1f32, 0f32] };
        let cam_pos = Mat3x1 { _val: [0f32, 0f32, 0f32] };
        let cam_id = 0; //dummy for now
        CamGuide {
            _prop_interp: HashMap::new(),
            _prop_next_update: HashMap::new(),
            _cam: camera::Cam::init( cam_id, fov, aspect, near, far, cam_pos, cam_foc_pos, cam_up ),
        }
    }
}

impl CamGuide {
    pub fn init( c: camera::Cam ) -> CamGuide {
        CamGuide {
            _prop_interp: HashMap::new(),
            _prop_next_update: HashMap::new(),
            _cam: c,
        }        
    }
}

impl ICamera for CamGuide {
    fn clear_property_next( & mut self ) {
        self._prop_next_update.clear();
    }
    ///buffer values to be set for interpolators
    fn set_property_next( & mut self, prop: ( camera::PropKey, camera::PropVal ) ) -> Result< (), & 'static str > {
        self._prop_next_update.insert( prop.0, prop.1 );
        Ok( () )
    }
    ///apply values into the interpolator in order for the interpolater to advance closer to the input value in upcoming interpolating cycles
    fn flush_property_next( & mut self ) -> Result< (), & 'static str > {
        unimplemented!();
    }
    ///run property interpolators for some steps
    fn run_interpolators( & mut self, _steps: i64 ) -> Result< (), & 'static str > {
        unimplemented!();
    }
    ///apply current interpolator values to camera properties
    fn apply_interp_to_cam( & mut self ) -> Result< ( & camera::Cam ), & 'static str > {
        unimplemented!();
    }
    fn get_cam( & mut self ) -> & camera::Cam {
        unimplemented!();
    }
}
