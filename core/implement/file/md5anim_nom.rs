#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate pretty_env_logger;
extern crate nom;

use std::str;
use std::str::FromStr;

use self::nom::digit;

use interface::i_md5::anim::*;
use interface::i_file::IParseStr;

named!( peek_version< &str, &str >,
        peek!(
            ws!( tag!("MD5Version") )
        )
);

named!( peek_commandline< &str, &str >,
        peek!(
            ws!( tag!("commandline") )
        )
);

named!( md5_commandline< &str, String >,
        do_parse!(
            ws!( tag!("commandline") ) >>
            cmd: delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ) >>
            ( cmd.to_string() )
        )
);

named!( peek_numFrames< &str, &str >,
        peek!(
            ws!( tag!("numFrames") )
        )
);

named!( peek_numJoints< &str, &str >,
        peek!(
            ws!( tag!("numJoints") )
        )
);

named!( peek_frameRate< &str, &str >,
        peek!(
            ws!( tag!("frameRate") )
        )
);

named!( peek_numAnimatedComponents< &str, &str >,
        peek!(
            ws!( tag!("numAnimatedComponents") )
        )
);

named!( single_u64< &str, u64 >,
        do_parse!(
            num: map_res!( ws!(digit), FromStr::from_str ) >>
            ( num )
        )
);

named!( peek_hierarchy< &str, &str >,
        peek!(
            ws!( tag!("hierarchy") )
        )
);

named!( md5anim_hierarchy_opening< &str, &str >,
        do_parse!(
            ws!( tag!("hierarchy") ) >>
            a: ws!( tag!("{") ) >>
            ( a )    
        )
);

named!( md5anim_hierarchy_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()    
        )
);

named!( consume_comments_start< &str, () >,
        do_parse!(
            take_until_either!( "\n\r" ) >>
            tag!("//") >>
            ()
        )
);

named!( consume_comments< &str, &str >,
        take_until_either!( "\n\r" )
);

named!( consume_newline< &str, &str >,
        not!( take_until_and_consume!( "\n\r" ) )
);

named!( signed_num< &str, &str >,
        recognize!(
            do_parse!(
                sgn: alt!( tag!("+") | tag!("-") | tag!("") ) >>
                d: digit >> ()
            )
        )
);

named!( md5anim_hierarchy< &str, JointHierarchy >,
        do_parse!(
            name: ws!(delimited!(
                tag!("\""),
                take_until!("\""),
                tag!("\"")
            ) ) >>
            parent: map_res!(ws!(signed_num), <i64 as FromStr>::from_str ) >>
            flags: map_res!(ws!(digit), <u64 as FromStr>::from_str ) >>
            start_index: map_res!(ws!(digit), <u64 as FromStr>::from_str ) >>   
            (
                JointHierarchy {
                    _name: name.to_string(),
                    _parent: parent,
                    _flags: flags,
                    _start_index: start_index,
                }
            )
        )
);

named!( peek_md5anim_bounds< &str, &str >,
        peek!(
            ws!( tag!("bounds") )
        )
);

named!( md5anim_bounds_opening< &str, &str >,
        do_parse!(
            ws!( tag!("bounds") ) >>
            a: ws!( tag!("{") ) >>
            ( a )    
        )
);

named!( md5anim_bounds_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()    
        )
);

named!( md5anim_bound< &str, Bound >,
        do_parse!(
            ws!(tag!("(")) >>
            min0: ws!(nom::float_s) >>
            min1: ws!(nom::float_s) >>
            min2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            ws!(tag!("(")) >>
            max0: ws!(nom::float_s) >>
            max1: ws!(nom::float_s) >>
            max2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            (
                Bound {
                        _min: [ min0, min1, min2 ],
                        _max: [ max0, max1, max2 ],
                }
            )
        )
);

named!( peek_md5anim_baseframe< &str, &str >,
        peek!(
            ws!( tag!("baseframe") )
        )
);

named!( md5anim_baseframe_opening< &str, &str >,
        do_parse!(
            ws!( tag!("baseframe") ) >>
            a: ws!( tag!("{") ) >>
            ( a )    
        )
);

named!( md5anim_baseframe_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()    
        )
);

named!( md5anim_baseframe< &str, FrameJoint >,
        do_parse!(
            ws!(tag!("(")) >>
            p0: ws!(nom::float_s) >>
            p1: ws!(nom::float_s) >>
            p2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            ws!(tag!("(")) >>
            o0: ws!(nom::float_s) >>
            o1: ws!(nom::float_s) >>
            o2: ws!(nom::float_s) >>
            ws!(tag!(")")) >>
            (
                FrameJoint {
                    _index: 0, //to be set by caller
                    _pos: [ p0, p1, p2 ],
                    _orient: [ o0, o1, o2 ],
                }
            )
        )
);

named!( single_word< &str, &str >,
        do_parse!(
            word: ws!(nom::alphanumeric) >>
            (
                word
            )
        )
);

named!( single_float< &str, f32 >,
        do_parse!(
            num: ws!(nom::float_s) >>
            (
                num
            )
        )
);

named!( peek_md5anim_frame< &str, &str >,
        peek!(
            ws!( tag!("frame") )
        )
);

named!( md5anim_frame_opening< &str, usize >,
        do_parse!(
            ws!( tag!("frame") ) >>
            frame_idx: map_res!(ws!(digit), <usize as FromStr>::from_str ) >>
            ws!( tag!("{") ) >>
            ( frame_idx )
        )
);

named!( md5anim_frame_closing< &str, () >,
        do_parse!(
            ws!( tag!("}") ) >>
            ()    
        )
);

named!( peek_comments< &str, &str >,
        peek!(
            ws!( tag!("//") )
        )
);


fn peek_and_consume_comments( mut input: & str ) -> Option< & str > {
    match peek_comments( input ) {
        nom::IResult::Done( i, o ) => {
            match consume_comments_start( i ) {
                nom::IResult::Done( j, o ) => {
                    input = j;
                },
                _ => {},
            }
            match consume_comments( input ) {
                nom::IResult::Done( j, o ) => {
                    input = j;
                },
                _ => {},
            }
            match consume_newline( input ) {
                nom::IResult::Done( k, o ) => {
                    input = k;
                },
                _ => {},
            }
            return Some( input )
        },
        _ => {},
    }
    None
}


fn parse_hierarchy( mut buf: & str, num_joints: Option<u64> ) -> Result< ( & str, Vec< JointHierarchy > ), & 'static str > {

    let mut hierarchies : Vec< JointHierarchy > = vec![];
    
    match md5anim_hierarchy_opening( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {
            return Err( "hierarchy opening token not found" )
        },
    }
    match num_joints {
        None => {
            return Err( "num joints not specified at point of hierarchy parsing" )
        },
        _ => {},
    }
    
    let n = num_joints.unwrap();
    let mut count = 0;
    while count < n {
        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                continue;
            },
            _ => {}
        }
        match md5anim_hierarchy( buf ) {
            nom::IResult::Done( i, o ) => {
                buf = i;
                hierarchies.push( o );
            },
            _ => {
                return Err("joint parse unsuccessful")
            },
        }

        count += 1;
    }                

    match peek_and_consume_comments( buf ) {
        Some(x) => {
            buf = x;
        },
        _ => {}
    }

    match md5anim_hierarchy_closing( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {},
    }

    match peek_and_consume_comments( buf ) {
        Some(x) => { buf = x; },
        _ => {},
    }

    Ok( ( buf, hierarchies ) )
}

fn parse_bounds( mut buf: & str, num_frames: Option< u64 > ) -> Result< ( & str, Vec< Bound > ), & 'static str > {

    let mut bounds : Vec< Bound > = vec![];

    match md5anim_bounds_opening( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {
            return Err( "hierarchy opening token not found" )
        },
    }
    match num_frames {
        None => {
            return Err( "num frames not specified at point of bounds parsing" )
        },
        _ => {},
    }
    
    let n = num_frames.unwrap();
    let mut count = 0;
    while count < n {
        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                continue;
            },
            _ => {}
        }
        match md5anim_bound( buf ) {
            nom::IResult::Done( i, o ) => {
                buf = i;
                bounds.push( o );
            },
            _ => {
                return Err("bounds parse unsuccessful")
            },
        }

        count += 1;
    }                

    match peek_and_consume_comments( buf ) {
        Some(x) => {
            buf = x;
        },
        _ => {}
    }

    match md5anim_bounds_closing( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {},
    }

    match peek_and_consume_comments( buf ) {
        Some(x) => { buf = x; },
        _ => {},
    }

    Ok( ( buf, bounds ) )
}

fn parse_baseframe( mut buf: & str, num_joints: Option< u64 > ) -> Result< ( & str, Vec< FrameJoint > ), & 'static str > {

    let mut baseframe : Vec< FrameJoint > = vec![];
    
    match md5anim_baseframe_opening( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {
            return Err( "baseframe opening token not found" )
        },
    }
    match num_joints {
        None => {
            return Err( "num joints not specified at point of baseframe parsing" )
        },
        _ => {},
    }
    
    let n = num_joints.unwrap();
    let mut count = 0;
    while count < n {
        match peek_and_consume_comments( buf ) {
            Some(x) => {
                buf = x;
                continue;
            },
            _ => {}
        }
        match md5anim_baseframe( buf ) {
            nom::IResult::Done( i, mut o ) => {
                buf = i;
                o._index = count as u64; //set index
                baseframe.push( o );
            },
            _ => {
                return Err("bounds parse unsuccessful")
            },
        }

        count += 1;
    }                

    match peek_and_consume_comments( buf ) {
        Some(x) => {
            buf = x;
        },
        _ => {}
    }

    match md5anim_baseframe_closing( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {},
    }

    match peek_and_consume_comments( buf ) {
        Some(x) => { buf = x; },
        _ => {},
    }

    Ok( ( buf, baseframe ) )
}

fn parse_frame( mut buf: & str, num_animated_components: Option< u64 > ) -> Result< ( & str, Frame ), & 'static str > {
    
    let frame_idx = match md5anim_frame_opening( buf ) { //returns the frame index number
        nom::IResult::Done( i, o ) => {
            buf = i;
            o
        },
        _ => {
            return Err( "frame opening token not found" )
        },
    };

    match num_animated_components {
        None => {
            return Err( "num animated components not specified at point of frame parsing" )
        },
        _ => {},
    }

    let m = num_animated_components.unwrap();

    let mut f = Frame {
        _index: frame_idx as u64,
        _data: vec![],
    };

    for _ in 0..m { //per animated component
        match single_float( buf ) {
            nom::IResult::Done( i, o ) => {
                buf = i;
                f._data.push( o );
            },
            _ => {
                return Err("frame data parse unsuccessful")
            },
        }
    }

    match md5anim_frame_closing( buf ) {
        nom::IResult::Done( i, o ) => {
            buf = i;
        },
        _ => {
            return Err("frame closing token not found")            
        },
    }

    match peek_and_consume_comments( buf ) {
        Some(x) => { buf = x; },
        _ => {},
    }

    Ok( ( buf, f ) )
}

fn consume_alphanumeric( buf: & str ) -> Result< & str, & 'static str > {
    match single_word( buf ) {
        nom::IResult::Done( i, o ) => {
            return Ok( i )
        },
        _ => {
            return Err( "consuming alphanumeric unsuccessful" )
        },
    }
}
pub struct Md5AnimParser {}

impl IParseStr for Md5AnimParser {

    type output = Md5AnimRoot;

    fn parse( file_content : &str ) -> Result< Md5AnimRoot, & 'static str > {
        let mut buf = file_content;

        let mut version = None;
        let mut cmdline = None;
        let mut num_frames = None;
        let mut num_joints = None;
        let mut frame_rate = None;
        let mut num_animated_components = None;
        let mut hierarchies = vec![];
        let mut bounds = vec![];
        let mut baseframe : Vec< FrameJoint > = vec![];
        let mut frames = vec![];

        loop {
            let mut progress = false;

            match peek_version( buf ) {
                nom::IResult::Done( _, _ ) => {
                    buf = consume_alphanumeric( buf ).unwrap();
                    match single_u64( buf ) {
                        nom::IResult::Done( i, o ) => {
                            version = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_commandline( buf ) {
                nom::IResult::Done( _, _ ) => {
                    match md5_commandline( buf ) {
                        nom::IResult::Done( i, o ) => {
                            cmdline = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_numFrames( buf ) {
                nom::IResult::Done( _, _ ) => {
                    buf = consume_alphanumeric( buf ).unwrap();
                    match single_u64( buf ) {
                        nom::IResult::Done( i, o ) => {
                            debug!( "num meshes: {:?}", o );
                            num_frames = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_numJoints( buf ) {
                nom::IResult::Done( _, _ ) => {
                    buf = consume_alphanumeric( buf ).unwrap();
                    match single_u64( buf ) {
                        nom::IResult::Done( i, o ) => {
                            debug!( "num joints: {:?}", o );
                            num_joints = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_frameRate( buf ) {
                nom::IResult::Done( _, _ ) => {
                    buf = consume_alphanumeric( buf ).unwrap();
                    match single_u64( buf ) {
                        nom::IResult::Done( i, o ) => {
                            debug!( "num frameRate: {:?}", o );
                            frame_rate = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_numAnimatedComponents( buf ) {
                nom::IResult::Done( _, _ ) => {
                    buf = consume_alphanumeric( buf ).unwrap();
                    match single_u64( buf ) {
                        nom::IResult::Done( i, o ) => {
                            debug!( "num numAnimatedComponents: {:?}", o );
                            num_animated_components = Some( o );
                            buf = i;
                            progress = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }

            match peek_hierarchy( buf ) {
                nom::IResult::Done( _, _ ) => {
                    let ( b, h ) = parse_hierarchy( buf, num_joints )?;
                    buf = b;
                    hierarchies = h;
                    progress = true;
                },
                _ => {},
            }
            
            match peek_md5anim_bounds( buf ) {
                nom::IResult::Done( _, _ ) => {
                    let ( b, bounds_inner ) = parse_bounds( buf, num_frames )?;
                    buf = b;
                    bounds = bounds_inner;
                    progress = true;
                },
                _ => {},
            } //end of bounds

            match peek_md5anim_baseframe( buf ) {
                nom::IResult::Done( _, _ ) => {
                    let( b, baseframe_inner ) = parse_baseframe( buf, num_joints )?;
                    buf = b;
                    baseframe = baseframe_inner;
                    progress = true;
                },
                _ => {},
            } //end of baseframe

            match peek_md5anim_frame( buf ) {
                nom::IResult::Done( _, _ ) => {
                    let ( b, f ) = parse_frame( buf, num_animated_components )?;
                    buf = b;
                    frames.push( f );
                    progress = true;
                },
                _ => {},
            } //end of frame x

            if !progress {
                break;
            }
        }//end of loop

        if let None = version {
            return Err( "version not found")
        }

        if let None = cmdline {
            return Err( "cmdline not found")
        }
        if let None = num_frames {
            return Err( "num frames not found")
        }
        if let None = num_joints {
            return Err( "num joints not found")
        }
        if let None = frame_rate {
            return Err( "frame rate not found")
        }
        if let None = num_animated_components {
            return Err( "num animated components not found")
        }

        Ok( Md5AnimRoot {
            _md5ver: version.unwrap(),
            _cmdline: cmdline.unwrap(),
            _numframes: num_frames.unwrap(),
            _numjoints: num_joints.unwrap(),
            _framerate: frame_rate.unwrap(),
            _num_animated_components: num_animated_components.unwrap(),
            _hierarchy: hierarchies,
            _bounds: bounds,
            _baseframe: baseframe,
            _frames: frames,
        } )
    }
}
