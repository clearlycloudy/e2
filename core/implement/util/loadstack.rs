#[derive(Clone, Debug)]
pub struct LoadStack < T > {
    _buf: Vec< T >,
}

impl < T > Default for LoadStack < T > {
    fn default() -> Self {
        Self {
            _buf: vec![],
        }
    }
}

impl < T > LoadStack < T > {

    pub fn load( & mut self, t: T ) -> Result< (), & 'static str > {
        self._buf.push( t );
        Ok( () )
    }

    pub fn apply< F >( & mut self, mut f: F ) -> Result< (), & 'static str >
        where F: FnMut( & T ) -> bool
    {
        let mut index = 0;

        {
            for (k,v) in self._buf.iter().rev().enumerate() {
                index = k;
                if !f( v ) { //stop when the function first returns false
                    break;
                }
            }
        }
        let temp : Vec< T > = self._buf.drain(..index).collect(); //discard traversed elements
        self._buf = temp;

        Ok( () )
    }
}

