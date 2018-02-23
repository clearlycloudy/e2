///sample implementation of game logic

extern crate image;
extern crate rand;
extern crate mazth;
extern crate e2rcore;
extern crate pretty_env_logger;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use self::e2rcore::interface::i_ele;
use self::e2rcore::interface::i_game_logic::IGameLogic;
use self::e2rcore::interface::i_ui::{ InputFiltered, KeyCode, State, Coord };
// use interface::i_camera::ICamera;
use self::e2rcore::interface::i_scheduler::IScheduler;

use self::e2rcore::implement::render::renderer_gl;
use self::e2rcore::implement::render::util_gl;
use self::e2rcore::implement::render::texture;
use self::e2rcore::implement::render::camera;
use self::e2rcore::implement::render::light;
use self::e2rcore::implement::render::mesh;
use self::e2rcore::implement::render::primitive;

use self::e2rcore::implement::ui::ui_cam::UiCam;

use self::e2rcore::implement::cam::trackball::TrackBall;

use self::mazth::mat;

use self::rand::Rng;
use self::image::GenericImage;

use self::rand::distributions::{IndependentSample, Range};

use std::env;

use std::collections::{ HashSet, HashMap };

use self::e2rcore::interface::i_kernel::IKernel;

use self::e2rcore::implement::kernel::kernel_impl_001::Kernel;

//todo: put this somewhere else
pub fn file_open( file_path: & str ) -> Option<String> {
    let path = File::open( file_path ).expect("file path open invalid");
    let mut buf_reader = BufReader::new(path);
    let mut contents = String::new();
    match buf_reader.read_to_string( & mut contents ){
        Err( e ) => { error!("{}", e ); return None },
        _ => (),
    }
    Some(contents)
}

#[derive(Clone, Debug)]
pub struct GameState {
    _exit: bool,
    _continue_compute: bool,
    _time_game: f32,
    _is_init_run_first_time: bool,

    pub _row: isize,
    pub _col: isize,
    pub _total: isize,
    pub _board: Vec< Vec< Option< GamePiece > > >,
    pub _n: isize,
    pub _piece_count: isize,
    pub _piece_gen_distr: HashMap< GamePiece, f32 >,

    pub _is_player_turn: bool,
}

impl Default for GameState {
    fn default() -> GameState {
        let row = 10;
        let col = 10;
        let num_cells = row * col;
        let n = 3;
        assert!( n <= num_cells );
        let piece_weight = [ ( GamePiece::Grass, 0.5 ),
                               ( GamePiece::Bush, 0.25 ),
                               ( GamePiece::Tree, 0.125 ),
                               ( GamePiece::House, 0.0625 ),
                               ( GamePiece::Bear, 0.03 ),
                               ( GamePiece::Cemetery, 0.015 ),
                               ( GamePiece::Church, 0.01 ),
        ].to_vec();

        let partition = piece_weight.iter().fold( 0., |accum, x| accum + x.1 );
        let mut accum = 0.0;
        let piece_distr = piece_weight.iter().map( |x| {
            accum += x.1;
            ( x.0, accum / partition ) 
        }).collect();

        GameState {
            _exit: false,
            _continue_compute: false,
            _time_game: 0.0,
            _is_init_run_first_time: false,

            _row: row,
            _col: col,
            _total: row * col,
            _board: vec![ vec![ None; col as usize ]; row as usize ],
            _n: n,
            _piece_count: 0,
            _piece_gen_distr: piece_distr,

            _is_player_turn: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameStateChangePending {
    
}

impl Default for GameStateChangePending {
    fn default() -> GameStateChangePending {
        GameStateChangePending {
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameStateChangeApply {
    _end_compute: bool,
}

impl Default for GameStateChangeApply {
    fn default() -> GameStateChangeApply {
        GameStateChangeApply {
            _end_compute: false,
        }
    }
}

impl From< ComputeUnit > for GameStateChangeApply {
    fn from( _c: ComputeUnit ) -> Self {
        match _c {
            ComputeUnit::SignalEndCompute => {
                Self {
                    _end_compute: true
                }
            },
            _ => {
                Default::default()
            },
        }
    }
}

#[derive(Clone)]
pub enum ComputeUnit {
    SignalEndCompute,
    TBD,
}

#[derive(Clone)]
pub struct ComputeSchedule {
    _compute_units: Vec< ComputeUnit >,
    _index: usize,
}

impl IScheduler for ComputeSchedule {
    type Item = ComputeUnit;
    fn new( _items: &[Self::Item] ) -> ComputeSchedule {
        ComputeSchedule {
            _compute_units: _items.to_vec(),
            _index: 0,
        }
    }
}

impl Iterator for ComputeSchedule {
    type Item = Vec< ComputeUnit >;
    fn next( & mut self ) -> Option< Self::Item > {
        if self._index >= self._compute_units.len() {
            None
        } else {
            //todo
            let s = Some( vec![ self._compute_units[ self._index ].clone() ] );
            self._index += 1;
            s
        }
    }
}

impl From< (GameState, GameStateChangeApply) > for GameState {
    fn from( (_s, _a): (GameState, GameStateChangeApply) ) -> Self {
        //todo
        let mut s = _s.clone();
        if _a._end_compute {
            s._continue_compute = false;
        }
        s
    }
}

pub enum RenderObj {
    InitialRender { _path_shader_vs: String, _path_shader_fs: String },
    TestGeometry { _time_game: f32, _light: light::LightAdsPoint, _camera: camera::Cam },
}


impl From< RenderObj > for Vec< renderer_gl::Event > {
    fn from( _r: RenderObj ) -> Self {
        match _r {
            RenderObj::InitialRender{ _path_shader_vs, _path_shader_fs } => {
                let mut render_events = vec![];
                
                info!("game logic: first time initialization.");

                let vs_src = file_open( _path_shader_vs.as_str() ).expect("vertex shader not retrieved");
                let fs_src = file_open( _path_shader_fs.as_str() ).expect("fragment shader not retrieved");
                let event_load_shader = renderer_gl::Event::LoadShader(
                    vec![
                        ( vs_src, util_gl::ShaderType::VERTEX ),
                        ( fs_src, util_gl::ShaderType::FRAGMENT ),
                    ] );
                render_events.push( event_load_shader );

                let img = image::open( &Path::new( "core/asset/images/texture0.jpg" ) ).unwrap();
                debug!( "image dimension: {:?}", img.dimensions() );
                debug!( "image type: {:?}", img.color() );
                
                let texture0 = texture::Texture::from( &img );
                let texture_data = Vec::from( texture0 );
                let ( w, h ) = img.dimensions();
                let event_load_texture = renderer_gl::Event::LoadTexture( String::from("texture0"), texture_data, w as _, h as _ );
                render_events.push( event_load_texture );

                info!( "press q to quit." );

                render_events
            },
            RenderObj::TestGeometry{ _time_game, _light, _camera } =>{
                let mut render_events = vec![];
                
                //create some meshes for test:
                //set triangle vert positions and normals
                let mut mesh = mesh::Mesh::init( 0 );
                mesh._pos.extend_from_slice( &[ mat::Mat3x1 { _val: [-1f32, -1f32, -1f32 ] },
                                                mat::Mat3x1 { _val: [ 5f32, -1f32, -1f32 ] },
                                                mat::Mat3x1 { _val: [-1f32,  1f32, -1f32 ] },
                                                mat::Mat3x1 { _val: [ 4f32, -1f32, 15f32 ] },
                                                mat::Mat3x1 { _val: [ 6f32, -1f32, 15f32 ] },
                                                mat::Mat3x1 { _val: [ 4f32,  1f32, 15f32 ] }, ] );

                mesh._normal.extend_from_slice( &[ mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                                   mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                                   mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                                   mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                                   mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] },
                                                   mat::Mat3x1 { _val: [ 0f32, 0f32, 1f32 ] }, ] );
                
                mesh._tc.extend_from_slice( &[ mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                               mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                               mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                               mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                               mat::Mat2x1 { _val: [ 0f32, 0f32 ] },
                                               mat::Mat2x1 { _val: [ 0f32, 0f32 ] }, ] );

                let mesh_copy = mesh.clone();

                let mut mesh2 = mesh_copy.clone();
                mesh2._pos.clear();
                mesh2._pos.extend_from_slice( &[ mat::Mat3x1 { _val: [-1f32+ _time_game, -1f32, -1f32 ] },
                                                 mat::Mat3x1 { _val: [ 5f32+_time_game, -1f32, -1f32 ] },
                                                 mat::Mat3x1 { _val: [-1f32+_time_game,  1f32, -1f32 ] },
                                                 mat::Mat3x1 { _val: [ 4f32+_time_game, -1f32, 15f32 ] },
                                                 mat::Mat3x1 { _val: [ 6f32+_time_game, -1f32, 15f32 ] },
                                                 mat::Mat3x1 { _val: [ 4f32+_time_game,  1f32, 15f32 ] }, ] );
                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( mesh2 ) ) );

                let prim_box = primitive::Poly6 { _pos: mat::Mat3x1 { _val: [ -5f32, -10f32, 5f32 ] },
                                                   _radius: 5f32 };

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( prim_box ) ) );

                let prim_sphere = primitive::SphereIcosahedron::init( mat::Mat3x1 { _val: [ -20f32, -10f32, 0f32 ] }, 5f32 );

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( prim_sphere ) ) );
                
                let l = &_light;
                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( l.clone() ) ) );

                render_events.push( renderer_gl::Event::AddObj( i_ele::Ele::init( _camera.clone() ) ) );

                render_events
            },
        }
    }
}

pub struct GameLogic {
    //todo
    _is_init: bool,
    _lights: Vec< light::LightAdsPoint >, //could move this to be generated by game logic or within a connecting adaptor between game logic and render interface
    _cameras: Vec< camera::Cam >, //todo: replace with camera wrapper
    // _cameras_wrapper: Vec< Box< ICamera > >,
    _delta: f32, //test parameter for object velocity
    _path_shader_vs: String,
    _path_shader_fs: String,
    _state: GameState,
    _uicam: UiCam,
}

impl IGameLogic for GameLogic {

    type EventInput = InputFiltered;
    type EventRender = renderer_gl::Event;
    type GameState = GameState;
    type GameStateChangePending = GameStateChangePending;
    type GameStateChangeApply = GameStateChangeApply;
    type ComputeUnit = ComputeUnit;
    type ComputeSchedule = ComputeSchedule;
    type RenderObj = RenderObj;

    fn new() -> GameLogic {
        let mut ret = GameLogic {
            _is_init: false,
            _lights: vec![],
            _cameras: vec![],
            // _cameras_wrapper: vec![],
            _delta: 0f32,
            _path_shader_vs: String::new(),
            _path_shader_fs: String::new(),
            _state: Default::default(),
            _uicam: UiCam {
                _trackball: TrackBall::new(500.,500.),
                .. Default::default()
            },
        };
        
        //lights
        let mut rng = rand::thread_rng();
        for i in 0..50 {
            let pos_x = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 3f32;
            let pos_y = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 - 4f32;
            let pos_z = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 6f32 + 10f32;
            let colour_r = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_g = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let colour_b = ( (rng.gen::<u8>() % 100) as f32 / 100f32 ) * 1f32;
            let l = light::LightAdsPoint {
                _id: i as u64,
                _pos: mat::Mat3x1 { _val: [ pos_x, pos_y, pos_z ] },
                _ads_val_spec: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_diff: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
                _ads_val_amb: mat::Mat3x1 { _val: [ colour_r, colour_g, colour_b ] },
            };
            ret._lights.push( l );
        }

        //camera
        let fov = 120f32;
        let aspect = 1f32;
        let near = 0.001f32;
        let far = 1000f32;
        let cam_foc_pos = mat::Mat3x1 { _val: [0f32, 0f32, 5f32] };
        let cam_up = mat::Mat3x1 { _val: [0f32, 1f32, 0f32] };
        let cam_pos = mat::Mat3x1 { _val: [5f32, 5f32, 20f32] };
        let cam_id = 0;
        let cam = camera::Cam::init( cam_id, fov, aspect, near, far, cam_pos, cam_foc_pos, cam_up );
        ret._cameras.push( cam );

        ret
    }

    ///do some initialization
    fn run_init_hook( & mut self ) -> Result< (), & 'static str > {
        self._path_shader_vs = String::from("core/asset/shader/ads.vs"); //some hard coded paths for now
        self._path_shader_fs = String::from("core/asset/shader/ads.fs");
        Ok( () )
    }

    ///computes changed game state given user inputs and current game state
    fn transition_states( & mut self, inputs: & [ InputFiltered ] ) -> GameStateChangePending {
        //todo

        for i in inputs.iter() {
            match i {
                &InputFiltered::Button { key: KeyCode::Q, .. } => {
                    self._state._exit = true;
                },
                _ => {},
            };
            self._uicam.process( i );
        }        

        self.set_continue_compute( true );

        // state_change
        Default::default()
    }
    fn get_states( & mut self ) -> & Self::GameState {
        & self._state
    }

    fn get_states_mut( & mut self ) -> & mut Self::GameState {
        & mut self._state
    }
    fn set_continue_compute( & mut self, b: bool ) {
        self._state._continue_compute = b;
    }
    fn continue_compute( & mut self ) -> bool {
        self._state._continue_compute
    }
    fn get_computations( & mut self, _changed_game_state: & GameStateChangePending ) -> Vec< ComputeUnit > {
        //todo: transform changed game state to additional computations

        let mut _compute_units = vec![];

        //append this to signal compute cycle is complete
        _compute_units.push( ComputeUnit::SignalEndCompute );

        _compute_units
    }
    fn schedule_computes( & mut self, _computes: Vec< ComputeUnit > ) -> Vec< Self::ComputeSchedule > {
        //todo
        let mut _compute_schedule = vec![];

        _compute_schedule
    }
    fn get_renderable_components( & mut self ) -> Vec< RenderObj > {

        //todo: use game specific game logic to produce render objects instead

        // {
        //     let test_impl = self.get_game_impl();
        // }
        
        let mut v = vec![];

        if !self._state._is_init_run_first_time {
            //does this once to setup some shaders
            self._state._is_init_run_first_time = true;
            let initial_render = RenderObj::InitialRender { _path_shader_fs: self._path_shader_fs.clone(),
                                                             _path_shader_vs: self._path_shader_vs.clone() };
            v.push( initial_render );
        }

        
        //update camera
        
        let mut focus = self._cameras[0]._focus.clone();
        let mut pos = self._cameras[0]._pos_orig;
        self._cameras[0]._pos_orig = pos;

        let axis_front = focus.minus( & pos ).unwrap().normalize().unwrap();
        let axis_right = axis_front.cross( & self._cameras[0]._up ).unwrap().normalize().unwrap();

        let move_front = axis_front.scale( self._uicam._move.0 as f32 * 0.3 ).unwrap();
        let move_right = axis_right.scale( self._uicam._move.1 as f32 * 0.3 ).unwrap();
        let move_up = self._cameras[0]._up.normalize().unwrap().scale( self._uicam._move.2 as f32 * 0.3 ).unwrap();
        
        pos = pos.plus( & move_front.plus( & move_right ).unwrap().plus( & move_up ).unwrap() ).unwrap();
        self._uicam._move = ( 0, 0, 0 );

        let rot_matrix = self._uicam._trackball.get_rot().to_rotation_matrix( true );
        self._uicam._trackball.reset_rot();
        let offset = mat::Mat4x1 { _val: [ pos[0] - focus[0],
                                           pos[1] - focus[1],
                                           pos[2] - focus[2],
                                           0. ] };
        
        let pos_update = rot_matrix.mul_mat4x1( & offset ).unwrap();

        let pos_new = focus.plus( & mat::Mat3x1 { _val: [ pos_update[0], pos_update[1], pos_update[2] ] } ).unwrap();
        self._cameras[0].update_pos( pos_new, focus );

        self._cameras[0]._pos_orig = pos_new;

        //dummy geometry to render
        v.push( RenderObj::TestGeometry { _time_game: self._state._time_game,
                                           _light: self._lights[0].clone(),
                                           _camera: self._cameras[0].clone() } );
        
        self._state._time_game -= 0.01;

        v
    }
    fn filter_renderables( & mut self, _r: Vec< RenderObj > ) -> Vec< RenderObj > {
        //todo: add spatial accelerator algo here
        _r
    }

    fn should_exit( & mut self ) -> bool {
        self._state._exit
    }

    // fn get_game_impl( & mut self ) -> & mut GameImpl {
    //     & mut self._game_impl
    // }
}

#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
pub enum GamePiece {
    Grass,
    Bush,
    Tree,
    House,
    Bear,
    Cemetery,
    Church,
}

impl GameLogic {
    pub fn place_piece( & mut self, piece: GamePiece, to: (isize,isize) ) -> bool {
        if to.0 as usize >= self._state._board[0].len() ||
            to.1 as usize >= self._state._board[0].len() {
                false
            } else {
                let is_empty = match &self._state._board[ to.0 as usize ][ to.1 as usize ] {
                    &None => {
                        true
                    },
                    &Some(ref _x) => {
                        false
                    },
                };
                if is_empty {
                    self._state._board[ to.0 as usize ][ to.1 as usize ] = Some( piece );
                    self.update_placement( to );
                    true
                } else {
                    false
                }
            }
    }
    pub fn piece_replacement( & self, _piece: GamePiece ) -> GamePiece {
        match _piece {
            GamePiece::Grass => { GamePiece::Bush },
            GamePiece::Bush => { GamePiece::Tree },
            GamePiece::Tree => { GamePiece::House },
            GamePiece::House => { GamePiece::House }, //todo further progress instead of capping
            GamePiece::Bear => { GamePiece::Cemetery },
            GamePiece::Cemetery => { GamePiece::Church },
            GamePiece::Church => { GamePiece::Church }, //todo further progress instead of capping
        }
    }
    pub fn update_placement( & mut self, loc: (isize,isize) ) {
        let mut hs = HashSet::new();
        let mut q = vec![];
        
        let piece =
            match &self._state._board[ loc.0 as usize ][ loc.1 as usize ] {
                &None => { panic!("cannot update empty placement"); },
                &Some(ref x) => { x.clone() },
            };
        
        q.push(loc);
        while !q.is_empty() {
            let a = q.pop().unwrap();
            let a_neighbour = [   ( a.0 - 1, a.1 ),
                                    ( a.0 + 1, a.1 ),
                                    ( a.0, a.1 - 1 ),
                                    ( a.0, a.1 + 1 )
            ].to_vec().into_iter()
                .filter( |x| {
                    x.0 >= 0 &&
                        x.0 < self._state._row &&
                        x.1 >= 0 &&
                        x.1 < self._state._col &&
                        &self._state._board[x.0 as usize][x.1 as usize] == &Some(piece) } );

            hs.insert( a );
            q.extend( a_neighbour );
        }

        if q.len() >= self._state._n as usize {
            let replacement = self.piece_replacement( piece );        
            
            for i in hs.iter() {
                self._state._board[i.0 as usize][i.1 as usize] = None;
            }
            
            self._state._board[loc.0 as usize][loc.1 as usize] = Some( replacement );
        }

    }
    fn is_game_over( & self ) -> bool {
        self._state._piece_count >= self._state._total
    }
    fn get_available_moves( & self ) -> Vec< (isize,isize) > {
        let mut moves = vec![];
        for i in 0..self._state._row {
            for j in 0..self._state._col {
                match self._state._board[i as usize][j as usize] {
                    None => {
                        moves.push( ( i, j ) );
                    },
                    _ => {},
                }
            }
        }
        moves
    }
    fn get_available_pieces( & self ) -> Vec< GamePiece > {
        [   GamePiece::Grass,
            GamePiece::Bush,
            GamePiece::Tree,
            GamePiece::House,
            GamePiece::Bear,
            GamePiece::Cemetery,
            GamePiece::Church,
        ].to_vec()
    }
    fn generate_random_location( & self ) -> Option< (isize,isize) > {
        
        let moves = self.get_available_moves();

        if moves.is_empty() {
            return None
        }
        
        let between = Range::new( 0, moves.len() );
        let mut rng = rand::thread_rng();
        
        let idx = between.ind_sample(&mut rng);

        assert!(idx < moves.len() );

        Some( moves[idx] )
    }
    fn generate_random_piece( & self ) -> Option< GamePiece > {

        let pieces = self.get_available_pieces();

        let mut piece_weight = vec![];
        
        for i in pieces.iter() {
            let is_present = self._state._piece_gen_distr.contains_key(i);
            if is_present {
                let w = self._state._piece_gen_distr.get( i ).unwrap();
                piece_weight.push( ( *i, *w ) );
            }
        }
        
        let partition = piece_weight.iter().fold( 0., |accum, x| accum + x.1 );
        let mut accum = 0.0;
        let piece_distr = piece_weight.iter().map( |x| {
            accum += x.1;
            ( x.0, accum / partition ) 
        }).collect::<Vec<_>>();
        
        let between = Range::new( 0.0, 1.0 );
        let mut rng = rand::thread_rng();
        
        let rand_fraction = between.ind_sample(&mut rng);

        let mut choosen_piece = None;
        for i in piece_distr.iter() {
            if rand_fraction <= i.1 {
                choosen_piece = Some(i.0);
                break;
            }
        }

        choosen_piece
    }
    fn get_player_move( & mut self ) -> ( GamePiece, (isize,isize) ){
        unimplemented!();
    }
    fn send_player_piece( & mut self, _piece: GamePiece ) {
        unimplemented!();
    }
    fn quit_game( & mut self ) {
        unimplemented!();
    }
    fn main_state_change( & mut self ) {
        //todo
        if self.is_game_over() {
            self.quit_game();
        } else {
            let ( piece, loc ) = self.get_player_move();
            self.place_piece( piece, loc );
            self.update_placement( loc );

            let rand_piece = self.generate_random_piece();
            match rand_piece {
                None => {
                    self.quit_game();
                },
                Some(piece)  => {
                    self.send_player_piece( piece );
                },
            }
        }
    }
}

#[main]
fn main() {

    env::set_var("LOG_SETTING", "info" );
    
    pretty_env_logger::init_custom_env( "LOG_SETTING" );
    
    let mut k : Kernel<GameLogic> = Kernel::new().unwrap();
    
    k.run();
    
}
