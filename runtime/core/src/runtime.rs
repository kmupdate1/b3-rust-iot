use core::marker::PhantomData;
use crate::lifecycle::{Lifecycle, OnCreat, OnDestroy, OnPrepare, OnResume, OnStart, OnStop, OnSuspend};

pub struct Uninitialized;
pub struct Created;
pub struct Prepared;
pub struct Running;
pub struct Suspended;
pub struct Stopped;

pub struct Runtime<S, A> {
    application: A,
    _state: PhantomData<S>,
}

impl<A> Runtime<Uninitialized, A> {
    pub fn new(application: A) -> Self {
        Self {
            application,
            _state: PhantomData,
        }
    }
}

impl<A> Runtime<Uninitialized, A>
where
    A: OnCreat,
{
    pub async fn create(mut self) -> Result<Runtime<Created, A>, <A as Lifecycle>::Error> {
        self.application.on_create().await?;

        Ok(Runtime {
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Created, A>
where
    A: OnPrepare,
{
    pub async fn prepare(mut self) -> Result<Runtime<Prepared, A>, <A as Lifecycle>::Error> {
        self.application.on_prepare().await?;

        Ok(Runtime{
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Prepared, A>
where
    A: OnStart,
{
    pub async fn start(mut self) -> Result<Runtime<Running, A>, <A as Lifecycle>::Error> {
        self.application.on_start().await?;

        Ok(Runtime{
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Running, A>
where
    A: OnStop,
{
    pub async fn stop(mut self) -> Result<Runtime<Stopped, A>, <A as Lifecycle>::Error> {
        self.application.on_stop().await?;

        Ok(Runtime{
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Running, A>
where
    A: OnSuspend,
{
    pub async fn suspend(mut self) -> Result<Runtime<Suspended, A>, <A as Lifecycle>::Error> {
        self.application.on_suspend().await?;

        Ok(Runtime {
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Suspended, A>
where
    A: OnResume,
{
    pub async fn resume(mut self) -> Result<Runtime<Running, A>, <A as Lifecycle>::Error> {
        self.application.on_resume().await?;

        Ok(Runtime {
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Suspended, A>
where
    A: OnStop,
{
    pub async fn stop(mut self) -> Result<Runtime<Stopped, A>, <A as Lifecycle>::Error> {
        self.application.on_stop().await?;

        Ok(Runtime{
            application: self.application,
            _state: PhantomData,
        })
    }
}

impl<A> Runtime<Stopped, A>
where
    A: OnDestroy,
{
    pub async fn destroy(mut self) -> Result<(), <A as Lifecycle>::Error> {
        self.application.on_destroy().await?;

        Ok(())
    }
}
