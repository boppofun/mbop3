//! Bit reader matching minimp3's `bs_t`.

/// Reads bits from `head` followed by `tail` (for Layer III main data: the
/// bit reservoir from previous frames, then this frame's payload), without
/// copying them together. Bytes past the end read as zero.
pub(crate) struct BitReader<'a> {
    head: &'a [u8],
    tail: &'a [u8],
    /// Position in bits. May exceed `limit` after a failed read.
    pub pos: i32,
    /// Readable bits. May be negative for a truncated frame.
    pub limit: i32,
}

impl<'a> BitReader<'a> {
    /// A reader over `buf` with a limit of `bytes` (which may be less than
    /// `buf.len()`, or negative).
    pub fn new(buf: &'a [u8], bytes: i32) -> Self {
        BitReader {
            head: buf,
            tail: &[],
            pos: 0,
            limit: bytes * 8,
        }
    }

    /// A reader over `head` then `tail`, limited to their total length.
    pub fn new_split(head: &'a [u8], tail: &'a [u8]) -> Self {
        BitReader {
            head,
            tail,
            pos: 0,
            limit: ((head.len() + tail.len()) * 8) as i32,
        }
    }

    #[inline]
    pub fn byte(&self, i: usize) -> u32 {
        match self.head.get(i) {
            Some(b) => *b as u32,
            None => self.tail.get(i - self.head.len()).copied().unwrap_or(0) as u32,
        }
    }

    /// The bytes from `start` (in bytes) to the end of the data.
    pub fn split_from(&self, start: usize) -> (&'a [u8], &'a [u8]) {
        if start <= self.head.len() {
            (&self.head[start..], self.tail)
        } else {
            (&[], self.tail.get(start - self.head.len()..).unwrap_or(&[]))
        }
    }

    /// Reads `n` (1..=24) bits. Past the limit this returns 0 but still advances.
    pub fn get_bits(&mut self, n: i32) -> u32 {
        let s = (self.pos & 7) as u32;
        let mut shl = n + s as i32;
        let mut p = (self.pos >> 3) as usize;
        self.pos += n;
        if self.pos > self.limit {
            return 0;
        }
        let mut cache = 0u32;
        let mut next = self.byte(p) & (255 >> s);
        p += 1;
        loop {
            shl -= 8;
            if shl <= 0 {
                break;
            }
            cache |= next << shl;
            next = self.byte(p);
            p += 1;
        }
        cache | (next >> -shl)
    }
}
