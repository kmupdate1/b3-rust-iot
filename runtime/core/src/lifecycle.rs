pub trait Lifecycle {
    type Error;
}

pub trait OnCreat: Lifecycle {
    async fn on_create(&mut self) -> Result<(), Self::Error>;
}

pub trait OnPrepare: Lifecycle {
    async fn on_prepare(&mut self) -> Result<(), Self::Error>;
}

pub trait OnStart: Lifecycle {
    async fn on_start(&mut self) -> Result<(), Self::Error>;
}

pub trait OnSuspend: Lifecycle {
    async fn on_suspend(&mut self) -> Result<(), Self::Error>;
}

pub trait OnResume: Lifecycle {
    async fn on_resume(&mut self) -> Result<(), Self::Error>;
}

pub trait OnStop: Lifecycle {
    async fn on_stop(&mut self) -> Result<(), Self::Error>;
}

pub trait OnDestroy: Lifecycle {
    async fn on_destroy(&mut self) -> Result<(), Self::Error>;
}

pub trait Process:
Lifecycle + OnCreat + OnPrepare + OnStart + OnStop + OnDestroy
{}

pub trait Suspendable:
Lifecycle + OnSuspend+ OnResume
{}
