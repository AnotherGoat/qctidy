use getset::{CopyGetters, Getters};
use inew::New;

use crate::{EdgeType, NodeView};

#[derive(Debug, Clone, Copy, PartialEq, Getters, CopyGetters, New)]
#[new(pub, const)]
#[must_use]
pub struct EdgeView {
    #[get_copy = "pub"]
    r#type: EdgeType,
    #[get = "pub"]
    start: NodeView,
    #[get = "pub"]
    end: NodeView,
}
