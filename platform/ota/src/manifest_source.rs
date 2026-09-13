use crate::manifest::UpdateManifest;

pub trait ManifestSource {
    type Error;
    
    async fn fetch(&mut self) -> Result<UpdateManifest, Self::Error>;
}
