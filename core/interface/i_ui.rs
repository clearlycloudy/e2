#[derive(Clone, Debug, Copy)]
pub enum KeyCode {
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Capital,
    Colon,
    Comma,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    LAlt,
    LBracket,
    LControl,
    LMenu,
    LShift,
    LWin,
    Period,
    RAlt,
    RBracket,
    RControl,
    RShift,
    Semicolon,
    Slash,
    Tab,
    Close,
    MouseL,
    MouseR,
}

#[derive(Clone, Debug, Copy)]
pub enum State {
    Press,
    Release,
}

#[derive(Clone, Debug, Copy)]
pub enum Coord {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Copy)]
pub enum InputFiltered {
    Button{ key: KeyCode, state: State },
    MouseCoord( Coord, f32 ),
    Ignored,
}

pub trait IUi{

    type EventInput;
    type EventInputFiltered;

    fn new() -> Self where Self: Sized;

    fn process_input_events( & mut self, e: & [ Self::EventInput ] ) -> Vec< Self::EventInputFiltered >;
}
