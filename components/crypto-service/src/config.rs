#![allow(non_camel_case_types)]

use heapless::consts;

// TODO: this needs to be overridable.
// Should we use the "config crate that can have a replacement patched in" idea?

pub type MAX_MESSAGE_LENGTH = consts::U1024;
pub type MAX_OBJECT_LABEL_LENGTH = consts::U256;
pub type MAX_SERVICE_CLIENTS = consts::U5;
pub type MAX_SIGNATURE_LENGTH = consts::U72;


