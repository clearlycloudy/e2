extern crate mazth;

use ::std::f32;

use self::mazth::mat;

pub fn perspective( fov: f32, aspect: f32, near: f32, far: f32 ) -> mat::Mat4< f32 > {
    //fov is the full field of view in y-plane, eg: fovy = fov/2
    let half_tan = (fov * f32::consts::PI / 360.0f32).tan();
    //creates a frustum
    let mut out : mat::Mat4<f32> = Default::default();
    *out.index_mut(0, 0) = 1.0/(aspect*half_tan);
    *out.index_mut(1, 1) = 1.0/half_tan;
    *out.index_mut(2, 2) = (far+near)/(near-far);
    *out.index_mut(3, 2) = -1f32;
    *out.index_mut(2, 3) = (2.0f32*far*near)/(near-far);
    out
}

pub fn look_at( eye: mat::Mat3x1< f32 >, center: mat::Mat3x1< f32 >, up: mat::Mat3x1< f32 > ) -> mat::Mat4< f32 > {

    //compute viewing plane's normal vector
    let mut n : mat::Mat3x1<f32> = Default::default();
    for i in 0..3{
	n[i] = eye[i] - center[i];
    }
    n = n.normalize().unwrap();

    //compute 1 orthogonal vector to plane's normal vector
    let up_vec = up.normalize().unwrap();
    let u = up_vec.cross( &n ).unwrap().normalize().unwrap();
    
    //compute the other orthogonal vector to plane's normal vector
    let v = n.cross( &u ).unwrap().normalize().unwrap();

    //space_original = a_inverse * space_camera
    //a is a rotation matrix, which means a_inverse = a_transpose
    //a is finally augmented with translations on the last column
    let mut camera_view : mat::Mat4<f32> = Default::default();
    *camera_view.index_mut( 0, 0 ) = u[0];
    *camera_view.index_mut( 0, 1 ) = u[1];
    *camera_view.index_mut( 0, 2 ) = u[2];
    *camera_view.index_mut( 0, 3 ) = -1f32 * (eye[0]*u[0] + eye[1]*u[1] + eye[2]*u[2]);
    *camera_view.index_mut( 1, 0 ) = v[0];
    *camera_view.index_mut( 1, 1 ) = v[1];
    *camera_view.index_mut( 1, 2 ) = v[2];
    *camera_view.index_mut( 1, 3 ) = -1f32 * (eye[0]*v[0] + eye[1]*v[1] + eye[2]*v[2]);
    *camera_view.index_mut( 2, 0 ) = n[0];
    *camera_view.index_mut( 2, 1 ) = n[1];
    *camera_view.index_mut( 2, 2 ) = n[2];
    *camera_view.index_mut( 2, 3 ) = -1f32 * (eye[0]*n[0] + eye[1]*n[1] + eye[2]*n[2]);
    *camera_view.index_mut( 3, 0 ) = 0f32;
    *camera_view.index_mut( 3, 1 ) = 0f32;
    *camera_view.index_mut( 3, 2 ) = 0f32;
    *camera_view.index_mut( 3, 3 ) = 1f32;

    camera_view
}

pub fn invert_rotate_mat4( input: mat::Mat4<f32> ) -> mat::Mat4<f32> {
    let mut out = input;
    for i in 0..3 {
        for j in 0..3 {
            *out.index_mut( i, j ) = input.index( j, i );
        }
    }
    out
}

pub fn invert_z_mat4( input: mat::Mat4<f32> ) -> mat::Mat4<f32> {
    let mut out = input;
    *out.index_mut( 2, 3 ) = -1f32 * input.index( 2, 3 );
    out
}

pub fn invert_xyz_mat4( input: mat::Mat4<f32> ) -> mat::Mat4<f32> {
    let mut out = input;
    *out.index_mut( 0, 3 ) = -1f32 * input.index( 0, 3 );
    *out.index_mut( 1, 3 ) = -1f32 * input.index( 1, 3 );
    *out.index_mut( 2, 3 ) = -1f32 * input.index( 2, 3 );
    out
}

pub fn identity_mat4() -> mat::Mat4<f32> {
    mat::Mat4::<f32>::init( [1f32, 0f32, 0f32, 0f32,
                             0f32, 1f32, 0f32, 0f32,
                             0f32, 0f32, 1f32, 0f32,
                             0f32, 0f32, 0f32, 1f32 ],
                             true )
}
