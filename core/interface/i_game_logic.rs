///interface for game logic as well as some default implementations

use std::fmt::Debug;

use interface::i_scheduler::IScheduler;

pub trait IGameLogic
{
    type EventInput : Debug;
    type EventRender;
    type GameState : Default + Clone + From< (Self::GameState, Self::GameStateChangeApply) >;
    type GameStateChangePending : Default + Clone;
    type GameStateChangeApply : Default + Clone + From< Self::ComputeUnit >;
    type ComputeUnit;
    type ComputeSchedule : IScheduler< Item = Self::ComputeUnit > + Iterator<Item = Vec<Self::ComputeUnit> >;
    /// transform a high level renderobj representation into render commands / elements
    type RenderObj : Into< Vec< Self::EventRender > >;
    
    fn new() -> Self;
    
    fn run_init_hook( & mut self ) -> Result< (), & 'static str >;

    fn get_states( & mut self ) -> & Self::GameState;

    fn get_states_mut( & mut self ) -> & mut Self::GameState;
    
    ///computes changed game state given user inputs and current game state
    fn transition_states( & mut self, inputs: & [ Self::EventInput ] ) -> Self::GameStateChangePending;

    ///compute constraints per cycle
    fn continue_compute( & mut self ) -> bool;

    fn set_continue_compute( & mut self, bool );

    ///get what to compute based on changed game state
    fn get_computations( & mut self, changed_game_state: & Self::GameStateChangePending ) -> Vec< Self::ComputeUnit >;

    ///schedule computations
    fn schedule_computes( & mut self, computes: Vec< Self::ComputeUnit > ) -> Vec< Self::ComputeSchedule >;

    ///get all renderable objects from current game state
    fn get_renderable_components( & mut self ) -> Vec< Self::RenderObj >;

    ///do optimization on renderable objects
    fn filter_renderables( & mut self, r: Vec< Self::RenderObj > ) -> Vec< Self::RenderObj >;

    fn should_exit( & mut self ) -> bool;
    
    ///default implementation
    fn process_input_events( & mut self, e: & [ Self::EventInput ] ) -> ( Vec< Self::EventRender >, bool ) {

        //process input events
        if e.len() > 0 {
            trace!( "filtered_input: {:?}", e );
        }
        
        //get changed states pending for computations
        let changed_states_pending = self.transition_states( e );

        let mut count_compute_cycle = 0;
        
        //perform computations
        while self.continue_compute() { //compute flag accessed in game state by game logic

            count_compute_cycle += 1;
            
            //todo: transform changed game states to determine what to compute/update
            let computes = self.get_computations( & changed_states_pending );

            //compute a schedule, returning a vector of iterators of compute unit,
            //with no dependencies across different series of iterator in the vector
            let scheduler = Self::ComputeSchedule::new( computes.as_slice() );

            for i in scheduler { //consume and fork work possible into parallel threads
                i.into_iter()
                    //execute computation unit and map it back to a state change
                    .map( |compute_unit| Self::GameStateChangeApply::from(compute_unit) )
                    //apply the change back to the game state
                    .for_each( |changes| {
                        //apply result of compute unit changes back into game state
                        //todo: optimize this
                        let game_state = self.get_states().clone();
                        let game_state_new = Self::GameState::from( ( game_state, changes ) );
                        *self.get_states_mut() = game_state_new;
                    });
            }

            trace!( "compute cycle(s): {}", count_compute_cycle );
        }

        //transform renderable objects to render events
        let render_objects = self.get_renderable_components();

        //do some spatial optimization here
        let render_objects_filtered = self.filter_renderables( render_objects );

        //do further render commands packaging here
        let render_events = render_objects_filtered
            .into_iter()
            //transformation of high level game object into render elements / commands
            .flat_map( |x| Self::RenderObj::into( x ) )
            .collect();
        
        let sig_exit = self.should_exit();
        
        ( render_events, sig_exit )
    }
}
