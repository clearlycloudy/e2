extern crate pretty_env_logger;
extern crate mazth;

use std::collections::HashMap;
use std::vec::Vec;
use std::any::Any;

use interface::i_renderobj;
use interface::i_ele;
use interface::i_component;

use self::mazth::mat::*;
use implement::math;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PropKey {
    Fov,
    Aspect,
    Near,
    Far,
    Pos,
    Up,
    Focus,
}

#[derive(Clone)]
pub enum PropVal {
    Fov(f32),
    Aspect(f32),
    Near(f32),
    Far(f32),
    Pos(Mat3x1< f32 >),
    Up(Mat3x1< f32 >),
    Focus(Mat3x1< f32 >),
}

#[derive(Clone)]
pub struct Cam {
    /// # helper id for the camera
    pub _id: u64,

    pub _proj_xform: Mat4< f32 >,
    /// # The following generates the projection matrix
    pub _fov: f32,
    pub _aspect: f32,
    pub _near: f32,
    pub _far: f32,

    pub _view_xform: Mat4< f32 >,
    /// # The following generates the view matrix
    pub _pos: Mat3x1< f32 >,
    pub _pos_orig: Mat3x1< f32 >,
    pub _up: Mat3x1< f32 >,
    pub _focus: Mat3x1< f32 >,
}

impl Cam {
    pub fn init( id: u64, fov: f32, aspect: f32, near: f32, far: f32, pos: Mat3x1< f32 >, focus: Mat3x1< f32 >, up: Mat3x1< f32 > ) -> Cam {
        Cam {
            _id: id,
            _fov: fov,
            _aspect: aspect,
            _near: near,
            _far: far,
            _pos: pos,
            _pos_orig: pos,
            _up: up,
            _focus: focus,
            _proj_xform: math::util::perspective( fov, aspect, near, far ),
            _view_xform: math::util::look_at( pos, focus, up ),
        }
    }
    pub fn update_pos( & mut self, pos: Mat3x1< f32 > ) {
        self._pos = pos;
        self._view_xform = math::util::look_at( pos, self._focus, self._up );
    }
}

impl i_ele::IObjImpl for Cam {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store uniform data
        {
            let model_transform = Mat4::<f32> { _val: [ 1f32, 0f32, 0f32, 0f32,
                                                                   0f32, 1f32, 0f32, 0f32,
                                                                   0f32, 0f32, 1f32, 0f32,
                                                                   0f32, 0f32, 0f32, 1f32 ],
                                                            _is_row_major: true };

            let mvp_transform = self._proj_xform.mul( &self._view_xform ).unwrap().mul( &model_transform ).unwrap();
            let model_view_transform = self._view_xform.mul( &model_transform ).unwrap();
            let normal_inv_transpose = model_view_transform.submat_mat3().inverse().unwrap().transpose();

            trace!( "mv: {:?}", model_view_transform );
            trace!( "mv submat3: {:?}", model_view_transform.submat_mat3() );
            trace!( "mvp: {:?}", mvp_transform );
            trace!( "normal matrix: {:?}", normal_inv_transpose );

            let data_map_mat4f : HashMap< String, Vec<f32> > =  [ ( String::from("ModelViewMatrix\0"), &model_view_transform._val[..] ),
                                                                  ( String::from("ProjectionMatrix\0"), &self._proj_xform._val[..] ),
                                                                  ( String::from("MVP\0"), &mvp_transform._val[..] ),
            ].into_iter().map(|&( ref k, ref v)| ( k.clone(), v[..].to_vec() ) ).collect();
            
            let data_map_mat3f : HashMap< String, Vec<f32> > =  [ ( String::from("NormalMatrix\0"), &normal_inv_transpose._val[..] ),
            ].into_iter().map(|&( ref k, ref v)| ( k.clone(), v[..].to_vec() ) ).collect();                                                              

            let uniform_group_id = 1;
            let data_group : HashMap< u64, Vec<String> > = [ ( uniform_group_id, [ String::from("ModelViewMatrix\0"), String::from("NormalMatrix\0"), //todo: add warning message on unmatched uniform name in uniform manager
                                                                                   String::from("ProjectionMatrix\0"), String::from("MVP\0") ] ),
            ].into_iter().map(|&( ref k, ref v)| ( k.clone(), v[..].to_vec() ) ).collect();
            
            let c = i_component::ComponentRenderUniform {
                _data_dict_mat4f: data_map_mat4f,
                _data_dict_mat3f: data_map_mat3f,
                _data_uniform_group: data_group,
                ..Default::default()
            };
            components.push( Box::new(c) );
            trace!( "load into render buffer: uniform: camera" );
        }
        Ok( () )
    }
}

impl i_renderobj::ObjPos for Cam {
    fn get_pos( & self ) -> Mat3x1< f32 > {
        self._pos
    }
}
