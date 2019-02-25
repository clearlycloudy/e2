use interface::i_file::IParseStr;

use implement::file::md5common;
use implement::file::md5mesh_nom::*;

#[test]
fn test_parse_md5mesh(){
    let file_content = md5common::file_open( "core/asset/md5/qshambler.md5mesh" ).expect("file open invalid");
    println!("file content length: {}", file_content.len() );
    let _mesh_root = Md5MeshParser::parse( &file_content ).expect("parse unsuccessful");

    // for i in mesh_root._joints.iter() {
    //     println!( "joint name: {:?}, parent index: {:?}, pos: {:?}, orient: {:?}, rot: {:?}", i._name, i._parent_index, i._pos, i._orient, i._rot );
    // }
    
    // for (idx, i) in mesh_root._meshes.iter().enumerate() {
    //     println!( "mesh {} {{", idx );
    //     println!( "shader: {}", i._shader );
    //     println!( "numverts: {}", i._numverts );
    //     for j in i._verts.iter() {
    //         println!( "vert index: {}, tex coords: {:?}, weight start: {}, weight count: {}", j._index, j._tex_coords, j._weight_start, j._weight_count );
    //     }
    //     println!( "numtris: {}", i._numtris );
    //     for j in i._tris.iter() {
    //         println!( "tri index: {}, vert indices: {:?}", j._index, j._vert_indices );
    //     }
    //     println!( "numweights: {}", i._numweights );
    //     for j in i._weights.iter() {
    //         println!("weight index: {}, joint index: {}, weight bias: {}, pos: {:?}", j._index, j._joint_index, j._weight_bias, j._pos );
    //     }
    //     println!( "}}" );
    // }
}
