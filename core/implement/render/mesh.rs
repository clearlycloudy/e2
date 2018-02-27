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

            // let pos = self._pos.iter()
            //     .flat_map(|x| x._val[..].to_vec() )
            //     .collect::<Vec<_>>();

            // let normal = self._normal.iter()
            //     .flat_map(|x| x.normalize().unwrap()._val[..].to_vec() )
            //     .collect::<Vec<_>>();

            // let tc = self._tc.iter()
            //     .flat_map(|x| x._val[..].to_vec() )
            //     .collect::<Vec<_>>();

            let mut pos = vec![];
            pos.reserve_exact( 3 * self._pos.len() );
            self._pos.iter_mut().for_each( |x| pos.append( & mut x._val.to_vec() ) );

            let mut normal = vec![];
            normal.reserve_exact( 3 * self._normal.len() );
            self._normal.iter_mut().for_each( |x| normal.append( & mut x.normalize().unwrap()._val.to_vec() ) );
            
            let mut tc = vec![];
            tc.reserve_exact( 2 * self._tc.len() );
            self._tc.iter_mut().for_each( |x| tc.append( & mut x._val.to_vec() ) );
            
            let data_map : HashMap< i_renderobj::BuffDataType, Vec<f32> > =  [ ( i_renderobj::BuffDataType::POS, pos ),
                                                                                 ( i_renderobj::BuffDataType::NORMAL, normal ),
                                                                                 ( i_renderobj::BuffDataType::TC, tc ) ].into_iter().cloned().collect();

            let c = i_component::ComponentRenderBuffer {
                _data_dict: data_map,
            };
            components.push( Box::new(c) );
            trace!( "load into render buffer: mesh: vertex count:{}", ele_len / 3 );
        }
        //todo: store uniform data
        {

            
        }
        Ok( () )
    }
}
