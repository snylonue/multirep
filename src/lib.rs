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
                .range(i..)
                .next()
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
        result.push_str(&s[end..pos]);
        result.push_str(new);
        end = pos + len;
    }

    result
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
        assert_eq!("Hana is kawaii", multi_replace("Hana is cute", &[("Rica", "Minami"), ("cute", "kawaii")]))
    }
}
