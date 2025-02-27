use super::{PushBuild, PushBuildBase};

use std::marker::PhantomData;

use crate::compiled::for_each::ForEach;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::handoff_list::PortList;
use crate::scheduled::port::SEND;
use crate::tt;

pub struct ForEachPushBuild<Func, In>
where
    Func: FnMut(In),
{
    func: Func,
    _phantom: PhantomData<fn(In)>,
}
impl<Func, In> ForEachPushBuild<Func, In>
where
    Func: FnMut(In),
{
    pub fn new(func: Func) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'hof, Func, In> = ForEach<In, impl FnMut(In)>;

impl<Func, In> PushBuildBase for ForEachPushBuild<Func, In>
where
    Func: FnMut(In),
{
    type ItemIn = In;
    type Build<'slf, 'hof> = PushBuildImpl<'slf, 'hof, Func, In>;
}

impl<Func, In> PushBuild for ForEachPushBuild<Func, In>
where
    Func: FnMut(In),
{
    type OutputHandoffs = tt!();

    fn build<'slf, 'hof>(
        &'slf mut self,
        _context: &Context<'_>,
        (): <Self::OutputHandoffs as PortList<SEND>>::Ctx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        ForEach::new(|x| (self.func)(x))
    }
}
