use super::{PushBuild, PushBuildBase};

use crate::compiled::filter::Filter;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::handoff_list::PortList;
use crate::scheduled::port::SEND;

pub struct FilterPushBuild<Next, Func>
where
    Next: PushBuild,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    next: Next,
    func: Func,
}
impl<Next, Func> FilterPushBuild<Next, Func>
where
    Next: PushBuild,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    pub fn new(next: Next, func: Func) -> Self {
        Self { next, func }
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'hof, Next, Func>
where
    Next: PushBuild,
= Filter<Next::ItemIn, impl FnMut(&Next::ItemIn) -> bool, Next::Build<'slf, 'hof>>;

impl<Next, Func> PushBuildBase for FilterPushBuild<Next, Func>
where
    Next: PushBuild,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    type ItemIn = Next::ItemIn;
    type Build<'slf, 'hof> = PushBuildImpl<'slf, 'hof, Next, Func>;
}

impl<Next, Func> PushBuild for FilterPushBuild<Next, Func>
where
    Next: PushBuild,
    Func: FnMut(&Next::ItemIn) -> bool,
{
    type OutputHandoffs = Next::OutputHandoffs;

    fn build<'slf, 'hof>(
        &'slf mut self,
        context: &Context<'_>,
        handoffs: <Self::OutputHandoffs as PortList<SEND>>::Ctx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        Filter::new(|x| (self.func)(x), self.next.build(context, handoffs))
    }
}
