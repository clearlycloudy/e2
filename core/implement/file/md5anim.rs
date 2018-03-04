use std::str;
use std::collections::HashMap;
use std::str::FromStr;

use interface::i_md5::anim::*;

use implement::file::md5common;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum Token {
    Version,
    Commandline,
    Numframes,
    Numjoints,
    Framerate,
    Numanimatedcomponents,
    Hierarchy,
    Bounds,
    Baseframe,
    Frame,
}

pub fn parse( file_content: &str ) -> Result< Md5AnimRoot, & 'static str > {
    let mut hm = HashMap::new();
    hm.insert( "MD5Version", Token::Version );
    hm.insert( "commandline", Token::Commandline );
    hm.insert( "numFrames", Token::Numframes );
    hm.insert( "numJoints", Token::Numjoints );
    hm.insert( "frameRate", Token::Framerate );
    hm.insert( "numAnimatedComponents", Token::Numanimatedcomponents );
    hm.insert( "hierarchy", Token::Hierarchy );
    hm.insert( "bounds", Token::Bounds );
    hm.insert( "baseframe", Token::Baseframe );
    hm.insert( "frame", Token::Frame );

    let mut idx = 0usize;
    let mut anim_root = Md5AnimRoot::init();

    loop {
        let ( tok, kw_tok, idx_s, idx_e, mut idx_next ) = md5common::tokenize( &file_content[0..], idx, & mut hm );
        match tok {
            md5common::Token::End => {
                println!("reached end of content at index {}.", idx_next );
                break;
            },
            md5common::Token::Keyword => {
                match kw_tok {
                    Some(Token::Version) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._md5ver = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse version failed");
                        idx_next = idx_next2;
                        println!("version: {:?}.", anim_root._md5ver );
                    },
                    Some(Token::Commandline) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._cmdline = String::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse cmdline failed");
                        idx_next = idx_next2;
                        println!("cmdline: {:?}.", anim_root._cmdline );
                    },
                    Some(Token::Numframes) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._numframes = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse numframes failed");
                        idx_next = idx_next2;
                        println!("numframes: {:?}.", anim_root._numframes );
                    },
                    Some(Token::Numjoints) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._numjoints = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse cmdline failed");
                        idx_next = idx_next2;
                        println!("numjoints: {:?}.", anim_root._numjoints );
                    },
                    Some(Token::Framerate) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._framerate = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse framerate failed");
                        idx_next = idx_next2;
                        println!("framerate: {:?}.", anim_root._framerate );
                    },
                    Some(Token::Numanimatedcomponents) => {
                        let ( _tok2, _kw_tok2, idx_s2, idx_e2, idx_next2 ) = md5common::tokenize( &file_content[0..], idx_next, & mut hm );
                        anim_root._num_animated_components = u64::from_str( &file_content[idx_s2..idx_e2] ).expect("md5mesh parse numanimatedcomponents failed");
                        idx_next = idx_next2;
                        println!("numanimatedcomponents: {:?}.", anim_root._num_animated_components );
                    },
                    Some(Token::Hierarchy) => {
                        match process_hierarchy( &file_content[..], idx_next, & hm, anim_root._numjoints ) {
                            Ok( ( v, hierarchy ) ) => {
                                idx_next = v;
                                anim_root._hierarchy = hierarchy;
                            },
                            Err(e) => assert!( false, e )
                        }
                        println!("hierarchy count: {:?}.", anim_root._hierarchy.len() );
                    },
                    Some(Token::Bounds) => {
                        match process_bounds( &file_content[..], idx_next, & hm, anim_root._numframes ) {
                            Ok( ( v, bounds ) ) => {
                                idx_next = v;
                                anim_root._bounds = bounds;
                            },
                            Err(e) => assert!( false, e )
                        }
                        println!("bounds count: {:?}.", anim_root._bounds.len() );
                    },
                    Some(Token::Baseframe) => {
                        match process_baseframe( &file_content[..], idx_next, & hm, anim_root._numjoints ) {
                            Ok( ( v, baseframe ) ) => {
                                idx_next = v;
                                anim_root._baseframe = baseframe;
                            },
                            Err(e) => assert!( false, e )
                        }
                        println!("baseframe joint count: {:?}.", anim_root._baseframe.len() );
                    },
                    Some(Token::Frame) => {
                        match process_frame( &file_content[..], idx_next, & hm, anim_root._num_animated_components ) {
                            Ok( ( v, frame ) ) => {
                                idx_next = v;
                                anim_root._frames.push( frame );
                            },
                            Err(e) => assert!( false, e )
                        }
                    },
                    _ => panic!("unexpected token detected at index {:?}. input: {:?}", idx_next, &file_content[idx_s..idx_e] ),
                }
            },
            _ => ()
        }
        idx = idx_next;
    }
    //verify parsed data
    assert!( anim_root._numjoints ==  anim_root._hierarchy.len() as u64 );
    assert!( anim_root._numjoints ==  anim_root._baseframe.len() as u64 );
    assert!( anim_root._frames.len() ==  anim_root._bounds.len() );
    let mut unique_frames = vec![false; anim_root._frames.len()];
    for i in 0..anim_root._frames.len() {
        assert!( anim_root._frames[i]._index < anim_root._numframes );
        assert!( unique_frames[ anim_root._frames[i]._index as usize ] == false );
        unique_frames[ anim_root._frames[i]._index as usize ] = true;
    }
    for i in unique_frames {
        assert!( i == true );
    }
    //check hierarchy parent indices
    let mut parent_min = 99999999;
    let mut parent_max = -99999999;
    let mut count_root = 0;
    for i in 0..anim_root._hierarchy.len() {
        if  anim_root._hierarchy[i]._parent < parent_min {
            parent_min = anim_root._hierarchy[i]._parent;
            if parent_min == -1 {
                count_root += 1;
            }
        }
        if  anim_root._hierarchy[i]._parent > parent_max {
            parent_max = anim_root._hierarchy[i]._parent;
        }
    }
    assert!( count_root == 1 );
    assert!( parent_min == -1 );
    assert!( parent_max < anim_root._hierarchy.len() as i64 ); 
    Ok( anim_root )
}

#[allow(unused)]
pub fn process_hierarchy( file_content: &str, idx: usize, hm: & HashMap< &str, Token >, numjoints: u64 ) -> Result< ( usize, Vec< JointHierarchy > ), & 'static str > {
    let mut vec_h : Vec< JointHierarchy > = vec![];
    let mut idx_current = idx;
    match md5common::expect_bracel( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    for n in 0..numjoints {
        let mut h = JointHierarchy {
            _name: String::from(""),
            _parent: -1i64,
            _flags: 0u64,
            _start_index: 0u64,
        };
        let ( tok, kw_tok, idx_s, idx_e, idx_next ) = md5common::tokenize( &file_content[0..], idx_current, & hm );
        match tok {
            md5common::Token::Str => {
                h._name = String::from_str( &file_content[idx_s..idx_e] ).expect("md5anim parse hierarchy name failed");
                idx_current = idx_next;
            },
            _ => panic!("unexpected token. string not found.")
        }
        let mut arr: [i64;3] = [0i64;3];
        match md5common::expect_int_array( &file_content[..], idx_current, 3usize, & mut arr ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        h._parent = arr[0];
        h._flags = arr[1] as u64;
        h._start_index = arr[2] as u64;
        vec_h.push(h);
    }    
    match md5common::expect_bracer( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, vec_h ) )
}
#[allow(unused)]
pub fn process_bounds( file_content: &str, idx: usize, hm: & HashMap< &str, Token >, numframes: u64 ) -> Result< ( usize, Vec< Bound > ), & 'static str > {
    let mut vec_b : Vec< Bound > = vec![];
    let mut idx_current = idx;
    match md5common::expect_bracel( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    for n in 0..numframes {
        let mut b = Bound {
            _min: [0f32;3],
            _max: [0f32;3],
        };
        match md5common::expect_parenl( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_float_array( &file_content[..], idx_current, 3usize, & mut b._min ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenr( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenl( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_float_array( &file_content[..], idx_current, 3usize, & mut b._max ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenr( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        vec_b.push( b );
    }    
    match md5common::expect_bracer( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, vec_b ) )
}
#[allow(unused)]
pub fn process_baseframe( file_content: &str, idx: usize, hm: & HashMap< &str, Token >, numframes: u64 ) -> Result< ( usize, Vec< FrameJoint > ), & 'static str > {
    let mut vec_b : Vec< FrameJoint > = vec![];
    let mut idx_current = idx;
    match md5common::expect_bracel( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    for n in 0..numframes {
        let mut b = FrameJoint {
            _index: n,
            _pos: [0f32;3],
            _orient: [0f32;3],
        };
        match md5common::expect_parenl( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_float_array( &file_content[..], idx_current, 3usize, & mut b._pos ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenr( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenl( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_float_array( &file_content[..], idx_current, 3usize, & mut b._orient ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        match md5common::expect_parenr( &file_content[..], idx_current ) {
            Ok(v) => idx_current = v,
            Err(e) => assert!(false, e )
        }
        vec_b.push( b );
    }    
    match md5common::expect_bracer( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, vec_b ) )
}
#[allow(unused)]
pub fn process_frame( file_content: &str, idx: usize, hm: & HashMap< &str, Token >, num_animated_components: u64 ) -> Result< ( usize, Frame ), & 'static str > {
    let mut idx_current = idx;
    let mut f = Frame {
        _index: 0u64,
        _data: vec![0f32; num_animated_components as usize ],
    };
    let mut arr = [0i64;1];
    match md5common::expect_int_array( &file_content[..], idx_current, 1usize, & mut arr ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    f._index = arr[0] as u64;
    match md5common::expect_bracel( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_float_array( &file_content[..], idx_current, num_animated_components as usize, & mut f._data[..] ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    match md5common::expect_bracer( &file_content[..], idx_current ) {
        Ok(v) => idx_current = v,
        Err(e) => assert!(false, e )
    }
    Ok( ( idx_current, f ) )
}
