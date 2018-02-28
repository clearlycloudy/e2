extern crate pretty_env_logger;
extern crate gl;
extern crate chrono;

use self::chrono::prelude::*;

use ::std::str;
use ::std::ops::DerefMut;
use std::collections::HashMap;
use std::cell::RefCell;   

use interface::i_ele;
use interface::i_renderobj::RenderDevice;
use interface::i_renderobj;
use interface::i_renderpass;
use interface::i_component;
use interface::i_renderer::IRenderer;

use implement::capability::capability_gl;
use implement::render::util_gl;
use implement::render::shader_collection;
use implement::render::texture_collection;
use implement::render::router;
use implement::render::renderdevice_gl;
use implement::render::render_commands;

pub struct Renderer {
    _rp: Vec< Box< i_renderpass::IRenderPass > >,
    _map_string_to_rp: HashMap< String, usize >,
    pub _shader_collection: RefCell< shader_collection::ShaderCollection >,
    _texture_collection: texture_collection::TextureCollection,
    _shader_programs: Vec< u64 >,
    _draw_groups: RefCell< Vec< renderdevice_gl::RenderDrawGroup > >,
    _vaos: Vec< gl::types::GLuint >,
    _vbos: Vec< gl::types::GLuint >,
    _objs: Vec< i_ele::Ele >,
    _uniforms: RefCell< renderdevice_gl::RenderUniformCollection >,
    _draw_group_uniforms: RefCell< Vec< Vec< u64 > > >,
    _shaders_compiled: Vec< gl::types::GLuint >,
    //todo: to be removed
    _current_shader_program: u64,
    _is_init: bool,
    _draw_group: usize,
}

impl Drop for Renderer {
    fn drop( & mut self ) {
        unsafe {
            gl::DeleteBuffers( self._vaos.len() as _, self._vaos.as_ptr(), );

            gl::DeleteBuffers( self._vbos.len() as _, self._vbos.as_ptr(), );
        }
        self._shader_collection.borrow_mut().clear().is_ok(); //does DeleteProgram
        unsafe {
            for i in self._shaders_compiled.iter() {
                gl::DeleteShader( *i );
            }
        }
    }
}

pub enum Event {
    AddObj( i_ele::Ele ),
    LoadShader( Vec< ( String, util_gl::ShaderType ) > ),
    LoadTexture( String, Vec< u8 >, usize, usize ),
    CreateDrawGroup( i_renderobj::RenderObjType ),
}

pub enum EventResult {
    IdShaderProgram( isize ),
    IdTexture( isize ),
    IdDrawGroup( isize ),
}

impl IRenderer for Renderer {

    type EventRender = Event;
    
    fn init() -> Result< Self, & 'static str > {
        Renderer::init()
    }
    fn process_render_events( & mut self, e: Vec< Self::EventRender > ) -> Result< (), & 'static str > {
        //first time initialization
        if !self._is_init {
            info!("renderer: first time initialization.");
            self._is_init = true;
            let ( _vao, _vbo, draw_group ) = self.create_draw_group( i_renderobj::RenderObjType::TRI ).unwrap();
            self._draw_group = draw_group;
        }
        
        //setup hardcoded draw group for now
        //todo: remove it and put in a configurable hook
        let dummy_str = "";
        let draw_group = self._draw_group;
        self.add_obj( dummy_str, i_ele::Ele::init( render_commands::CmdDrawGroupClear::init( draw_group ) ) ).is_ok();

        //handle events
        //todo: handle return values from calls
        for i in e {
            let t0 = Local::now();
            match i {
                Event::AddObj( x ) => {
                    self.add_obj( dummy_str, x )?;
                    let t1 = Local::now();
                    let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;
                    info!( "t_render_add_obj: {} ms", t_delta );
                },
                Event::LoadShader( x ) => {
                    self.load_shader( x.as_slice() )?;
                    let t1 = Local::now();
                    let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;
                    info!( "t_render_load_shader: {} ms", t_delta );
                },
                Event::LoadTexture( s, data, w, h ) => {
                    self.load_texture( s, &data, w, h )?;
                    let t1 = Local::now();
                    let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;
                    info!( "t_render_load_texture: {} ms", t_delta );
                },
                Event::CreateDrawGroup( x ) => {
                    self.create_draw_group( x )?;
                    let t1 = Local::now();
                    let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;
                    info!( "t_render_create_draw_group: {} ms", t_delta );
                },
            }
            util_gl::check_last_op();
        }

        //some hardcoded draw calls in preparation for rendering
        //todo: put these in a configurable hook

        let t0 = Local::now();

        self.add_obj( dummy_str, i_ele::Ele::init( render_commands::CmdDrawGroupBind::init( draw_group ) ) ).is_ok();

        let t1 = Local::now();
        
        self.add_obj( dummy_str, i_ele::Ele::init( render_commands::CmdDrawGroupDependentUniforms::init( draw_group, &[0u64,1u64] ) ) ).is_ok();

        let t2 = Local::now();

        self.add_obj( dummy_str, i_ele::Ele::init( render_commands::CmdDrawGroupDispatch::init( draw_group ) ) ).is_ok();

        let t3 = Local::now();

        {
            let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;
            info!( "t_draw_group_bind: {} ms", t_delta );
        }
        {
            let t_delta = t2.signed_duration_since(t1).num_milliseconds() as f64;
            info!( "t_draw_group_dependent_uniforms: {} ms", t_delta );
        }
        {
            let t_delta = t3.signed_duration_since(t2).num_milliseconds() as f64;
            info!( "t_draw_group_dispatch: {} ms", t_delta );
        }

        Ok( () )
    }
}

impl Renderer {
    pub fn init() -> Result< Renderer, & 'static str > {
        let rk = Renderer {
            _rp: vec![],
            _map_string_to_rp: HashMap::new(),
            _shader_collection: RefCell::new( Default::default() ),
            _texture_collection: Default::default(),
            _shader_programs: vec![],
            _draw_groups: RefCell::new( vec![] ),
            _vaos: vec![],
            _vbos: vec![],
            _objs: vec![],
            _uniforms: RefCell::new( Default::default() ),
            _draw_group_uniforms: RefCell::new( vec![] ),
            _shaders_compiled: vec![],
            _current_shader_program: 0,
            _is_init: false,
            _draw_group: 0usize,
        };
        let cap = capability_gl::query_gl();
        info!( "GL capability: {}", cap );
        Ok( rk )
    }
    pub fn load_shader( & mut self, sources: &[ ( String, util_gl::ShaderType ) ] ) -> Result< ( u64 ), & 'static str > {
        let mut compiled_shaders = vec![];
        for &(ref src, ref src_type ) in sources.into_iter() {
            let s = match util_gl::load_and_compile_shader( (*src).as_str(), *src_type ) {
                Ok( o ) => o,
                Err( o ) => {
                    error!( "{}", o );
                    return Err( "error loading shader" )
                }
            };
            util_gl::check_last_op();
            compiled_shaders.push( s );
        }
        {
            let i = self._shader_programs.len();
            {
                self._shader_collection.borrow_mut().put( i as u64, router::ShaderType::GLSL, util_gl::create_program_from_shaders( compiled_shaders.as_slice() ) as _, String::from("ads_program") ).is_ok();
            }
            let shader_program = self._shader_collection.borrow_mut().get( i as u64 ).unwrap();
            unsafe {
                gl::UseProgram( shader_program as _ );
                self._current_shader_program = i as u64;
                gl::BindFramebuffer( gl::FRAMEBUFFER, 0 );
                gl::Enable( gl::DEPTH_TEST );
            }
            self._shader_programs.push( shader_program as u64 );
            {
                self._shaders_compiled.append( & mut compiled_shaders );
            }
            Ok( i as u64 )
        }
    }
    pub fn load_texture( & mut self, description: String, image: &[u8], w: usize, h: usize ) -> Result< ( u64 ), & 'static str > {
        let shader_program_internal = self._shader_collection.borrow_mut().get( self._current_shader_program ).unwrap();
        let handle = match util_gl::load_texture( shader_program_internal as _, 0, image, w, h ) {
            Ok( h ) => h,
            _ => return Err( "loading texture failed" )
        };
        let h = match self._texture_collection.add( router::ShaderType::GLSL, handle as _, description ) {
            Ok( h ) => h,
            Err( e ) => return Err( e ),
        };
        Ok( h )
    }
    pub fn create_draw_group( & mut self, prim_type: i_renderobj::RenderObjType ) -> Result< ( gl::types::GLuint, gl::types::GLuint, usize ), & 'static str > {
        let mut obj_vao = 0;
        let mut obj_vbo = 0;
        unsafe {
            gl::GenVertexArrays( 1, & mut obj_vao );
            util_gl::check_last_op();
            gl::GenBuffers( 1, & mut obj_vbo );
            util_gl::check_last_op();
        }
        
        let draw_group = match prim_type {
            i_renderobj::RenderObjType::TRI => renderdevice_gl::RenderDrawGroup::init_with_default_format_triangle( obj_vao as _, obj_vbo as _ ),
            i_renderobj::RenderObjType::POINT => renderdevice_gl::RenderDrawGroup::init_with_default_format_point( obj_vao as _, obj_vbo as _ ),
            _ => return Err("unsupported primitive type for draw group detected")
        };
        // let mut draw_group = renderdevice_gl::RenderDrawGroup::init_with_default_format( obj_vao as _, obj_vbo as _ );
        self._draw_groups.borrow_mut().push( draw_group );
        Ok( ( obj_vao, obj_vbo, self._draw_groups.borrow_mut().len() - 1) )
    }
    #[allow(unused)]
    pub fn add_obj( & mut self, name: &str, e: i_ele::Ele ) -> Result< ( usize ), & 'static str > {
        
        let index = self._objs.len();
        self._objs.push( e );

        let t0 = Local::now();
        
        //load component data
        match self._objs[index].update_components_from_impl() {
            Err( e ) => { return Err( e ) },
            _ => (),
        }

        let t1 = Local::now();

        let t_delta = t1.signed_duration_since(t0).num_milliseconds() as f64;

        info!( "t_update_components_from_impl: {} ms", t_delta );

        //detect command to flush and process all data in buffer
        let mut trigger_to_process_objs = false;
        let mut group_id = 0;

        // println!( "component numbers: {}", self._objs.borrow_mut()[index]._components.len() );
        
        for j in self._objs[index]._components.iter() {
            match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDispatch >() {
                Some( o ) => {
                    trace!("detected trigger for draw group dispatch");
                    trigger_to_process_objs = true;
                    group_id = o._group_id;
                },
                None => {},
            }
        }
        
        if trigger_to_process_objs {

            let t2 = Local::now();
            
            Renderer::process_objs( self, group_id )?;

            let t3 = Local::now();

            let t_delta = t3.signed_duration_since(t2).num_milliseconds() as f64;

            info!( "t_process_objs flush: {} ms", t_delta );
        }

        Ok( self._objs.len() )
    }
    pub fn process_objs( renderer: & mut Renderer, group_index: usize ) -> Result< (), & 'static str > {        

        let drained = renderer._objs.drain(..).collect::<Vec<_>>();

        for i in drained {

            for j in i._components {

                match j.as_any().downcast_ref::< i_component::ComponentRenderBuffer >() {
                    Some( o ) => {
                        trace!("buffer flushed");
                        match o.flush_into_render_device( & mut renderer._draw_groups.get_mut()[ group_index ] ) {
                            Err( e ) => return Err( e ),
                            _ => { continue; },
                        }
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentRenderUniform >() {
                    Some( o ) => {
                        trace!("uniform flushed");
                        let shader_program = renderer._shader_collection.borrow_mut().get( renderer._current_shader_program ).unwrap();
                        match o.flush_into_uniform_collection( shader_program, & mut renderer._uniforms.borrow_mut() ) {
                            Err( e ) => return Err( e ),
                            _ => { continue; },
                        }
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupClear >() {
                    Some( o ) => {
                        trace!("draw group clear");
                        renderer.reset_draw_group_data( &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupBind >() {
                    Some( o ) => {
                        trace!("draw group bind");
                        renderer.bind_draw_group_data( &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDependentUniforms >() {
                    Some( o ) => {
                        trace!("draw group dependent uniforms");
                        renderer.set_draw_group_uniforms( o._group_id, &o._uniform_ids[..] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                match j.as_any().downcast_ref::< i_component::ComponentDrawGroupDispatch >() {
                    Some( o ) => {
                        trace!("draw group dispatch");
                        let renderer_immut : & Renderer = & * renderer;
                        Renderer::drawcall_draw_group( renderer_immut, &[ o._group_id ] ).is_ok();
                        continue;
                    },
                    None => (),
                }
                return Err( &"unmatching render command" );
            }
        }

        assert_eq!(renderer._objs.len(), 0 );

        Ok( () )
    }
    pub fn reset_draw_group_data( & self, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            if i >= self._draw_groups.borrow_mut().len() {
                return Err( "object index out of range" )
            }
            let mut dg = self._draw_groups.borrow_mut();
            dg[ i ].clear_buff_data();
        }
        Ok( () )
    }
    pub fn bind_draw_group_data( & self, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            if i >= self._draw_groups.borrow_mut().len() {
                return Err( "object index out of range" )
            }
            match self._draw_groups.borrow_mut()[ i ].bind_buffer() {
                Err( e ) => return Err( e ),
                _ => (),
            }
        }
        Ok( () )
    }
    pub fn drawcall_draw_group( renderer: & Renderer, group_indices: &[ usize ] ) -> Result< (), & 'static str > {
        for &i in group_indices {
            for uniform_group in renderer._draw_group_uniforms.borrow()[i].iter() {
                trace!("dispatching uniform group: {}", *uniform_group );
                if i >= renderer._draw_groups.borrow().len() {
                    return Err( "object index out of range" )
                }
                match renderer._uniforms.borrow_mut().send_uniform_group( *uniform_group ){
                    Err(e) => return Err(e),
                    _ => ()
                }
                match renderer._draw_groups.borrow_mut()[ i ].draw_buffer_all() {
                    Err( e ) => return Err( e ),
                    _ => (),
                }
            }
        }
        Ok( () )
    }
    pub fn add_renderpass< T > ( & mut self, name: String, rp: T ) -> usize
    where T: i_renderpass::IRenderPass + 'static {
        let i = self._rp.len();
        self._map_string_to_rp.insert( name, i );
        self._rp.push( Box::new( rp ) );
        i
    }
    pub fn get_renderpass( & mut self, name: String ) -> Option< & mut i_renderpass::IRenderPass > {
        match self._map_string_to_rp.get( &name ){
            None => None,
            Some( o ) => {
                Some( self._rp[*o].deref_mut() )
            }
        }
    }
    // pub fn uniforms_ref( & mut self ) -> & mut renderdevice_gl::RenderUniformCollection {
    //     & mut self._uniforms.get_mut()
    // }
    pub fn add_draw_group_uniforms( & self, draw_group: usize, uniform_group: &[u64] ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            self._draw_group_uniforms.borrow_mut().resize( draw_group + 1, vec![] );
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].extend_from_slice( uniform_group );
        Ok( () )
    }
    pub fn set_draw_group_uniforms( & self, draw_group: usize, uniform_group: &[u64] ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            self._draw_group_uniforms.borrow_mut().resize( draw_group + 1, vec![] );
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].clear();
        self._draw_group_uniforms.borrow_mut()[ draw_group ].extend_from_slice( uniform_group );
        Ok( () )
    }
    pub fn clear_draw_group_uniforms( & self, draw_group: usize ) -> Result< (), & 'static str > {
        if self._draw_group_uniforms.borrow_mut().len() <= draw_group {
            return Err( "draw group out of range" )
        }
        self._draw_group_uniforms.borrow_mut()[ draw_group ].clear();
        Ok( () )
    }
    pub fn get_shader_program( & mut self, id: u64 ) -> Option< i64 > {
        self._shader_collection.borrow_mut().get( id )
    }
}

