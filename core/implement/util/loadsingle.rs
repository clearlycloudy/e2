#[derive(Clone, Debug)]
pub struct LoadSingle < T > {
    _buf: Option< T >,
    ///if true, no-load if there exists data
    pub _is_blocking: bool,
}

impl < T > Default for LoadSingle < T > {
    fn default() -> Self {
        Self {
            _buf: None,
            _is_blocking: false,
        }
    }
}

impl < T > LoadSingle < T > {

    pub fn load( & mut self, t: T ) -> Result< (), & 'static str > {
        if self._is_blocking {
            if self._buf.is_none() {
                self._buf = Some( t );
            }
        } else {
            self._buf = Some( t );
        }
        
        Ok( () )
    }

    pub fn apply< F >( & mut self, mut f: F ) -> Result< (), & 'static str >
        where F: FnMut( & T ) -> bool
    {

        let consumed = match self._buf {
            None => { false }, //do nothing if no data
            Some(ref x) => { f( x ); true },
        };

        if consumed {
            self._buf = None;
        }

        Ok( () )
    }
}

