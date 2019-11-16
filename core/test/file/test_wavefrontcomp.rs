use implement::file::md5common;
use implement::file::wavefrontobj;
use implement::file::wavefrontcomp;

#[test]
#[ignore] //temporary
fn test_wavefrontobjcomp(){
    let file_content = md5common::file_open( "core/asset/obj/25-vaz-2108/2108_tri.obj" ).expect("file open invalid");
    println!("file content length: {}", file_content.len() );

    let _wavefront_objs = wavefrontobj::parse( &file_content ).expect("parse unsuccessful");

    println!( "number of groups: {}", _wavefront_objs._groups.len() );

    let obj_compute = wavefrontcomp::process( & _wavefront_objs ).expect( "wavefront compute unsuccessful" );

    assert_eq!( obj_compute._batch_vert.len(), obj_compute._batch_normal.len() );
    assert_eq!( obj_compute._batch_tc.len(), obj_compute._batch_normal.len()/3*2 );

    // println!( "obj_compute._bbox_upper: {:?}", obj_compute._bbox_upper );
    // println!( "obj_compute._bbox_lower: {:?}", obj_compute._bbox_lower );
    
    for i in 0..3 {
        assert!( obj_compute._bbox_lower[i] < obj_compute._bbox_upper[i] );
    }
}
