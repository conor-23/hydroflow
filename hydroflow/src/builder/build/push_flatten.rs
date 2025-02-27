use super::{PushBuild, PushBuildBase};

use std::marker::PhantomData;

use crate::compiled::flatten::Flatten;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::handoff_list::PortList;
use crate::scheduled::port::SEND;

pub struct FlattenPushBuild<Next, In>
where
    Next: PushBuild,
    In: IntoIterator<Item = Next::ItemIn>,
{
    next: Next,
    _phantom: PhantomData<fn(In)>,
}
impl<Next, In> FlattenPushBuild<Next, In>
where
    Next: PushBuild,
    In: IntoIterator<Item = Next::ItemIn>,
{
    pub fn new(next: Next) -> Self {
        Self {
            next,
            _phantom: PhantomData,
        }
    }
}

impl<Next, In> PushBuildBase for FlattenPushBuild<Next, In>
where
    Next: PushBuild,
    In: IntoIterator<Item = Next::ItemIn>,
{
    type ItemIn = In;
    type Build<'slf, 'hof> = Flatten<Next::Build<'slf, 'hof>, In>;
}

impl<Next, In> PushBuild for FlattenPushBuild<Next, In>
where
    Next: PushBuild,
    In: IntoIterator<Item = Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;

    fn build<'slf, 'hof>(
        &'slf mut self,
        context: &Context<'_>,
        handoffs: <Self::OutputHandoffs as PortList<SEND>>::Ctx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        Flatten::new(self.next.build(context, handoffs))
    }
}
