use std::fmt::Debug;
use std::hash::Hash;

use serde::Serialize;

use crate::Actor;

pub trait BRBDataType: Debug {
    type Op: Debug + Clone + Hash + Eq + Serialize;
    type ValidationError: Debug + 'static;

    /// initialize a new replica of this datatype
    fn new(actor: Actor) -> Self;

    /// Protection against Byzantines
    fn validate(&self, source: &Actor, op: &Self::Op) -> Result<(), Self::ValidationError>;

    /// Executed once an op has been validated
    fn apply(&mut self, op: Self::Op);
}
