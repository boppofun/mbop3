//! Bit reader matching minimp3's `bs_t`.

pub(crate) struct BitReader<'a> {
    pub buf: &'a [u8],
    /// Position in bits. May exceed `limit` after a failed read.
    pub pos: i32,
    /// Readable bits. May be negative for a truncated frame.
    pub limit: i32,
}

impl<'a> BitReader<'a> {
    pub fn new(buf: &'a [u8], bytes: i32) -> Self {
        BitReader {
            buf,
            pos: 0,
            limit: bytes * 8,
        }
    }

    #[inline]
    fn byte(&self, i: usize) -> u32 {
        self.buf.get(i).copied().unwrap_or(0) as u32
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
