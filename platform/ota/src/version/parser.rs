use heapless::String;

use crate::{ManifestError, Version};

pub fn parse(sem_ver: &str) -> Result<Version, ManifestError> {
    let value = sem_ver.strip_prefix('v').unwrap_or(sem_ver);
    let (release, build) = value
        .split_once('+')
        .map_or((value, None), |(release, build)| (release, Some(build)));
    let (core, pre) = release
        .split_once('-')
        .map_or((release, None), |(core, pre)| (core, Some(pre)));
    let mut parts = core.split('.');

    let major = number(parts.next())?;
    let minor = number(parts.next())?;
    let patch = number(parts.next())?;
    if parts.next().is_some() {
        return Err(ManifestError::InvalidVersion);
    }

    Ok(Version {
        major,
        minor,
        patch,
        pre: optional_string(pre)?,
        build: optional_string(build)?,
    })
}

fn number(value: Option<&str>) -> Result<u32, ManifestError> {
    value
        .and_then(|value| value.parse().ok())
        .ok_or(ManifestError::InvalidVersion)
}

fn optional_string(value: Option<&str>) -> Result<Option<String<32>>, ManifestError> {
    value
        .map(|value| {
            if value.is_empty() {
                return Err(ManifestError::InvalidVersion);
            }
            let mut result = String::new();
            result
                .push_str(value)
                .map_err(|_| ManifestError::FieldTooLong)?;
            Ok(result)
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_has_higher_precedence_than_pre_release() {
        let release = parse("1.2.3").unwrap();
        let candidate = parse("1.2.3-rc.1").unwrap();
        assert!(release.has_higher_precedence_than(&candidate));
    }

    #[test]
    fn build_metadata_does_not_change_precedence() {
        let left = parse("1.2.3+one").unwrap();
        let right = parse("1.2.3+two").unwrap();
        assert!(!left.has_higher_precedence_than(&right));
        assert!(!right.has_higher_precedence_than(&left));
    }
}