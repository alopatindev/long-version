use smallvec::SmallVec;
use std::{
    cmp::{max, Ordering},
    iter,
    str::FromStr,
};

#[derive(Eq)]
pub struct Version(VersionInner);

type VersionInner = SmallVec<[u64; 4]>;

impl Version {
    fn normalize(left: &Self, right: &Self) -> (VersionInner, VersionInner) {
        let Self(left) = left;
        let Self(right) = right;
        let length = max(left.len(), right.len());
        let left = Self::with_padding(&left, length);
        let right = Self::with_padding(&right, length);
        debug_assert_eq!(left.len(), right.len());
        (left, right)
    }

    fn with_padding(items: &[u64], total_length: usize) -> VersionInner {
        let zeros = iter::once(0).take(total_length - items.len());
        items.iter().copied().chain(zeros).collect::<VersionInner>()
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let (this, other) = Self::normalize(self, other);
        this.eq(&other)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (this, other) = Self::normalize(self, other);
        this.partial_cmp(&other)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let (this, other) = Self::normalize(self, other);
        this.cmp(&other)
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(version: &str) -> Result<Self, Self::Err> {
        let inner = version
            .split('.')
            .map(|i| i.parse().unwrap_or(0)) // ignore things like "alpha1", "rc1", etc. for simplicity
            .collect();
        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn compare_versions() -> Result<()> {
        assert!("1.2.3.4".parse::<Version>()? == "1.2.3.4".parse::<Version>()?);
        assert!("1.2.3.3".parse::<Version>()? < "1.2.3.4".parse::<Version>()?);
        assert!("1.2.0.4".parse::<Version>()? < "1.2.3.4".parse::<Version>()?);
        assert!("2.1.1.9".parse::<Version>()? > "2.1.1.8".parse::<Version>()?);
        assert!("1.1.1".parse::<Version>()? < "1.1.1.2".parse::<Version>()?);
        assert!("1.0.0".parse::<Version>()? < "1.1.0.0".parse::<Version>()?);
        assert!("2.0.0".parse::<Version>()? > "1.1.0.0".parse::<Version>()?);
        assert!("1.1.0".parse::<Version>()? == "1.1.0.0".parse::<Version>()?);
        assert!("1.1".parse::<Version>()? == "1.1.0".parse::<Version>()?);
        assert!("1.1-rc1".parse::<Version>()? == "1.1-rc2".parse::<Version>()?);
        Ok(())
    }
}
