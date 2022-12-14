use std::collections::BTreeMap;

/// Multiple version of `str::replace` which replaces multiple patterns at a time.
///
///
/// ```
/// use multirep::multi_replace;
///
/// let s = "Hana is cute";
/// let r = multi_replace(s, &[("Hana", "Minami"), ("cute", "kawaii")]);
/// assert_eq!(r, "Minami is kawaii");
/// ```
///
/// The replacement takes place in order of `pats`
///
/// ```
/// use multirep::multi_replace;
/// assert_eq!("Minami is kawaii", multi_replace("Hana is cute", &[("Hana", "Minami"), ("cute", "kawaii"), ("na", "no")]));
/// ```
///
/// Replacement will not be interfere with previosly replaced strings.
///
/// ```
/// use multirep::multi_replace;
/// assert_eq!("Minami is kawaii", multi_replace("Hana is cute", &[("Hana", "Minami"), ("cute", "kawaii"), ("kawaii", "hot")]));
/// ```
///
pub fn multi_replace(s: &str, pats: &[(&str, &str)]) -> String {
    let mut indices = BTreeMap::new();

    for (pat, new) in pats {
        for (i, p) in s.match_indices(pat) {
            if indices
                .range(..=i)
                .next_back()
                .map(|(pos, (len, _))| pos + len <= i)
                .unwrap_or(true)
            {
                indices.insert(i, (p.len(), *new));
            }
        }
    }

    let mut result = String::new();
    let mut end = 0usize;

    for (pos, (len, new)) in indices {
        // SAFETY: pos is returned by `str::match_indices`, which is valid
        // end >= 0 since it starts at 0 and only increases
        // end < pos since `str::match_indices` doesn't overlap
        // len is the length of one pattern string, so `pos + len`(`end`) should be on unicode boundaries.
        result.push_str(unsafe { s.get_unchecked(end..pos) });
        result.push_str(new);
        end = pos + len;
    }

    if end < s.len() {
        // SAFETY: end >= 0 and is on unicode boundaries as above
        // end < s.len()
        result.push_str(unsafe { s.get_unchecked(end..) });
    }

    result
}

/// Exchanges two patterns in a string
/// ```
/// use multirep::exchange;
/// assert_eq!("foo bar", exchange("bar foo", "foo", "bar"));
/// ```
pub fn exchange(s: &str, a: &str, b: &str) -> String {
    // if a contains b, searching b first will also match a substring of a.
    // search the longer one to avoid such a situation.
    let pat = if a.len() > b.len() {
        [(a, b), (b, a)]
    } else {
        [(b, a), (a, b)]
    };
    multi_replace(s, &pat)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn replace() {
        let s = "Hana is cute";

        let r = multi_replace(s, &[("Hana", "Minami"), ("cute", "kawaii")]);
        assert_eq!(r, "Minami is kawaii");
    }

    #[test]
    fn not_match() {
        assert_eq!(
            "Hana is kawaii",
            multi_replace("Hana is cute", &[("Rica", "Minami"), ("cute", "kawaii")])
        )
    }

    #[test]
    fn remain() {
        assert_eq!(
            "Hana is kawaii",
            multi_replace("Minami is kawaii", &[("Minami", "Hana")])
        )
    }

    #[test]
    fn overlap() {
        assert_eq!(
            "Both Minami and Hana are kawaii",
            multi_replace(
                "Bouh Aoi and Hana are kawaii",
                &[("Bouh", "Both"), ("Aoi", "Minami"), ("oi", "io")]
            )
        )
    }

    #[test]
    fn exchange() {
        let s = "Both Hana and Minami are kawaii";

        assert_eq!(
            "Both Minami and Hana are kawaii",
            super::exchange(s, "Minami", "Hana")
        );
        assert_eq!(
            "Both Minami and Hana are kawaii",
            super::exchange(s, "Hana", "Minami")
        );
        assert_eq!(
            "Both Hinata and Hina are kawaii",
            super::exchange("Both Hina and Hinata are kawaii", "Hina", "Hinata")
        );
        assert_eq!(
            "Both Hinata and Hina are kawaii",
            super::exchange("Both Hina and Hinata are kawaii", "Hinata", "Hina")
        );
    }
}
