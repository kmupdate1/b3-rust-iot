use core::cmp::Ordering;

use heapless::String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String<32>>,
    pub build: Option<String<32>>,
}

impl Version {
    /// Compare SemVer precedence. Build metadata is intentionally ignored.
    pub fn precedence_cmp(&self, other: &Self) -> Ordering {
        match (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
        {
            Ordering::Equal => compare_pre_release(
                self.pre.as_ref().map(String::as_str),
                other.pre.as_ref().map(String::as_str),
            ),
            ordering => ordering,
        }
    }

    pub fn is_newer_than(&self, other: &Self) -> bool {
        self.precedence_cmp(other) == Ordering::Greater
    }
}

fn compare_pre_release(left: Option<&str>, right: Option<&str>) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        // A release version has higher precedence than a pre-release.
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(left), Some(right)) => compare_pre_release_parts(left, right),
    }
}

fn compare_pre_release_parts(left: &str, right: &str) -> Ordering {
    let mut left_parts = left.split('.');
    let mut right_parts = right.split('.');

    loop {
        match (left_parts.next(), right_parts.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(left), Some(right)) => {
                let ordering = compare_identifier(left, right);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }
    }
}

fn compare_identifier(left: &str, right: &str) -> Ordering {
    let left_numeric = !left.is_empty() && left.bytes().all(|byte| byte.is_ascii_digit());
    let right_numeric = !right.is_empty() && right.bytes().all(|byte| byte.is_ascii_digit());

    match (left_numeric, right_numeric) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => left.cmp(right),
        (true, true) => {
            let left = left.trim_start_matches('0');
            let right = right.trim_start_matches('0');
            let left = if left.is_empty() { "0" } else { left };
            let right = if right.is_empty() { "0" } else { right };

            left.len()
                .cmp(&right.len())
                .then_with(|| left.cmp(right))
        }
    }
}

pub mod parser;
