extern crate mazth;

use std::collections::HashMap;

use self::mazth::mat;
use implement::render::renderdevice_gl::RenderUniformCollection;

pub trait Xform {
    fn get_xform() -> mat::Mat4< f32 >;
}

pub trait ObjPos {
    fn get_pos( & self ) -> mat::Mat3x1< f32 >;
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum RenderObjType {
    TRI,
    //todo
    QUAD,
    POINT,
    LINE,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
pub enum BuffDataType {
    POS,
    NORMAL,
    TC,
}

pub trait RenderDevice {
    fn bind_buffer( & mut self ) -> Result< (), & 'static str >;
    fn draw_buffer_all( & mut self ) -> Result< (), & 'static str >;
    fn draw_buffer_range( & mut self ) -> Result< (), & 'static str >;
    fn store_buff_data( & mut self, data: & HashMap< BuffDataType, Vec< f32 > > ) -> Result< (), & 'static str >;
    fn clear_buff_data( & mut self );
}

pub trait IRenderBuffer {
    fn load_into_buffer( & mut self, rd: & mut RenderDevice ) -> Result< (), & 'static str >;
}

pub enum RenderMethod {
    ADS,
    PBR,
    NONE,
}
pub trait IRenderable : IRenderBuffer {
    fn get_render_method( & self ) -> RenderMethod;
}

pub trait IRenderUniform {
    fn load_into_uniform( & mut self, uniforms: & mut RenderUniformCollection ) -> Result< (), & 'static str >;
}
