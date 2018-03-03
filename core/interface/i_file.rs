pub trait IParseStr {
    type output;
    fn parse( file_content: &str ) -> Result< Self::output, & 'static str >;
}


