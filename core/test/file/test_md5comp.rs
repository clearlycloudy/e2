use implement::file::md5common;
use implement::file::md5mesh;
use implement::file::md5anim;
use implement::file::md5rig;
use implement::file::md5comp;

#[test]
fn test_parse_md5comp(){
    let file_mesh = md5common::file_open( "core/asset/md5/qshambler.md5mesh" ).expect("md5mesh file open invalid");
    let file_anim = md5common::file_open( "core/asset/md5/qshamblerattack01.md5anim" ).expect("md5anim file open invalid");
    let mesh = match md5mesh::parse( &file_mesh ) {
        Ok( o ) => o,
        Err( e ) => panic!( e ),
    };
    let anim = match md5anim::parse( &file_anim ) {
        Ok( o ) => o,
        Err( e ) => panic!( e ),
    };
    let posecollection = match md5rig::process( & anim ) {
        Ok( o ) => o,
        Err( e ) => panic!( e ),
    };
    assert!( anim._numframes as usize == posecollection._frames.len() );

    for n in 0..posecollection._frames.len() as u64 - 1 {
        let _comp = match md5comp::process( & posecollection, & mesh, n, n+1, 0.0f32 ){
            Ok( o ) => o,
            Err( e ) => panic!( e ),
        };
        // println!( "number of meshes: {:?}", _comp._meshcomputes.len() );

        // let mut pos = vec![];
        // let mut nor = vec![];
        // for i in _comp._meshcomputes.iter() {
        //     for j in i._tris.iter() {
        //         for k in 0..3 {
        //             let idx_vert = j._vert_indices[ k ];
        //             let vert = & i._verts[ idx_vert as usize ];
        //             pos.push( vert._pos );
        //             nor.push( vert._normal );
        //         }
        //     }
        // }

        assert_eq!( _comp._batch_vert.len(), _comp._batch_normal.len() );
        assert_eq!( _comp._batch_vert.len() / 3, _comp._batch_tc.len() / 2 );
        
        println!( "min bbox pos: {:?}", _comp._bbox_lower );
        println!( "max bbox pos: {:?}", _comp._bbox_upper );

        // for i in &pos {
        //     println!( "pos: {:?}", i );
        // }
        // for i in &nor {
        //     // assert!( i[0] * i[0] + i[1] * i[1] + i[2] * i[2] <= 1. );
        //     println!( "normal: {:?}", i );
        // }
        // println!( "pos: {:?}", pos );
        // println!( "nor: {:?}", nor );
    }
}
