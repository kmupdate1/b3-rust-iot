use crate::lifecycle::{Lifecycle, OnCreat, OnDestroy, OnPrepare, OnResume, OnStart, OnStop, OnSuspend};

pub struct Application
where
    Self: OnCreat + OnPrepare + OnStart + OnSuspend + OnResume + OnStop + OnDestroy,
{}
pub struct Builder {}

impl OnCreat for Application {
    async fn on_create(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnPrepare for Application {
    async fn on_prepare(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnStart for Application {
    async fn on_start(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnSuspend for Application {
    async fn on_suspend(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnResume for Application {
    async fn on_resume(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnStop for Application {
    async fn on_stop(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OnDestroy for Application {
    async fn on_destroy(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Lifecycle for Application {
    type Error = ();
}

impl Application {
    pub fn builder() -> Builder {
        Builder {}
    }
}

impl Builder {
    pub fn target<T>(self, target: T) -> Self {
        todo!()
    }

    pub fn attach<T>(self, component: T) -> Self {
        todo!()
    }

    pub fn build(self) -> Application {
        todo!()
    }
}
