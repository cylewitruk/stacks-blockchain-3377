use std::{fmt, error, fmt::Write};

// borrowed from Andrew Poelstra's rust-bitcoin library
/// Convert a hexadecimal-encoded string to its corresponding bytes
pub fn hex_bytes(s: &str) -> Result<Vec<u8>, HexError> {
    let mut v = vec![];
    let mut iter = s.chars().pair();
    // Do the parsing
    iter.by_ref().fold(Ok(()), |e, (f, s)| {
        if e.is_err() {
            e
        } else {
            match (f.to_digit(16), s.to_digit(16)) {
                (None, _) => Err(HexError::BadCharacter(f)),
                (_, None) => Err(HexError::BadCharacter(s)),
                (Some(f), Some(s)) => {
                    v.push((f * 0x10 + s) as u8);
                    Ok(())
                }
            }
        }
    })?;
    // Check that there was no remainder
    match iter.remainder() {
        Some(_) => Err(HexError::BadLength(s.len())),
        None => Ok(v),
    }
}

/// Hex deserialization error
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum HexError {
    /// Length was not 64 characters
    BadLength(usize),
    /// Non-hex character in string
    BadCharacter(char),
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HexError::BadLength(n) => write!(f, "bad length {} for sha256d hex string", n),
            HexError::BadCharacter(c) => write!(f, "bad character {} in sha256d hex string", c),
        }
    }
}

impl error::Error for HexError {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            HexError::BadLength(_) => "sha256d hex string non-64 length",
            HexError::BadCharacter(_) => "sha256d bad hex character",
        }
    }
}

pub struct Pair<I>
where
    I: Iterator,
{
    iter: I,
    last_elem: Option<I::Item>,
}

impl<I: Iterator> Iterator for Pair<I> {
    type Item = (I::Item, I::Item);

    #[inline]
    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        let elem1 = self.iter.next();
        if elem1.is_none() {
            None
        } else {
            let elem2 = self.iter.next();
            if elem2.is_none() {
                self.last_elem = elem1;
                None
            } else {
                Some((elem1.unwrap(), elem2.unwrap()))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter.size_hint() {
            (n, None) => (n / 2, None),
            (n, Some(m)) => (n / 2, Some(m / 2)),
        }
    }
}

impl<I: Iterator> Pair<I> {
    /// Returns the last element of the iterator if there were an odd
    /// number of elements remaining before it was Pair-ified.
    #[inline]
    pub fn remainder(self) -> Option<I::Item> {
        self.last_elem
    }
}

/// Returns an iterator that returns elements of the original iterator 2 at a time
pub trait Pairable: Sized + Iterator {
    /// Returns an iterator that returns elements of the original iterator 2 at a time
    fn pair(self) -> Pair<Self>;
}

impl<I: Iterator> Pairable for I {
    /// Creates an iterator that yields pairs of elements from the underlying
    /// iterator, yielding `None` when there are fewer than two elements to
    /// return.
    #[inline]
    fn pair(self) -> Pair<I> {
        Pair {
            iter: self,
            last_elem: None,
        }
    }
}

/// Convert a slice of u8 to a hex string
pub fn to_hex(s: &[u8]) -> String {
    let mut r = String::with_capacity(s.len() * 2);
    for b in s.iter() {
        write!(r, "{:02x}", b).unwrap();
    }
    return r;
}