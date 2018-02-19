extern crate mazth;

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::cmp::Ordering;

use std::fmt::Debug;
#[allow(unused_imports)]
use self::mazth::mat::Mat4x1;
use self::mazth::i_comparable::IComparableError;

#[allow(unused_imports)]
use interface::i_interpolate::IInterpolate;

use implement::math::piecewise::Piecewise;

#[cfg(test)]
pub fn test_interp_forward< T >( interp: & mut IInterpolate< T, Item=T >, _start_val: & T, end_val: & T ) -> () 
    where T: Debug + IComparableError< f64 > + Clone
{
    
    #[allow(unused_variables)]
    while !interp.interp_is_end() {
        let p = interp.interp_delta( 1 ).expect("forward interpolation result unexpected");
        println!("interp: {:?}", p );
    }
    //iteration end value check
    {
        let p = interp.interp_current();
        println!("interp: {:?}", p );
        assert!( p.is_equal( &end_val, 0.00001 ).expect( "iteration end value unexpected" ) );
    }
    //iteration stop condition check
    #[allow(unused_variables)]
    for x in 0..3 {
        let p = interp.interp_delta( 1 ).expect("forward interpolation result unexpected");
        println!("{:?}", p );
        assert!( p.is_equal( &end_val, 0.00001 ).expect( "iteration stop behaviour unexpected" ) );
    }
}

#[cfg(test)]
pub fn test_interp_forward_back< T >( interp: & mut IInterpolate< T, Item=T >, start_val: & T, end_val: & T ) -> () 
    where T: Debug + IComparableError< f64 > + Clone
{
    
    #[allow(unused_variables)]
    while !interp.interp_is_end() {
        let p = interp.interp_delta( 1 ).expect("forward interpolation result unexpected");
        println!("interp: {:?}", p );
    }
    //iteration end value check
    {
        let p = interp.interp_current();
        println!("interp: {:?}", p );
        assert!( p.is_equal( &end_val, 0.00001 ).expect( "iteration end value unexpected" ) );
    }
    //iteration stop condition check
    #[allow(unused_variables)]
    for x in 0..3 {
        let p = interp.interp_delta( 1 ).expect("forward interpolation result unexpected");
        println!("{:?}", p );
        assert!( p.is_equal( &end_val, 0.00001 ).expect( "iteration stop behaviour unexpected" ) );
    }
    //backward iteration
    while !interp.interp_is_start() {
        let p = interp.interp_delta( -1 ).expect("backward interpolation result unexpected");
        println!("interp: {:?}", p );
    }
    //iteration start value check
    {
        let p = interp.interp_current();
        assert!( p.is_equal( &start_val, 0.00001 ).expect( "iteration start value unexpected" ) );
    }
    //iteration stop condition check for back iteration
    #[allow(unused_variables)]
    for x in 0..3 {
        let p = interp.interp_delta( -1 ).expect("backward interpolation result unexpected");
        println!("{:?}", p );
        assert!( p.is_equal( &start_val, 0.00001 ).expect( "iteration stop behaviour unexpected" ) );
    }
}

#[cfg(test)]
pub fn test_iterator< T >( interp: & mut IInterpolate< T, Item=T >, start_val: & T, end_val: & T ) -> ()
    where T: Debug + IComparableError< f64 > + Clone
{
    for x in interp.enumerate() {
        println!("{}: {:?}", x.0, x.1 );
        match x {
            (0,val) => {
                assert!( val.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") );
            },
            (9,val) => {
                assert!( val.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") );
            },
            _ => (),
        }
    }
}

#[cfg(test)]
pub fn test_reverse_iterator< T >( interp: & mut IInterpolate< T, Item=T >, start_val: & T, end_val: & T ) -> ()
    where T: Debug + IComparableError< f64 > + Clone
{
    let mut index = 0;
    for x in interp.next() {
        println!("{}: {:?}", index, x );
        if index == 0 {
            assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") );
        }
        if index == 9 {
            assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") );
        }
        index += 1;
    }
    for x in interp.next_back() {
        println!("{}: {:?}", index, x );
        if index == 9 {
            assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") );
        }
        if index == 0 {
            assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") );
        }
        index -= 1;
    }
}


#[cfg(test)]
pub fn test_piecewise< I, T >( interps: & mut Piecewise< I, T >, start_val: & T, end_val: & T, piece_count: usize, total_steps: u64 ) -> ()
    where T: Debug + IComparableError< f64 > + Clone,
          I: IInterpolate< T, Item=T >
{
    assert!(interps._pieces.len() == piece_count );
    assert!(interps.num_steps() == total_steps );

    let mut index = 0;
    assert!( interps.interp_is_start() );
    interps.interp_delta(1);
    while !interps.interp_is_end() {
        let x = interps.interp_current();
        match index {
            0 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            9 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            10 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            29 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            30 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            59 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            60 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            99 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            _ => ()
        }
        index += 1;
        interps.interp_delta(1);
    }
    assert!( index == total_steps );
}


#[cfg(test)]
pub fn test_piecewise_iterator< I, T >( interps: & mut Piecewise< I, T >, start_val: & T, end_val: & T, piece_count: usize, total_steps: u64 ) -> ()
    where T: Debug + IComparableError< f64 > + Clone,
          I: IInterpolate< T, Item=T >
{
    assert!( interps._pieces.len() == piece_count );
    assert!( interps._total_steps == total_steps );
    let mut max_index = 0usize;
    for (i, x) in interps.enumerate() {
        // println!("piecewise point: {}: {:?}", i, x );
        max_index = i;
        match i {
            0 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            9 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            10 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            29 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            30 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            59 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            60 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
            99 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
            _ => ()
        }
    }
    assert!( max_index == total_steps as usize - 1 );
}

#[cfg(test)]
pub fn test_piecewise_reset< I, T >( interps: & mut Piecewise< I, T >, start_val: & T, end_val: & T, piece_count: usize, total_steps: u64 ) -> ()
    where T: Debug + IComparableError< f64 > + Clone,
          I: IInterpolate< T, Item=T >
{
    assert!(interps._pieces.len() == piece_count );
    assert!(interps.num_steps() == total_steps );
    {
        let mut i = 0;
        while let Some(x) = interps.next() {
            match i {
                0 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                9 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                10 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                29 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                30 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                59 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                60 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                99 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                _ => ()
            }
            i += 1;
        }
        assert!( i == total_steps );
    }
    interps.reset();
    assert!(interps._pieces.len() == piece_count );
    assert!(interps._total_steps == total_steps );
    {
        let mut i = 0;
        while let Some(x) = interps.next() {
            match i {
                0 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                9 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                10 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                29 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                30 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                59 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                60 => assert!( x.is_equal( &start_val, 0.00001f64 ).expect("is_equal invalid") ),
                99 => assert!( x.is_equal( &end_val, 0.00001f64 ).expect("is_equal invalid") ),
                _ => ()
            }
            i += 1;
        }
        assert!( i == total_steps );
    }
}
