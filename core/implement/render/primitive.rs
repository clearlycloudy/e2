extern crate pretty_env_logger;
extern crate mazth;

use std::collections::HashMap;
use std::any::Any;

use interface::i_ele;
use interface::i_renderobj;
use interface::i_component;

use self::mazth::mat;

/// # 6 sided polyhedral / box
#[derive(Clone)]
pub struct Poly6 {
    /// # _pos := center of the box
    pub _pos: mat::Mat3x1< f32 >,
    pub _scale: mat::Mat3x1< f32 >,
    /// # _radius := approximation of the box size
    pub _radius: f32,
}

impl i_ele::IObjImpl for Poly6 {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store vertex data
        {
            let x0 = self._pos[0] - self._radius/2.0 * self._scale[0];
            let x1 = self._pos[0] + self._radius/2.0 * self._scale[0];

            let y0 = self._pos[1] - self._radius/2.0 * self._scale[1];
            let y1 = self._pos[1] + self._radius/2.0 * self._scale[1];

            let z0 = self._pos[2] - self._radius/2.0 * self._scale[2];
            let z1 = self._pos[2] + self._radius/2.0 * self._scale[2];

            let mut pos = vec![];
            let mut normal = vec![];
            let mut tc = vec![];

            //face 1 (front) trig 1
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 1 (front) trig 2
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, 1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 2 (back) trig 1.0
            pos.extend_from_slice( &[ x0, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 2 (back) trig 2
            pos.extend_from_slice( &[ x1, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 0.0, -1.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 3 (left) trig 1.0
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z1 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z0 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 3 (left) trig 2
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z0 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y0, z0 ] );
            normal.extend_from_slice( &[ -1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 4 (right) trig 1.0
            pos.extend_from_slice( &[ x1, y0, z1 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z0 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z0 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 4 (right) trig 2
            pos.extend_from_slice( &[ x1, y0, z1 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z0 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z1 ] );
            normal.extend_from_slice( &[ 1.0, 0.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 5 (top) trig 1.0
            pos.extend_from_slice( &[ x0, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 5 (top) trig 2
            pos.extend_from_slice( &[ x0, y1, z1 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y1, z0 ] );
            normal.extend_from_slice( &[ 0.0, 1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 6 (bottom) trig 1.0
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x0, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            //face 6 (bottom) trig 2
            pos.extend_from_slice( &[ x0, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z0 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            pos.extend_from_slice( &[ x1, y0, z1 ] );
            normal.extend_from_slice( &[ 0.0, -1.0, 0.0 ] );
            tc.extend_from_slice( &[ 0.0, 0.0 ] );

            let ele_len = pos.len();

            let data_map : HashMap< i_renderobj::BuffDataType, Vec< f32 > > =  [ ( i_renderobj::BuffDataType::POS, pos ),
                                                                                 ( i_renderobj::BuffDataType::NORMAL, normal ),
                                                                                 ( i_renderobj::BuffDataType::TC, tc ) ].iter().cloned().collect();

            let c = i_component::ComponentRenderBuffer {
                _data_dict: data_map,
            };
            components.push( Box::new(c) );
            trace!( "load into ComponentRenderBuffer: Poly6: vertex count:{}", ele_len / 3 );
        }
        //store uniform data
        {
            
            
        }
        Ok( () )
    }
}

#[derive(Clone)]
pub struct SphereIcosahedron {
    pub _pos: mat::Mat3x1< f32 >,
    pub _radius: f32,
    pub _index: usize,
    pub _positions: Vec<[f32;3]>,
}

impl SphereIcosahedron {
    pub fn init( pos: mat::Mat3x1< f32 >, radius: f32 ) -> SphereIcosahedron {
        SphereIcosahedron {
            _pos: pos,
            _radius: radius,
            _index: 0,
            _positions: vec![],
        }
    }
    /// # add vertex to mesh, fix position to be on unit sphere, return index
    pub fn add_vertex( & mut self, p: &[f32] ) {
        let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2] ).sqrt();
        self._positions.push( [ p[0]/length, p[1]/length, p[2]/length ] );
        self._index += 1;
    }
    /// # return index of point in the middle of p1 and p2
    pub fn get_middle_point( & mut self, p1: usize, p2: usize ) -> usize {
        // first check if we have it already
        let first_is_smaller : bool = p1 < p2;
        let smaller_index = if first_is_smaller { p1 } else { p2 };
        let greater_index = if first_is_smaller { p2 } else { p1 };
        let _key = (smaller_index << 32) + greater_index;
        
        // int ret;
        // if (this.middlePointIndexCache.TryGetValue(key, out ret))
        // {
        //     return ret;
        // }

        // not in cache, calculate it
        let point1 = self._positions[p1];
        let point2 = self._positions[p2];
        let middle = [ (point1[0] + point2[0]) / 2.0,
                        (point1[1] + point2[1]) / 2.0,
                        (point1[2] + point2[2]) / 2.0 ];

        // add vertex makes sure point is on unit sphere
        let i = self._index;
        self.add_vertex( &middle );

        // // store it, return index
        // this.middlePointIndexCache.Add(key, i);
        i
    }
}

impl i_ele::IObjImpl for SphereIcosahedron {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store vertex data
        {
            //starting icosahedron
            let t = ( 1f32 + 5f32.sqrt() ) / 2f32;

            self.add_vertex( &[-1f32,  t,  0f32 ] );
            self.add_vertex( &[ 1f32,  t,  0f32 ] );
            self.add_vertex( &[-1f32, -t,  0f32 ] );
            self.add_vertex( &[ 1f32, -t,  0f32 ] );

            self.add_vertex( &[ 0f32, -1f32,  t ] );
            self.add_vertex( &[ 0f32,  1f32,  t ] );
            self.add_vertex( &[ 0f32, -1f32, -t ] );
            self.add_vertex( &[ 0f32,  1f32, -t ] );

            self.add_vertex( &[ t,  0f32, -1f32 ] );
            self.add_vertex( &[ t,  0f32,  1f32 ] );
            self.add_vertex( &[-t,  0f32, -1f32 ] );
            self.add_vertex( &[-t,  0f32,  1f32 ] );
            
            // create 20f32 triangles of the icosahedron
            let mut faces = vec![];

            // 5 faces around point 0
            faces.push( [0, 11, 5] );
            faces.push( [0, 5, 1] );
            faces.push( [0, 1, 7] );
            faces.push( [0, 7, 10] );
            faces.push( [0, 10, 11] );

            // 5 adjacent faces
            faces.push( [1, 5, 9] );
            faces.push( [5, 11, 4] );
            faces.push( [11, 10, 2] );
            faces.push( [10, 7, 6] );
            faces.push( [7, 1, 8] );

            // 5 faces around point 3
            faces.push( [3, 9, 4] );
            faces.push( [3, 4, 2] );
            faces.push( [3, 2, 6] );
            faces.push( [3, 6, 8] );
            faces.push( [3, 8, 9] );

            // 5 adjacent faces
            faces.push( [4, 9, 5] );
            faces.push( [2, 4, 11] );
            faces.push( [6, 2, 10] );
            faces.push( [8, 6, 7] );
            faces.push( [9, 8, 1] );

            let iterations = 2;
            // refine triangles
            for _ in 0..iterations {
                let mut faces2 = vec![];
                for tri in faces.iter() {
                    // subdivide triangle into 4
                    let a = self.get_middle_point( tri[0], tri[1] );
                    let b = self.get_middle_point( tri[1], tri[2] );
                    let c = self.get_middle_point( tri[2], tri[0] );
                    faces2.push( [ tri[0], a, c ] );
                    faces2.push( [ tri[1], b, a ] );
                    faces2.push( [ tri[2], c, b ] );
                    faces2.push( [ a, b, c ] );
                }
                faces = faces2;
            }

            let mut pos = vec![];
            let mut normal = vec![];
            let mut tc = vec![];
            let r = self._radius;
            let offset = self._pos;
            for i in faces.iter() {
                let x = self._positions[ i[0] ].into_iter().enumerate().map(|(k,o)| o*r + offset[k] ).collect::<Vec<f32> >();
                let y = self._positions[ i[1] ].into_iter().enumerate().map(|(k,o)| o*r + offset[k] ).collect::<Vec<f32> >();
                let z = self._positions[ i[2] ].into_iter().enumerate().map(|(k,o)| o*r + offset[k] ).collect::<Vec<f32> >();

                let n = mat::Mat3x1 { _val: [ z[0]-y[0], z[1]-y[1], z[2]-y[2] ] }.cross( & mat::Mat3x1 { _val: [ x[0]-y[0], x[1]-y[1], x[2]-y[2] ] } ).unwrap().normalize().unwrap();
                pos.extend_from_slice( &x[..] );
                pos.extend_from_slice( &y[..] );
                pos.extend_from_slice( &z[..] );
                normal.extend_from_slice( &n._val[..] );
                normal.extend_from_slice( &n._val[..] );
                normal.extend_from_slice( &n._val[..] );
                tc.extend_from_slice( &[ 0f32; 6 ] );            
            }

            let ele_len = pos.len();
            
            let data_map : HashMap< i_renderobj::BuffDataType, Vec< f32 > > =  [ ( i_renderobj::BuffDataType::POS, pos ),
                                                                                 ( i_renderobj::BuffDataType::NORMAL, normal ),
                                                                                 ( i_renderobj::BuffDataType::TC, tc ) ].iter().cloned().collect();
            
            let c = i_component::ComponentRenderBuffer {
                _data_dict: data_map,
            };
            components.push( Box::new(c) );
            trace!( "load into ComponentRenderBuffer: SphereIcosahedron: vertex count:{}", ele_len / 3 );
        }
        //store uniform data
        {
            
        }
        Ok( () )
    }
}

//todo
#[derive(Clone)]
pub struct Point {
    pub _pos: mat::Mat3x1< f32 >,
    pub _radius: f32,
}

impl i_ele::IObjImpl for Point {
    fn as_any( & self ) -> & Any {
        self
    }
    fn update_components( & mut self, components: & mut Vec< Box< i_component::IComponent > > ) -> Result< (), & 'static str > {

        //store vertex data
        {
            let mut pos = vec![];
            let mut normal = vec![];
            let mut tc = vec![];

            pos.extend_from_slice( &self._pos._val[..] );
            //todo: remove dummy noromal and texture coordinate
            for i in 0..self._pos._val.len() {
                if i % 3 == 2 {
                    normal.push( 1f32 );
                }else{
                    normal.push( 0f32 );
                }
                tc.push( 0f32 );
            }

            let ele_len = pos.len();

            let data_map : HashMap< i_renderobj::BuffDataType, Vec< f32 > > =  [ ( i_renderobj::BuffDataType::POS, pos ),
                                                                                 ( i_renderobj::BuffDataType::NORMAL, normal ),
                                                                                 ( i_renderobj::BuffDataType::TC, tc ) ].iter().cloned().collect();

            let c = i_component::ComponentRenderBuffer {
                _data_dict: data_map,
            };
            components.push( Box::new(c) );
            trace!( "load into ComponentRenderBuffer: Point: vertex count:{}", ele_len / 3 );
        }
        Ok( () )
    }
}

//todo
pub struct Line {
    pub _pos_start: mat::Mat3x1< f32 >,
    pub _pos_end: mat::Mat3x1< f32 >,
}

