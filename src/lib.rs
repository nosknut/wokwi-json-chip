/*
    Note, This should be considered almost a separate crate however i'm prototyping
    For actual use please convert main1 to some form of example and keep redefine exports for the rest
*/

pub mod json_parser;
pub mod uart;

mod main1;
pub use main1::*;
