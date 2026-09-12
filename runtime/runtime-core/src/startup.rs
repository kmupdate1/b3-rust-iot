use crate::{Ota, OtaStatus};

pub async fn check<O>(
    ota: &mut O,
    current_v: &str,
) -> Result<OtaStatus, O::Error>
where
    O: Ota,
{
    if !ota.available(current_v).await? {
        return Ok(OtaStatus::UpToDate);
    }

    ota.update().await?;

    Ok(OtaStatus::ReadyToReboot)
}
