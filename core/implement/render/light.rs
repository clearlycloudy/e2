extern crate pretty_env_logger;
extern crate mazth;

use std::collections::HashMap;
use std::vec::Vec;
use std::any::Any;

use interface::i_ads;
use interface::i_renderobj;
use interface::i_ele;
use interface::i_component;

use self::mazth::mat::{ Mat4, Mat3x1 };

#[derive(Clone)]
pub struct LightAdsPoint {
    pub _id: u64,
    pub _pos: Mat3x1< f32 >,

    /// # ads values
    pub _ads_val_spec: Mat3x1< f32 >,
    pub _ads_val_diff: Mat3x1< f32 >,
    pub _ads_val_amb: Mat3x1< f32 >,
}

impl i_ele::IObjImpl for LightAdsPoint {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store uniform data
        {
            //todo: move material properties to its own component 
            let data_map : HashMap< String, Vec<f32> > =  [ ( String::from("Light.Position\0"), &self._pos._val[..] ),
                                                            ( String::from("Light.La\0"), &self._ads_val_amb._val[..] ),
                                                            ( String::from("Light.Ld\0"), &self._ads_val_diff._val[..] ),
                                                            ( String::from("Light.Ls\0"), &self._ads_val_spec._val[..] ),
                                                            ( String::from("Material.Ka\0"), &[ 0.2f32, 0.2f32, 0.2f32 ] ),
                                                            ( String::from("Material.Kd\0"), &[ 0.1f32, 0.1f32, 0.9f32 ] ),
                                                            ( String::from("Material.Ks\0"), &[ 0.9f32, 0.1f32, 0.1f32 ] ),
                                                            ( String::from("Material.Shininess\0"), &[ 3f32 ] ),
            ].into_iter().map(|&( ref k, ref v)| ( k.clone(), v[..].to_vec() ) ).collect();

            let uniform_group_id = 0;
            let data_group : HashMap< u64, Vec<String> > = [ ( uniform_group_id, [ String::from("Material.Ka\0"), String::from("Material.Kd\0"),
                                                                                   String::from("Material.Ks\0"), String::from("Material.Shininess\0"),
                                                                                   String::from("Light.Position\0"), String::from("Light.La\0"),
                                                                                   String::from("Light.Ld\0"), String::from("Light.Ls\0") ] ),
            ].into_iter().map(|&( ref k, ref v)| ( k.clone(), v[..].to_vec() ) ).collect();
            
            let c = i_component::ComponentRenderUniform {
                _data_dict_vf: data_map,
                _data_uniform_group: data_group,
                ..Default::default()
            };
            components.push( Box::new(c) );
            trace!( "load into render buffer: uniform: light" );
        }
        Ok( () )
    }
}

impl i_ads::AdsVal for LightAdsPoint {
    fn get_ads_val_spec( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_diff( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_amb( & self ) -> Mat3x1< f32 > {
        self._ads_val_amb
    }
}

impl i_renderobj::ObjPos for LightAdsPoint {
    fn get_pos( & self ) -> Mat3x1< f32 > {
        self._pos
    }
}

pub struct LightAdsSpot {
    pub _id: u64,
    pub _pos: Mat3x1< f32 >,

    /// # ads values
    pub _ads_val_spec: Mat3x1< f32 >,
    pub _ads_val_diff: Mat3x1< f32 >,
    pub _ads_val_amb: Mat3x1< f32 >,

    pub _fov: f32,
    pub _focus: Mat4< f32 >,

    /// # computed view from _pos, _focus
    pub _light_view_xform: Mat4< f32 >,
}

impl i_ads::AdsVal for LightAdsSpot {
    fn get_ads_val_spec( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_diff( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_amb( & self ) -> Mat3x1< f32 > {
        self._ads_val_amb
    }
}

impl i_renderobj::ObjPos for LightAdsSpot {
    fn get_pos( & self ) -> Mat3x1< f32 > {
        self._pos
    }
}

pub struct LightAdsDir {
    pub _id: u64,
    pub _pos: Mat3x1< f32 >,

    /// # ads values
    pub _ads_val_spec: Mat3x1< f32 >,
    pub _ads_val_diff: Mat3x1< f32 >,
    pub _ads_val_amb: Mat3x1< f32 >,

    pub _dir: Mat3x1< f32 >,
    pub _light_view_xform: Mat4< f32 >,
}

impl i_ads::AdsVal for LightAdsDir {
    fn get_ads_val_spec( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_diff( & self ) -> Mat3x1< f32 > {
        self._ads_val_spec
    }
    fn get_ads_val_amb( & self ) -> Mat3x1< f32 > {
        self._ads_val_amb
    }
}
