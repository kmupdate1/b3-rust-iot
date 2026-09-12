use crate::Ota;

pub async fn check<O>(
    ota: &mut O,
    current_v: &str,
) -> Result<bool, O::Error>
where
    O: Ota,
{
    if !ota.available(current_v).await? {
        return Ok(false);
    }

    ota.update().await?;

    Ok(true)
}
