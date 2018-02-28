extern crate rand;

use self::rand::distributions::{IndependentSample, Range};

#[derive(Clone, Debug)]
pub struct LoadDistrUniform < T > {
    _buf: Vec< T >,
}

impl < T > Default for LoadDistrUniform < T > {
    fn default() -> Self {
        Self {
            _buf: vec![],
        }
    }
}

impl < T > LoadDistrUniform < T > {

    pub fn load( & mut self, t: T ) -> Result< (), & 'static str > {
        self._buf.push( t );
        Ok( () )
    }

    pub fn apply< F >( & mut self, mut f: F ) -> Result< (), & 'static str >
        where F: FnMut( & T ) -> bool
    {
        if self._buf.len() > 0 {
            let between = Range::new( 0, self._buf.len() );
            let mut rng = rand::thread_rng();
            let i = between.ind_sample( & mut rng );
            f( & self._buf[ i ] );
        }

        self._buf.clear();
        
        Ok( () )
    }
}

