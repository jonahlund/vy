//! The MIT License (MIT)
//! Copyright (c) 2020 Ryohei Machida
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to
//! deal in the Software without restriction, including without limitation the
//! rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
//! sell copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
//! EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
//! MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
//! IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//! DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
//! OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
//! USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::sync::atomic::{AtomicUsize, Ordering};

/// Dynamically updated size hint
#[doc(hidden)]
pub struct SizeHint {
    value: AtomicUsize,
}

impl SizeHint {
    /// Initialize size hint
    pub const fn new(v: usize) -> SizeHint {
        SizeHint {
            value: AtomicUsize::new(v),
        }
    }

    /// Get the current value
    #[inline]
    pub fn get(&self) -> usize {
        let value = self.value.load(Ordering::Acquire);
        value + value / 8 + 75
    }

    /// Update size hint based on given value.
    ///
    /// There is no guarantee that the value of get() after calling update() is
    /// same as the value passed on update()
    #[inline]
    pub fn update(&self, value: usize) {
        let mut old = self.value.load(Ordering::Acquire);
        if old == 0 {
            old = value;
        }
        self.value
            .store(old - old / 4 + value / 4, Ordering::Release);
    }
}

impl Default for SizeHint {
    fn default() -> Self {
        Self::new(0)
    }
}

#[test]
fn test_update() {
    let hint = SizeHint::new(0);

    for size in 1..=100 {
        let cap = hint.get();
        assert!(size <= cap);
        assert!(cap <= size + size / 8 + 75);
        hint.update(size);
    }
}
