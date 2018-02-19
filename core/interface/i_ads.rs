extern crate mazth;

use self::mazth::mat;

pub trait AdsCoeff {
    fn get_ads_coeff_spec( & self ) -> mat::Mat3x1< f32 >;
    fn get_ads_coeff_diff( & self ) -> mat::Mat3x1< f32 >;
    fn get_ads_coeff_amb( & self ) -> mat::Mat3x1< f32 >;
}

pub trait AdsVal {
    fn get_ads_val_spec( & self ) -> mat::Mat3x1< f32 >;
    fn get_ads_val_diff( & self ) -> mat::Mat3x1< f32 >;
    fn get_ads_val_amb( & self ) -> mat::Mat3x1< f32 >;
}
