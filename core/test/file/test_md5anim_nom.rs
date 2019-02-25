use interface::i_file::IParseStr;

use implement::file::md5common;
use implement::file::md5anim_nom::*;

#[test]
fn test_parse_md5anim_nom(){
    let file_content = md5common::file_open( "core/asset/md5/qshamblerattack01.md5anim" ).expect("file open invalid");
    println!("file content length: {}", file_content.len() );
    match <Md5AnimParser as IParseStr>::parse( &file_content ) {
        Ok( x ) => {
            println!( "{:?}", x );
        },
        _ => { panic!( "parse unsuccessful" ); },
    }
}
