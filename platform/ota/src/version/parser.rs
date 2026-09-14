use heapless::String;
use crate::ManifestError;
use crate::version::Version;

pub fn parse(sem_ver: &str) -> Result<Version, ManifestError> {
    let value = sem_ver.strip_prefix('v').unwrap_or(sem_ver);

    let (release, build) = match value.split_once('+') {
        None => { (value, None) }
        Some((release, build)) => { (release, Some(build)) }
    };

    let (core, pre) = match release.split_once('-') {
        None => { (release, None) }
        Some((core, pre)) => { (core, Some(pre)) }
    };

    let mut parts = core.split('.');

    let major = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(ManifestError::InvalidVersion)?;

    let minor = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(ManifestError::InvalidVersion)?;

    let patch = parts
        .next()
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or(ManifestError::InvalidVersion)?;

    if parts.next().is_some() {
        return Err(ManifestError::InvalidVersion);
    }

    let pre = optional_string::<32>(pre)?;
    let build = optional_string::<32>(build)?;

    Ok(Version{
        major,
        minor,
        patch,
        pre,
        build,
    })
}

fn optional_string<const N: usize>(
    value: Option<&str>,
) -> Result<Option<String<N>>, ManifestError> {
    match value {
        Some(value) => {
            let mut result = String::new();
            result
                .push_str(value)
                .map_err(|_| ManifestError::FieldTooLong)?;

            Ok(Some(result))
        }
        None => Ok(None),
    }
}
