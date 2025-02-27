use super::{PushBuild, PushBuildBase};

use std::marker::PhantomData;

use crate::compiled::filter_map::FilterMap;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::handoff_list::PortList;
use crate::scheduled::port::SEND;

pub struct FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    next: Next,
    func: Func,
    _phantom: PhantomData<fn(In)>,
}
impl<Next, Func, In> FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    pub fn new(next: Next, func: Func) -> Self {
        Self {
            next,
            func,
            _phantom: PhantomData,
        }
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'hof, Next, Func, In>
where
    Next: PushBuild,
= FilterMap<Next::Build<'slf, 'hof>, impl FnMut(In) -> Option<Next::ItemIn>, In>;

impl<Next, Func, In> PushBuildBase for FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    type ItemIn = In;
    type Build<'slf, 'hof> = PushBuildImpl<'slf, 'hof, Next, Func, In>;
}

impl<Next, Func, In> PushBuild for FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;

    fn build<'slf, 'hof>(
        &'slf mut self,
        context: &Context<'_>,
        handoffs: <Self::OutputHandoffs as PortList<SEND>>::Ctx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        FilterMap::new(|x| (self.func)(x), self.next.build(context, handoffs))
    }
}
