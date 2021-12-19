mod mouse_event;

use std::fmt::Debug;
use web_sys::{EventTarget, MouseEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEventWrapper(pub MouseEvent);
