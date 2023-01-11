use yewdux::prelude::Store;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct Global {
    pub value: i64
}