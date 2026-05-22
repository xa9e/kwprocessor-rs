use crate::constants::ROUTE_LENGTH_MAX;

#[derive(Clone)]
pub(crate) struct Route {
    pub(crate) repeat: [u8; ROUTE_LENGTH_MAX],
    pub(crate) changes: usize,
}

impl Route {
    pub(crate) fn new() -> Self {
        Self {
            repeat: [0; ROUTE_LENGTH_MAX],
            changes: 0,
        }
    }
}
