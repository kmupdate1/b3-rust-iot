use crate::command::Termination;
use crate::lifecycle::{Lifecycle, OnCreat, OnDestroy, OnPrepare, OnResume, OnStart, OnStop, OnSuspend};
use core::marker::PhantomData;
use crate::outcome::ApplicationOutcome;
use crate::run::Run;

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
    async fn create(mut self) -> Result<Runtime<Created, A>, <A as Lifecycle>::Error> {
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
    async fn prepare(mut self) -> Result<Runtime<Prepared, A>, <A as Lifecycle>::Error> {
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
    async fn start(mut self) -> Result<Runtime<Running, A>, <A as Lifecycle>::Error> {
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
    async fn stop(mut self) -> Result<Runtime<Stopped, A>, <A as Lifecycle>::Error> {
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
    async fn suspend(mut self) -> Result<Runtime<Suspended, A>, <A as Lifecycle>::Error> {
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
    async fn resume(mut self) -> Result<Runtime<Running, A>, <A as Lifecycle>::Error> {
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
    async fn stop(mut self) -> Result<Runtime<Stopped, A>, <A as Lifecycle>::Error> {
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
    async fn destroy(mut self) -> Result<(), <A as Lifecycle>::Error> {
        self.application.on_destroy().await?;

        Ok(())
    }
}

impl<A> Runtime<Uninitialized, A>
where
    A: OnCreat + OnPrepare + OnStart,
{
    async fn run_on(self) -> Result<Runtime<Running, A>, <A as Lifecycle>::Error> {
        let runtime = self.create().await?;
        let runtime = runtime.prepare().await?;
        runtime.start().await
    }
}

impl<A> Runtime<Running, A>
where
    A: OnStop + OnDestroy,
{
    async fn run_off(
        self,
        termination: Termination,
    ) -> Result<Termination, <A as Lifecycle>::Error> {
        let runtime = self.stop().await?;
        runtime.destroy().await?;

        Ok(termination)
    }
}

impl<A> Runtime<Uninitialized, A>
where
    A: OnCreat + OnPrepare + OnStart + OnStop + OnDestroy
    + Run<Output = ApplicationOutcome>,
{
    pub async fn run(self) -> Result<Termination, <A as Lifecycle>::Error> {
        let mut runtime = self.run_on().await?;

        let outcome = runtime.application.run().await;

        let termination = match outcome {
            ApplicationOutcome::Stop => Termination::Stop,
            ApplicationOutcome::Reboot => Termination::Reboot,
            ApplicationOutcome::Shutdown => Termination::Shutdown,
        };

        runtime.run_off(termination).await
    }
}

impl<A> Runtime<Uninitialized, A>
where
    A: OnCreat + OnPrepare + OnStart + OnSuspend + OnResume + OnStop + OnDestroy
    + Run<Output = ApplicationOutcome>,
{
    pub async fn run_with_sleep(self) -> Result<Termination, <A as Lifecycle>::Error> {
        let mut runtime= self.run_on().await?;

        let outcome = runtime.application.run().await;

        let termination = match outcome {
            ApplicationOutcome::Stop => Termination::Stop,
            ApplicationOutcome::Reboot => Termination::Reboot,
            ApplicationOutcome::Shutdown => Termination::Shutdown,
        };

        runtime.run_off(termination).await
    }
}
