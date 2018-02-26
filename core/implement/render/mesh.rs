extern crate pretty_env_logger;
extern crate mazth;

use self::mazth::mat::{ Mat3x1, Mat2x1 };
use std::collections::HashMap;
use std::vec::Vec;
use std::any::Any;

use interface::i_ele;
use interface::i_renderobj;
use interface::i_component;

#[derive(Clone)]
pub struct Mesh {
    pub _id: u64,
    pub _pos: Vec< Mat3x1< f32 > >,
    pub _normal: Vec< Mat3x1< f32 > >,
    pub _tc: Vec< Mat2x1< f32 > >,
    // _xform: Mat4< f32 >,
}

impl Mesh {
    pub fn init( id: u64 ) -> Mesh {
        Mesh {
            _id: id,
            _pos: vec![],
            _normal: vec![],
            _tc: vec![],
        }
    }
}

impl i_ele::IObjImpl for Mesh {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store vertex data
        {
            if self._pos.len() != self._normal.len() ||
                self._pos.len() != self._tc.len()
            {
                return Err( &"unequal length for position, normal, tc data" )
            }
            let ele_len = self._pos.len();
            let mut pos = vec![];
            let mut normal = vec![];
            let mut tc = vec![];
            for i in 0..self._pos.len() {
                pos.extend_from_slice( &self._pos[i]._val[..] );
            }
            for i in 0..self._normal.len() {
                let n = &self._normal[i].normalize().unwrap();
                // normal.extend_from_slice( &self._normal[i]._val[..] );
                normal.extend_from_slice( &n._val[..] );
            }
            for i in 0..self._tc.len() {
                tc.extend_from_slice( &self._tc[i]._val[..] );
            }
            let data_map : HashMap< i_renderobj::BuffDataType, Vec<f32> > =  [ ( i_renderobj::BuffDataType::POS, pos ),
                                                                                 ( i_renderobj::BuffDataType::NORMAL, normal ),
                                                                                 ( i_renderobj::BuffDataType::TC, tc ) ].iter().cloned().collect();

            let c = i_component::ComponentRenderBuffer {
                _data_dict: data_map,
            };
            components.push( Box::new(c) );
            trace!( "load into render buffer: mesh: vertex count:{}", ele_len / 3 );
        }
        //store uniform data
        {

            
        }
        Ok( () )
    }
}
