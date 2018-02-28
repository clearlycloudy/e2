extern crate mazth;

use self::mazth::i_shape::{ ShapeType, IShape };
use self::mazth::i_bound::IBound;
use self::mazth::i_vicinity::IVicinity;

use self::mazth::mat::Mat3x1;
use self::mazth::bound::AxisAlignedBBox;

#[derive(Debug, Clone)]
pub struct Frustum {
    ///a frustum of square top and bottom caps
    ///work in progress
    pub _ori: Mat3x1< f64 >,
    pub _halfedge_small: f64, //smaller of the 2 caps
    pub _halfedge_big: f64, //bigger of the 2 caps
    pub _halflength: f64, //center half length of the frustum
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}
    
impl Frustum {
    pub fn init( origin: &[f64], halflength: f64, halfedge_small: f64, halfedge_big: f64 ) -> Frustum {
        assert!( origin.len() == 3 );
        Frustum {
            _ori: Mat3x1 { _val: [ origin[0], origin[1], origin[2] ] },
            _halflength: halflength,
            _halfedge_small: halfedge_small,
            _halfedge_big: halfedge_big,
            _bound: AxisAlignedBBox::init( ShapeType::FRUSTUM, &[ &origin[0..3], &[ halflength ] ].concat() ),
            _vicinity: 0.000001f64,
        }
        
    }
}

impl IShape for Frustum {
    fn get_shape_data( & self ) -> Vec< f64 > {
        vec![ self._ori[0], self._ori[1], self._ori[2], self._halflength, self._halfedge_small, self._halfedge_big ]
    }
    fn get_type( & self ) -> ShapeType {
        ShapeType::SPHERE
    }
    fn get_bound( & self ) -> &IBound {
        &self._bound
    }
    fn get_intersect( & self, _other: & IShape ) -> ( bool, Option< Mat3x1< f64 > > ){
        unimplemented!();
    }
    fn get_support( & self, _v: & Mat3x1< f64 > ) -> Option< Mat3x1< f64 > > {
        unimplemented!();
    }
}

impl IVicinity< f64 > for Frustum {
    fn set_vicinity( & mut self, epsilon: f64 ) {
        self._vicinity = epsilon.abs();
    }
    fn within_vicinity( & self, a: f64, b: f64 ) -> bool {
        if a + self._vicinity >= b &&
           a - self._vicinity <= b {
            true
        } else {
            false
        }
    }
}
