use core::{char, ptr};

use vy_core::{escape_char, Escape, PreEscaped};

pub struct Concat<T>(pub T);

#[macro_export]
macro_rules! concat {
    ($($arg:expr $(,)*),*) => {{
        const LEN: usize = {
            let mut sum = 0;
            $(sum += $crate::concat::Concat($arg).len();)*
            sum
        };
        #[allow(unused_assignments)]
        const CONCAT: [u8; LEN] = {
            let mut arr = [0u8; LEN];
            let mut n = 0;
            $({
                let arg = $crate::concat::Concat($arg);
                let dst = ((&mut arr[n]) as *mut u8);
                arg.encode(dst);
                n += arg.len();
            })*
            arr
        };
        unsafe { ::core::str::from_utf8_unchecked(&CONCAT) }
    }};
}

impl Concat<&str> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        let src = self.0.as_bytes().as_ptr();
        unsafe { ptr::copy_nonoverlapping(src, dst, self.len()) }
    }
}

impl Concat<bool> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.as_str()).encode(dst);
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self.0 {
            true => "true",
            false => "false",
        }
    }
}

impl Concat<char> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len_utf8()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        let slice = ptr::slice_from_raw_parts_mut(dst, self.len());
        unsafe {
            self.0.encode_utf8(&mut *slice);
        }
    }
}

// ==== Escaped types ====

impl Concat<Escape<&str>> {
    pub const fn len(&self) -> usize {
        let bytes = self.0 .0.as_bytes();
        let mut sum = 0;
        let mut i = 0;

        while i < bytes.len() {
            sum += Concat(Escape(bytes[i] as char)).len();
            i += 1;
        }

        sum
    }

    pub const fn encode(&self, dst: *mut u8) {
        let bytes = self.0 .0.as_bytes();
        let mut i = 0;
        let mut n = 0;

        while i < bytes.len() {
            let dst = unsafe { dst.add(n) };
            let char = Concat(Escape(bytes[i] as char));
            char.encode(dst);
            n += char.len();
            i += 1;
        }
    }
}

impl Concat<Escape<char>> {
    #[inline]
    pub const fn len(&self) -> usize {
        let char = self.0 .0;
        match escape_char(char) {
            Some(esc) => esc.len(),
            _ => char.len_utf8(),
        }
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        let char = self.0 .0;
        match escape_char(char) {
            Some(esc) => Concat(esc).encode(dst),
            _ => {
                Concat(char).encode(dst);
            }
        }
    }
}

impl Concat<Escape<bool>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0).encode(dst);
    }
}

// ==== Pre-escaped types ====

impl Concat<PreEscaped<&str>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0).encode(dst)
    }
}

impl Concat<PreEscaped<char>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0).encode(dst)
    }
}

impl Concat<PreEscaped<bool>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0).encode(dst)
    }
}

impl Concat<Escape<PreEscaped<&str>>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0 .0).encode(dst)
    }
}

impl Concat<Escape<PreEscaped<char>>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0 .0).encode(dst)
    }
}

impl Concat<Escape<PreEscaped<bool>>> {
    #[inline]
    pub const fn len(&self) -> usize {
        Concat(self.0 .0 .0).len()
    }

    #[inline]
    pub const fn encode(&self, dst: *mut u8) {
        Concat(self.0 .0 .0).encode(dst)
    }
}
