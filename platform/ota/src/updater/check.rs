use crate::{manifest, ManifestSource, OtaError, UpdateManifest, Version};

pub(super) async fn for_update<Source, TargetError>(
    source: &mut Source,
    current: &Version,
    buffer: &mut [u8],
) -> Result<Option<UpdateManifest>, OtaError<Source::Error, TargetError>>
where
    Source: ManifestSource,
{
    let length = source.read(buffer).await.map_err(OtaError::Source)?;
    let document = buffer
        .get(..length)
        .ok_or(OtaError::Manifest(crate::ManifestError::InvalidManifest))?;
    let manifest = manifest::parser::json(document).map_err(OtaError::Manifest)?;

    Ok(manifest
        .version
        .has_higher_precedence_than(current)
        .then_some(manifest))
}
