pub trait IRenderer {

    type EventRender;

    fn init() -> Result< Self, & 'static str > where Self: Sized;

    fn process_render_events( & mut self, Vec< Self::EventRender > ) -> Result< (), & 'static str >;
}

