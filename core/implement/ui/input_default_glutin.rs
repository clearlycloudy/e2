extern crate glutin;

use interface::i_ui::{ IUi, InputFiltered, KeyCode, Coord, State };

pub struct XformInput {
    _moving_avg: u32,
}

impl Default for XformInput {

    fn default() -> Self {
        XformInput {
            _moving_avg: 1, //todo
        }
    }

}

impl IUi for XformInput {
    
    type EventInput = glutin::Event;
    type EventInputFiltered = InputFiltered;

    fn new() -> Self {
        Default::default()
    }
    
    fn process_input_events( & mut self, e: & [ Self::EventInput ], win_offset: (i32,i32), win_size: (u32,u32) ) -> Vec< InputFiltered > {
        let filtered = e.iter()
            .map( |x| InputFiltered::from( ( x, win_offset, win_size ) ) )
            .filter( |x| if let &InputFiltered::Ignored = x { false } else { true } )
            .inspect( |x| { trace!("{:?}", x); () } )
            .collect();
        filtered
    }
}


///convert glutin::Event into i_ui::InputFiltered
impl< 'a > From< (& 'a glutin::Event, (i32,i32), (u32,u32) )> for InputFiltered {
    fn from( input: (& 'a glutin::Event, (i32,i32), (u32,u32) ) ) -> Self {
        let e = input.0;
        let win_offset = input.1;
        let win_size = input.2;
        match e {
            &glutin::Event::WindowEvent{ ref event, .. } => match event {
                &glutin::WindowEvent::CloseRequested => {
                    InputFiltered::Button { key: KeyCode::Close, state: State::Press }
                },
                &glutin::WindowEvent::Resized( logical_size ) => { //todo
                    InputFiltered::Ignored
                },
                &glutin::WindowEvent::ReceivedCharacter(x) => {
                    match x {
                        'a' | 'A' => InputFiltered::Button { key: KeyCode::A, state: State::Press },
                        'b' | 'B' => InputFiltered::Button { key: KeyCode::B, state: State::Press },
                        'c' | 'C' => InputFiltered::Button { key: KeyCode::C, state: State::Press },
                        'd' | 'D' => InputFiltered::Button { key: KeyCode::D, state: State::Press },
                        'e' | 'E' => InputFiltered::Button { key: KeyCode::E, state: State::Press },
                        'f' | 'F' => InputFiltered::Button { key: KeyCode::F, state: State::Press },
                        'g' | 'G' => InputFiltered::Button { key: KeyCode::G, state: State::Press },
                        'h' | 'H' => InputFiltered::Button { key: KeyCode::H, state: State::Press },
                        'i' | 'I' => InputFiltered::Button { key: KeyCode::I, state: State::Press },
                        'j' | 'J' => InputFiltered::Button { key: KeyCode::J, state: State::Press },
                        'k' | 'K' => InputFiltered::Button { key: KeyCode::K, state: State::Press },
                        'l' | 'L' => InputFiltered::Button { key: KeyCode::L, state: State::Press },
                        'm' | 'M' => InputFiltered::Button { key: KeyCode::M, state: State::Press },
                        'n' | 'N' => InputFiltered::Button { key: KeyCode::N, state: State::Press },
                        'o' | 'O' => InputFiltered::Button { key: KeyCode::O, state: State::Press },
                        'p' | 'P' => InputFiltered::Button { key: KeyCode::P, state: State::Press },
                        'q' | 'Q' => InputFiltered::Button { key: KeyCode::Q, state: State::Press },
                        'r' | 'R' => InputFiltered::Button { key: KeyCode::R, state: State::Press },
                        's' | 'S' => InputFiltered::Button { key: KeyCode::S, state: State::Press },
                        't' | 'T' => InputFiltered::Button { key: KeyCode::T, state: State::Press },
                        'u' | 'U' => InputFiltered::Button { key: KeyCode::U, state: State::Press },
                        'v' | 'V' => InputFiltered::Button { key: KeyCode::V, state: State::Press },
                        'w' | 'W' => InputFiltered::Button { key: KeyCode::W, state: State::Press },
                        'x' | 'X' => InputFiltered::Button { key: KeyCode::X, state: State::Press },
                        'y' | 'Y' => InputFiltered::Button { key: KeyCode::Y, state: State::Press },
                        'z' | 'Z' => InputFiltered::Button { key: KeyCode::Z, state: State::Press },
                        '0' => InputFiltered::Button { key: KeyCode::Num0, state: State::Press },
                        '1' => InputFiltered::Button { key: KeyCode::Num1, state: State::Press },
                        '2' => InputFiltered::Button { key: KeyCode::Num2, state: State::Press },
                        '3' => InputFiltered::Button { key: KeyCode::Num3, state: State::Press },
                        '4' => InputFiltered::Button { key: KeyCode::Num4, state: State::Press },
                        '5' => InputFiltered::Button { key: KeyCode::Num5, state: State::Press },
                        '6' => InputFiltered::Button { key: KeyCode::Num6, state: State::Press },
                        '7' => InputFiltered::Button { key: KeyCode::Num7, state: State::Press },
                        '8' => InputFiltered::Button { key: KeyCode::Num8, state: State::Press },
                        '9' => InputFiltered::Button { key: KeyCode::Num9, state: State::Press },
                        ' ' => InputFiltered::Button { key: KeyCode::Space, state: State::Press },
                        _ => {
                            trace!("key events ignored: {:?}", e );
                            InputFiltered::Ignored
                        },
                    }
                },
                // issues: conversion from measured analog reading to physical or logical screen unit
                &glutin::WindowEvent::AxisMotion { ref axis, ref value, .. } => {
                    // info!("mouse input: {:?}, {:?}", axis, value );
                    let ( coord, val ) = match axis {
                        &0 => {
                            // println!("window mouse event x: {}, win offset: {:?}", *value as f32, win_offset );
                            ( Coord::X, *value as f32 - win_offset.0 as f32 )
                        },
                        &1 => {
                            ( Coord::Y, *value as f32 - win_offset.1 as f32 )
                        },
                        _ => {
                            ( Coord::Z, *value as f32 )
                        },
                    };
                    InputFiltered::MouseCoord( coord, val )
                },
                // &glutin::WindowEvent::CursorMoved { ref device_id, ref position/*logical pos*/, .. } => {
                //     info!("mouse input: {:?}", position );
                //     InputFiltered::MouseCoord2( position.x as f32 - win_offset.0 as f32,
                //                                 position.y as f32 - win_offset.1 as f32 )
                // },
                &glutin::WindowEvent::KeyboardInput { ref input, .. } => {
                    match input {
                        &glutin::KeyboardInput {
                            ref state,
                            ref virtual_keycode,
                            ..
                        } => {
                            // trace!("probing: {:?}", input );
                            let k = match virtual_keycode {
                                &Some(glutin::VirtualKeyCode::Left) => Some( KeyCode::Left ),
                                &Some(glutin::VirtualKeyCode::Right) => Some( KeyCode::Right ),
                                &Some(glutin::VirtualKeyCode::Up) => Some( KeyCode::Up ),
                                &Some(glutin::VirtualKeyCode::Down) => Some( KeyCode::Down ),
                                _ => None,
                            };
                            let s = match state {
                                &glutin::ElementState::Pressed => State::Press,
                                _ => State::Release,
                            };
                            if let Some( x ) = k {
                                InputFiltered::Button { key: x, state: s }
                            } else {
                                InputFiltered::Ignored
                            }
                        },
                    }
                },
                &glutin::WindowEvent::MouseInput { ref state, ref button, .. } => {
                    let k = match button {
                        &glutin::MouseButton::Left  => {
                            Some( KeyCode::MouseL )
                        },
                        &glutin::MouseButton::Right  => {
                            Some( KeyCode::MouseR )
                        },
                        _ => None
                    };
                    let s = match state {
                        &glutin::ElementState::Pressed => State::Press,
                        _ => State::Release,
                    };
                    if let Some( x ) = k {
                        InputFiltered::Button { key: x, state: s }
                    } else {
                        InputFiltered::Ignored
                    }
                },
                _ => {
                    trace!("events input ignored: {:?}", e );
                    InputFiltered::Ignored
                },
            },
            _ => InputFiltered::Ignored
        }
    }
}
