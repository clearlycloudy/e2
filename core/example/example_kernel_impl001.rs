#![feature(use_extern_macros)]

extern crate e2rcore;
extern crate pretty_env_logger;

use std::env;

use self::e2rcore::interface::i_kernel::IKernel;

use self::e2rcore::implement::kernel::kernel_impl_001::Kernel;

pub struct DummyGameImpl {}

impl Default for DummyGameImpl {
    fn default() -> DummyGameImpl {
        DummyGameImpl {}
    }
}

#[allow(unused)]
fn main() {

    env::set_var("LOG_SETTING", "info" );
    
    pretty_env_logger::init_custom_env( "LOG_SETTING" );
    
    let mut k : Kernel<DummyGameImpl> = Kernel::new().unwrap();

    k.run();

}
