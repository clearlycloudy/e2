use interface::i_interpolate::IInterpolate;
use interface::i_step::Step;

pub fn step_delta< T, V >( s: & mut Step, step_delta: f64, interp: & mut T ) -> V where T : IInterpolate< V >, V: Clone
{
    if s._current_val >= s._range_val.1 {
        return interp.interp_current()
    }
    //clamp values to constraints
    let a = s._current_val + step_delta;
    let b = if a < s._range_val.0 {
        s._range_val.0
    }else if a > s._range_val.1 {
        s._range_val.1
    }else {
        a
    };
    let c = b - s._range_val.0;
    let total_steps = interp.num_steps();
    let fraction = c / ( s._range_val.1 - s._range_val.0 );
    let calc_steps = ( fraction * total_steps as f64 ) as u64;
    let mut ret = interp.interp_current();
    s._current_val = if s._current_val + c > s._range_val.1 {
        s._range_val.1
    } else {
        s._current_val + c
    };

    for _ in 0..calc_steps as usize {
        match interp.next() {
            None => { break; },
            Some( o ) => { ret = o; },
        }
    }
    ret
}
