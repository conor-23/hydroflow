use super::PushSurfaceReversed;

use std::marker::PhantomData;

use crate::builder::build::push_handoff::HandoffPushBuild;
use crate::scheduled::handoff::{CanReceive, Handoff};
use crate::scheduled::port::SendPort;
use crate::{tl, tt};

pub struct HandoffPushSurfaceReversed<Hof, In>
where
    Hof: Handoff + CanReceive<In>,
{
    port: SendPort<Hof>,
    _phantom: PhantomData<fn(In)>,
}

impl<Hof, In> HandoffPushSurfaceReversed<Hof, In>
where
    Hof: Handoff + CanReceive<In>,
{
    pub fn new(port: SendPort<Hof>) -> Self {
        Self {
            port,
            _phantom: PhantomData,
        }
    }
}

impl<Hof, In> PushSurfaceReversed for HandoffPushSurfaceReversed<Hof, In>
where
    Hof: Handoff + CanReceive<In>,
{
    type ItemIn = In;

    type OutputHandoffs = tt!(SendPort<Hof>);
    type Build = HandoffPushBuild<Hof, In>;

    fn into_parts(self) -> (Self::OutputHandoffs, Self::Build) {
        (tl!(self.port), HandoffPushBuild::new())
    }
}
