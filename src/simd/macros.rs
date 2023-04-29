// Unfortunately this can't be a parametric function because we can't be generic over unsafe functions ...
// it's conceptually
// fn::<N: usize>(bytes: &mut Bytes, matcher: fn(*const u8) -> usize)
macro_rules! simd_batch_match {
    ($N:literal, $bytes:ident, $matcher:ident) => {
        let end = unsafe { $bytes.end().sub($N) };
        let mut ptr = $bytes.as_ptr();
        while end >= ptr {
            // SAFETY: ptr is without bounds
            unsafe {
                let advance = $matcher(ptr);
                ptr = ptr.add(advance);

                if advance != $N {
                    $bytes.set_cursor(ptr);
                    return;
                }
            }
        }
        // SAFETY: ptr is without bounds
        unsafe { $bytes.set_cursor(ptr); }
    }
}

// NOTE: this importantly differs from the above by breaking vs returning
// this is designed for matchers that may yield false-negatives
// so you can have a slower cold fallback matcher to check & re-enter
// so far this is only used in SWAR, the other SIMD impls block mathers
// never yield falst-negatigves
macro_rules! simd_batch_match_fallback {
    ($N:literal, $bytes:ident, $matcher:ident) => {
        let end = unsafe { $bytes.end().sub($N) };
        let mut ptr = $bytes.as_ptr();
        while end >= ptr {
            // SAFETY: ptr is without bounds
            unsafe {
                let advance = $matcher(ptr);
                ptr = ptr.add(advance);

                if advance != $N {
                    break;
                }
            }
        }
        // SAFETY: ptr is without bounds
        unsafe { $bytes.set_cursor(ptr); }
    }
}
