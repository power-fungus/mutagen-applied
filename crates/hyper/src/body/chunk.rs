use std::fmt;

use bytes::{Buf, Bytes};

/// A piece of a message body.
///
/// These are returned by [`Body`](::Body). It is an efficient buffer type.
///
/// A `Chunk` can be easily created by many of Rust's standard types that
/// represent a collection of bytes, using `Chunk::from`.
pub struct Chunk {
    /// The buffer of bytes making up this body.
    bytes: Bytes,
}

// An unexported type to prevent locking `Chunk::into_iter()` to `Bytes::into_iter()`.
#[derive(Debug)]
pub struct IntoIter {
    inner: <Bytes as IntoIterator>::IntoIter,
}


#[cfg_attr(test, ::mutagen::mutate)] impl Chunk {
    /// Converts this `Chunk` directly into the `Bytes` type without copies.
    ///
    /// This is simply an inherent alias for `Bytes::from(chunk)`, which exists,
    /// but doesn't appear in rustdocs.
    #[inline]
    pub fn into_bytes(self) -> Bytes {
        self.into()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Buf for Chunk {
    #[inline]
    fn remaining(&self) -> usize {
        //perf: Bytes::len() isn't inline yet,
        //so it's slightly slower than checking
        //the length of the slice.
        self.bytes().len()
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        self.bytes.advance(cnt);
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<Vec<u8>> for Chunk {
    #[inline]
    fn from(v: Vec<u8>) -> Chunk {
        Chunk::from(Bytes::from(v))
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<&'static [u8]> for Chunk {
    #[inline]
    fn from(slice: &'static [u8]) -> Chunk {
        Chunk::from(Bytes::from_static(slice))
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<String> for Chunk {
    #[inline]
    fn from(s: String) -> Chunk {
        s.into_bytes().into()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<&'static str> for Chunk {
    #[inline]
    fn from(slice: &'static str) -> Chunk {
        slice.as_bytes().into()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<Bytes> for Chunk {
    #[inline]
    fn from(bytes: Bytes) -> Chunk {
        Chunk {
            bytes: bytes,
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl From<Chunk> for Bytes {
    #[inline]
    fn from(chunk: Chunk) -> Bytes {
        chunk.bytes
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl ::std::ops::Deref for Chunk {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl AsRef<[u8]> for Chunk {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl fmt::Debug for Chunk {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.bytes, f)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Default for Chunk {
    #[inline]
    fn default() -> Chunk {
        Chunk::from(Bytes::new())
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl IntoIterator for Chunk {
    type Item = u8;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.bytes.into_iter(),
        }
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Extend<u8> for Chunk {
    #[inline]
    fn extend<T>(&mut self, iter: T) where T: IntoIterator<Item=u8> {
        self.bytes.extend(iter)
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Iterator for IntoIter {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl ExactSizeIterator for IntoIter {}

#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_chunk_static_buf(b: &mut Bencher) {
        use bytes::BufMut;

        let s = "Hello, World!";
        b.bytes = s.len() as u64;

        let mut dst = Vec::with_capacity(128);

        b.iter(|| {
            let chunk = crate::Chunk::from(s);
            dst.put(chunk);
            ::test::black_box(&dst);
            dst.clear();
        })
    }
}

