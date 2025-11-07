#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimeter {
    /// Return (start, end) indices of next delimiter occurrence in `s`.
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimeter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        // keep looping until we either return a non-empty token or run out
        loop {
            let rem = self.remainder.as_mut()?; // None -> done

            if rem.is_empty() {
                // empty remainder: consume and stop
                self.remainder = None;
                return None;
            }

            if let Some((delim_start, delim_end)) = self.delimiter.find_next(rem) {
                let token = &rem[..delim_start];
                // advance remainder past the delimiter
                *rem = &rem[delim_end..];

                // skip empty tokens produced by consecutive delimiters
                if token.is_empty() {
                    continue;
                } else {
                    return Some(token);
                }
            } else {
                // no more delimiters: return remainder if non-empty, otherwise None
                return self.remainder.take().and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                });
            }
        }
    }
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        if self.is_empty() {
            // empty delimiter: treat as "split on whitespace chars" (one char delimiter)
            s.char_indices()
                .find(|(_, c)| c.is_whitespace())
                .map(|(start, c)| (start, start + c.len_utf8()))
        } else {
            s.find(*self).map(|start| (start, start + self.len()))
        }
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}

pub fn until_char<'s>(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("strSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn itworks() {
         let haystack = "a b c  d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    println!("letters = {:?}", letters);
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, "").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d"]);
    }
}
