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
