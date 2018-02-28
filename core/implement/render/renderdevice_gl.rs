extern crate pretty_env_logger;
extern crate gl;

use std::collections::HashMap;
use std::vec::Vec;

use interface::i_renderobj;
use implement::render::util_gl;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct BufferFormat {
    pub _buff_data_type: i_renderobj::BuffDataType,
    pub _index: u64,
    pub _num_data: u64,
    pub _stride_bytes: u64,
    pub _offset_bytes: u64,
}

/// # the following is analogous to vertex array object in opengl
#[derive(Debug)]
pub struct RenderDrawGroup {
    pub _group_handle: u64,// <=> vao handle
    pub _buffer_handle: u64, // <-> vbo handle
    pub _buffer_draw: Vec< f32 >,
    pub _format: Vec< BufferFormat >,
    pub _stride: u64,
    pub _primitive_type: i_renderobj::RenderObjType,
}

impl RenderDrawGroup {
    pub fn init( internal_group_handle: u64, buffer_handle: u64, format: Vec< BufferFormat >, stride: u64, primitive_type: i_renderobj::RenderObjType ) -> RenderDrawGroup {
        RenderDrawGroup {
            _group_handle: internal_group_handle,
            _buffer_handle: buffer_handle,
            _buffer_draw: vec![],
            _format: format,
            _stride: stride,
            _primitive_type: primitive_type,
        }
    }
    pub fn init_with_default_format_triangle( internal_group_handle: u64, buffer_handle: u64 ) -> RenderDrawGroup {
        let stride = 8 * ::std::mem::size_of::<f32>();
        let format = vec![ BufferFormat { _buff_data_type: i_renderobj::BuffDataType::POS, _index: 0, _num_data: 3, _stride_bytes: stride as _, _offset_bytes: 0 },
                           BufferFormat { _buff_data_type: i_renderobj::BuffDataType::NORMAL, _index: 1, _num_data: 3, _stride_bytes: stride as _, _offset_bytes: (3 * ::std::mem::size_of::<f32>()) as _ },
                           BufferFormat { _buff_data_type: i_renderobj::BuffDataType::TC, _index: 2, _num_data: 2, _stride_bytes: stride as _, _offset_bytes: (6 * ::std::mem::size_of::<f32>()) as _ },
        ];
        RenderDrawGroup {
            _group_handle: internal_group_handle,
            _buffer_handle: buffer_handle,
            _buffer_draw: vec![],
            _format: format,
            _stride: stride as _,
            _primitive_type: i_renderobj::RenderObjType::TRI,
            // _primitive_type: i_renderobj::RenderObjType::POINT,
        }
    }
    pub fn init_with_default_format_point( internal_group_handle: u64, buffer_handle: u64 ) -> RenderDrawGroup {
        //todo: remove normal and tc
        let stride = 8 * ::std::mem::size_of::<f32>();
        let format = vec![ BufferFormat { _buff_data_type: i_renderobj::BuffDataType::POS, _index: 0, _num_data: 3, _stride_bytes: stride as _, _offset_bytes: 0 },
                           BufferFormat { _buff_data_type: i_renderobj::BuffDataType::NORMAL, _index: 1, _num_data: 3, _stride_bytes: stride as _, _offset_bytes: (3 * ::std::mem::size_of::<f32>()) as _ },
                           BufferFormat { _buff_data_type: i_renderobj::BuffDataType::TC, _index: 2, _num_data: 2, _stride_bytes: stride as _, _offset_bytes: (6 * ::std::mem::size_of::<f32>()) as _ },
        ];
        RenderDrawGroup {
            _group_handle: internal_group_handle,
            _buffer_handle: buffer_handle,
            _buffer_draw: vec![],
            _format: format,
            _stride: stride as _,
            _primitive_type: i_renderobj::RenderObjType::POINT,
        }
    }
}

impl i_renderobj::RenderDevice for RenderDrawGroup{
    fn bind_buffer( & mut self ) -> Result< (), & 'static str > {
        let data_len = self._buffer_draw.len() * ::std::mem::size_of::<f32>();
        unsafe {
            gl::BindVertexArray( self._group_handle as _ );
            gl::BindBuffer( gl::ARRAY_BUFFER, self._buffer_handle as _ );
            gl::BufferData( gl::ARRAY_BUFFER, data_len as isize, self._buffer_draw.as_ptr() as _, gl::STATIC_DRAW );
            util_gl::check_last_op();

            for &i in self._format.iter() {
                gl::VertexAttribPointer( i._index as _, i._num_data as _, gl::FLOAT, gl::FALSE, i._stride_bytes as i32, i._offset_bytes as _ );
                gl::EnableVertexAttribArray( i._index as _ );
                util_gl::check_last_op();
            }
            gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
            gl::BindVertexArray( 0 );
        }
        Ok( () )
    }
    fn draw_buffer_all( & mut self) -> Result< (), & 'static str > {
        unsafe {
            gl::BindVertexArray( self._group_handle as _ );
            let num_elements = self._buffer_draw.len() / (self._stride as usize / ::std::mem::size_of::<f32>());
            match self._primitive_type {
                i_renderobj::RenderObjType::TRI => {
                    trace!("draw buffer all: num verts: {}", num_elements );
                    gl::DrawArrays(gl::TRIANGLES, 0, num_elements as _ );
                },
                i_renderobj::RenderObjType::POINT => {
                    trace!("draw buffer all: num points: {}", num_elements );
                    //todo: add configurable point size
                    gl::PointSize(3f32);
                    gl::DrawArrays(gl::POINTS, 0, num_elements as _ );
                },
                _=> return Err( "unsupported primite type for drawing detected" )
            }
            gl::BindVertexArray( 0 );
        }
        Ok( () )
    }
    fn draw_buffer_range( & mut self) -> Result< (), & 'static str > {
        unimplemented!();
    }
    fn clear_buff_data( & mut self ){
        self._buffer_draw.clear();
    }
    fn store_buff_data( & mut self, data: & HashMap< i_renderobj::BuffDataType, Vec< f32 > > ) -> Result< (), & 'static str > {
        let mut data_pos : Vec< f32 > = vec![];
        let mut data_normal : Vec< f32 > = vec![];
        let mut data_tc : Vec< f32 > = vec![];
        for i in self._format.iter() {
            match i._buff_data_type {
                i_renderobj::BuffDataType::POS => {
                    match data.get( &i_renderobj::BuffDataType::POS ) {
                        None => return Err( "render buffer data expected data not found: pos" ),
                        Some( ref pos ) => {
                            data_pos.extend_from_slice( &pos[..] );
                        }
                    }
                },
                i_renderobj::BuffDataType::NORMAL => {
                    match data.get( &i_renderobj::BuffDataType::NORMAL ) {
                        None => return Err( "render buffer data expected data not found: normal" ),
                        Some( ref normal ) => {
                            data_normal.extend_from_slice( &normal[..] );
                        }
                    }
                },
                i_renderobj::BuffDataType::TC => {
                    match data.get( &i_renderobj::BuffDataType::TC ) {
                        None => return Err( "render buffer data expected data not found: tc" ),
                        Some( ref tc ) => {
                            data_tc.extend_from_slice( &tc[..] );
                        }
                    }
                },
            }
        }
        match self._primitive_type {
            i_renderobj::RenderObjType::TRI => {
                if data_pos.len() != data_normal.len() {
                    return Err( "render buffer data length not equal" )
                }
                if data_pos.len() % 3 != 0 {
                    return Err( "render buffer data length not divisible by 3" )
                }
                if data_tc.len() % 2 != 0 {
                    return Err( "render buffer data length not divisible by 2" )
                }
                let count_data = data_pos.len() / 3;
                if count_data != data_tc.len() / 2 {
                    return Err( "render buffer data length not equal" )
                }
                for i in 0..count_data {
                    self._buffer_draw.push( data_pos[i*3] );
                    self._buffer_draw.push( data_pos[i*3+1] );
                    self._buffer_draw.push( data_pos[i*3+2] );
                    
                    self._buffer_draw.push( data_normal[i*3] );
                    self._buffer_draw.push( data_normal[i*3+1] );
                    self._buffer_draw.push( data_normal[i*3+2] );

                    self._buffer_draw.push( data_tc[i*2] );
                    self._buffer_draw.push( data_tc[i*2+1] );
                }
            },
            i_renderobj::RenderObjType::POINT => {
                //todo: remove normal and texture coordinate
                if data_pos.len() != data_normal.len() {
                    return Err( "render buffer data length not equal" )
                }
                if data_pos.len() % 3 != 0 {
                    return Err( "render buffer data length not divisible by 3" )
                }
                if data_tc.len() % 2 != 0 {
                    return Err( "render buffer data length not divisible by 2" )
                }
                let count_data = data_pos.len() / 3;
                if count_data != data_tc.len() / 2 {
                    return Err( "render buffer data length not equal" )
                }
                for i in 0..count_data {
                    self._buffer_draw.push( data_pos[i*3] );
                    self._buffer_draw.push( data_pos[i*3+1] );
                    self._buffer_draw.push( data_pos[i*3+2] );
                    
                    self._buffer_draw.push( data_normal[i*3] );
                    self._buffer_draw.push( data_normal[i*3+1] );
                    self._buffer_draw.push( data_normal[i*3+2] );

                    self._buffer_draw.push( data_tc[i*2] );
                    self._buffer_draw.push( data_tc[i*2+1] );
                }
            },
            i_renderobj::RenderObjType::LINE => {
                //todo
            },
            _ => { unimplemented!(); }
        }
        Ok( () )
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum UniformType {
    VEC,
    MAT4,
    MAT4X1,
    MAT4X2,
    MAT4X3,
    MAT3,
    MAT3X1,
    MAT3X2,
    MAT2,
    MAT2X1,
}

//todo: move uniform related functionalities into render device
// #[derive(Debug)]
pub struct RenderUniformCollection {
    /// # key: (shader_program_handle, uniform_name), val: uniform_vals
    pub _uniforms_f: HashMap< (u64, String), ( UniformType, Vec<f32> ) >,
    /// # key: (group_id), val: (shader_program_handle, Vec<uniform_names>)
    pub _uniforms_groups: HashMap< u64, (u64, Vec<String>) >,
}

impl Default for RenderUniformCollection {
    fn default() -> RenderUniformCollection {
        RenderUniformCollection {
            _uniforms_f: HashMap::new(),
            _uniforms_groups: HashMap::new(),
        }
    }
}

impl RenderUniformCollection {
    /// # set uniform values to be sent to shader later
    pub fn set_uniform_f( & mut self, shader_program: u64, name: &str, val_type: UniformType, vals: &[f32] ) {
        self._uniforms_f.insert( (shader_program, String::from( name ) ), ( val_type, vals.to_vec() ) );
    }
    /// # group uniforms together
    pub fn set_group( & mut self, shader_program: u64, group_id: u64, names: Vec< String > ) -> Result< (), & 'static str > {
        for i in names.iter() {
            match self._uniforms_f.get( &( shader_program, i.clone() ) ) {
                None => return Err( &"unfound uniform name" ),
                _ => (),
            }
        }
        self._uniforms_groups.insert( group_id, ( shader_program, names ) );
        Ok( () )
    }
    pub fn get_group( & self, group_id: u64 ) -> Result< Vec< (String, UniformType, Vec<f32> )>, & 'static str > {
        let mut ret = vec![];
        match self._uniforms_groups.get( &group_id ) {
            None => return Err( &"unfound uniform group id" ),
            Some( &( ref program, ref v ) ) => {
                for name in v.iter() {
                    match self._uniforms_f.get( &( *program, name.clone()) ) {
                        None => return Err( &"unfound uniform name" ),
                        Some( &( ref uniform_type , ref vals) ) => {
                            ret.push( ( name.clone(), uniform_type.clone(), vals.clone() ) );
                        }
                    }
                }
            }
        }
        Ok( ret )
    }
    /// # send the uniform values belonging to the uniform group to the shader
    pub fn send_uniform_group( & self, group_id: u64 ) -> Result< (), & 'static str > {
        match self._uniforms_groups.get( &group_id ) {
            None => return Err( &"unfound uniform group id" ),
            Some( &( ref program, ref v ) ) => {
                for name in v.iter() {
                    match self._uniforms_f.get( &( *program, name.clone()) ) {
                        None => return Err( &"unfound uniform name" ),
                        Some( &( ref uniform_type , ref vals) ) => {
                            match *uniform_type {
                                UniformType::VEC => {
                                    unsafe {
                                        match vals.len() {
                                            1 => gl::Uniform1f( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), vals[0] ),
                                            2 => gl::Uniform2fv( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), 1, vals.as_ptr() as _ ),
                                            3 => gl::Uniform3fv( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), 1, vals.as_ptr() as _ ),
                                            4 => gl::Uniform4fv( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), 1, vals.as_ptr() as _ ),
                                            _ => return Err( &"unsupported uniform data length" ),
                                        }
                                    }
                                },
                                UniformType::MAT4 => {
                                    if vals.len() != 16 {
                                        return Err( &"unmatched uniform length for MAT4" )
                                    }
                                    unsafe {
                                        //assumes matrix is row major
                                        gl::UniformMatrix4fv( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), 1, gl::TRUE, vals.as_ptr() as _ );
                                    }
                                },
                                UniformType::MAT3 => {
                                    if vals.len() != 9 {
                                        return Err( &"unmatched uniform length for MAT3" )
                                    }
                                    unsafe {
                                        //assumes matrix is row major
                                        gl::UniformMatrix3fv( gl::GetUniformLocation( *program as _, name.as_ptr() as * const i8 ), 1, gl::TRUE, vals.as_ptr() as _ );
                                    }
                                },
                                _ => { unimplemented!(); },
                            }
                        }
                    }
                }
            }
        }
        Ok( () )
    }
}
