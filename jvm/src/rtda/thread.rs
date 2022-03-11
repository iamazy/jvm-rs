use crate::rtda::{Frame, Stack};

struct Thread {
    pc: usize,
    stack: Option<Stack>,
}
