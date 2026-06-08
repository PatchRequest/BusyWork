#[derive(Clone, Default)]
pub struct WorkData {
    bytes: Vec<u8>,
}

pub trait FeedWork {
    fn write_work_bytes(&self, out: &mut Vec<u8>);
}

macro_rules! impl_feed_int {
    ($($t:ty),*) => {
        $(impl FeedWork for $t {
            fn write_work_bytes(&self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_ne_bytes());
            }
        })*
    };
}

impl_feed_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl FeedWork for bool {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.push(*self as u8);
    }
}

impl FeedWork for str {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_bytes());
    }
}

impl FeedWork for String {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self.as_bytes());
    }
}

impl FeedWork for [u8] {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
}

impl FeedWork for Vec<u8> {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
}

impl<const N: usize> FeedWork for [u8; N] {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(self);
    }
}

impl<T: FeedWork + ?Sized> FeedWork for &T {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        (*self).write_work_bytes(out);
    }
}

impl<T: FeedWork + ?Sized> FeedWork for Box<T> {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        (**self).write_work_bytes(out);
    }
}

impl WorkData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, data: &(impl FeedWork + ?Sized)) {
        data.write_work_bytes(&mut self.bytes);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// XOR work bytes cyclically into buf, creating a data dependency.
    pub(crate) fn blend_into(&self, buf: &mut [u8]) {
        if self.bytes.is_empty() {
            return;
        }
        for (i, byte) in buf.iter_mut().enumerate() {
            *byte ^= self.bytes[i % self.bytes.len()];
        }
    }

    /// Fold all bytes into a u64 seed for control-flow integration.
    pub(crate) fn blend_seed(&self) -> u64 {
        let mut seed: u64 = 0;
        for (i, &b) in self.bytes.iter().enumerate() {
            seed ^= (b as u64).wrapping_shl((i as u32).wrapping_mul(7) & 63);
        }
        seed
    }

    /// Derive a usize from work data at the given logical index.
    pub(crate) fn derive_usize(&self, index: usize) -> usize {
        if self.bytes.is_empty() {
            return 0;
        }
        let offset = index.wrapping_mul(7) % self.bytes.len();
        let mut buf = [0u8; 8];
        for (i, slot) in buf.iter_mut().enumerate() {
            *slot = self.bytes[(offset + i) % self.bytes.len()];
        }
        usize::from_ne_bytes(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Construction & feed ────────────────────────────────────────────

    #[test]
    fn new_is_empty() {
        let wd = WorkData::new();
        assert!(wd.is_empty());
        assert_eq!(wd.as_bytes().len(), 0);
    }

    #[test]
    fn default_is_empty() {
        let wd = WorkData::default();
        assert!(wd.is_empty());
    }

    #[test]
    fn feed_u8() {
        let mut wd = WorkData::new();
        wd.feed(&0xABu8);
        assert_eq!(wd.as_bytes(), &[0xAB]);
    }

    #[test]
    fn feed_u16() {
        let mut wd = WorkData::new();
        wd.feed(&0x1234u16);
        assert_eq!(wd.as_bytes().len(), 2);
        assert_eq!(wd.as_bytes(), &0x1234u16.to_ne_bytes());
    }

    #[test]
    fn feed_u32() {
        let mut wd = WorkData::new();
        wd.feed(&42u32);
        assert!(!wd.is_empty());
        assert_eq!(wd.as_bytes().len(), 4);
        assert_eq!(wd.as_bytes(), &42u32.to_ne_bytes());
    }

    #[test]
    fn feed_u64() {
        let mut wd = WorkData::new();
        wd.feed(&0xDEADBEEFCAFEBABEu64);
        assert_eq!(wd.as_bytes().len(), 8);
    }

    #[test]
    fn feed_u128() {
        let mut wd = WorkData::new();
        wd.feed(&1u128);
        assert_eq!(wd.as_bytes().len(), 16);
    }

    #[test]
    fn feed_usize() {
        let mut wd = WorkData::new();
        wd.feed(&999usize);
        assert_eq!(wd.as_bytes().len(), std::mem::size_of::<usize>());
    }

    #[test]
    fn feed_i8() {
        let mut wd = WorkData::new();
        wd.feed(&(-1i8));
        assert_eq!(wd.as_bytes(), &(-1i8).to_ne_bytes());
    }

    #[test]
    fn feed_i16() {
        let mut wd = WorkData::new();
        wd.feed(&(-256i16));
        assert_eq!(wd.as_bytes().len(), 2);
    }

    #[test]
    fn feed_i32() {
        let mut wd = WorkData::new();
        wd.feed(&(-42i32));
        assert_eq!(wd.as_bytes(), &(-42i32).to_ne_bytes());
    }

    #[test]
    fn feed_i64() {
        let mut wd = WorkData::new();
        wd.feed(&i64::MIN);
        assert_eq!(wd.as_bytes().len(), 8);
    }

    #[test]
    fn feed_i128() {
        let mut wd = WorkData::new();
        wd.feed(&i128::MAX);
        assert_eq!(wd.as_bytes().len(), 16);
    }

    #[test]
    fn feed_isize() {
        let mut wd = WorkData::new();
        wd.feed(&(-1isize));
        assert_eq!(wd.as_bytes().len(), std::mem::size_of::<isize>());
    }

    #[test]
    fn feed_f32() {
        let mut wd = WorkData::new();
        wd.feed(&3.14f32);
        assert_eq!(wd.as_bytes().len(), 4);
        assert_eq!(wd.as_bytes(), &3.14f32.to_ne_bytes());
    }

    #[test]
    fn feed_f64() {
        let mut wd = WorkData::new();
        wd.feed(&2.718f64);
        assert_eq!(wd.as_bytes().len(), 8);
        assert_eq!(wd.as_bytes(), &2.718f64.to_ne_bytes());
    }

    #[test]
    fn feed_bool_true() {
        let mut wd = WorkData::new();
        wd.feed(&true);
        assert_eq!(wd.as_bytes(), &[1]);
    }

    #[test]
    fn feed_bool_false() {
        let mut wd = WorkData::new();
        wd.feed(&false);
        assert_eq!(wd.as_bytes(), &[0]);
    }

    #[test]
    fn feed_str_literal() {
        let mut wd = WorkData::new();
        wd.feed("hello");
        assert_eq!(wd.as_bytes(), b"hello");
    }

    #[test]
    fn feed_string_owned() {
        let mut wd = WorkData::new();
        let s = String::from("world");
        wd.feed(&s);
        assert_eq!(wd.as_bytes(), b"world");
    }

    #[test]
    fn feed_byte_slice() {
        let mut wd = WorkData::new();
        let data: &[u8] = &[1, 2, 3];
        wd.feed(data);
        assert_eq!(wd.as_bytes(), &[1, 2, 3]);
    }

    #[test]
    fn feed_vec_u8() {
        let mut wd = WorkData::new();
        let data = vec![10u8, 20, 30];
        wd.feed(&data);
        assert_eq!(wd.as_bytes(), &[10, 20, 30]);
    }

    #[test]
    fn feed_byte_array() {
        let mut wd = WorkData::new();
        wd.feed(&[0xAA, 0xBB, 0xCC]);
        assert_eq!(wd.as_bytes(), &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn feed_boxed_u32() {
        let mut wd = WorkData::new();
        let boxed: Box<u32> = Box::new(42);
        wd.feed(&boxed);
        assert_eq!(wd.as_bytes(), &42u32.to_ne_bytes());
    }

    #[test]
    fn feed_boxed_str() {
        let mut wd = WorkData::new();
        let boxed: Box<str> = "boxed".into();
        wd.feed(&boxed);
        assert_eq!(wd.as_bytes(), b"boxed");
    }

    #[test]
    fn feed_reference_delegates() {
        let mut wd1 = WorkData::new();
        wd1.feed(&42u32);
        let mut wd2 = WorkData::new();
        let val = 42u32;
        let r = &val;
        wd2.feed(r);
        assert_eq!(wd1.as_bytes(), wd2.as_bytes());
    }

    // ── Accumulation ───────────────────────────────────────────────────

    #[test]
    fn feed_accumulates_bytes() {
        let mut wd = WorkData::new();
        wd.feed(&1u8);
        assert_eq!(wd.as_bytes().len(), 1);
        wd.feed(&2u8);
        assert_eq!(wd.as_bytes().len(), 2);
        wd.feed("abc");
        assert_eq!(wd.as_bytes().len(), 5);
    }

    #[test]
    fn feed_multiple_types_concatenates() {
        let mut wd = WorkData::new();
        wd.feed(&1u8);
        wd.feed(&2u16);
        wd.feed(&3u32);
        wd.feed(&4u64);
        assert_eq!(wd.as_bytes().len(), 1 + 2 + 4 + 8);
    }

    #[test]
    fn feed_order_matters() {
        let mut wd1 = WorkData::new();
        wd1.feed(&1u8);
        wd1.feed(&2u8);

        let mut wd2 = WorkData::new();
        wd2.feed(&2u8);
        wd2.feed(&1u8);

        assert_ne!(wd1.as_bytes(), wd2.as_bytes());
    }

    // ── Empty / zero-length feeds ──────────────────────────────────────

    #[test]
    fn feed_empty_string_stays_empty() {
        let mut wd = WorkData::new();
        wd.feed("");
        assert!(wd.is_empty());
    }

    #[test]
    fn feed_empty_slice_stays_empty() {
        let mut wd = WorkData::new();
        let empty: &[u8] = &[];
        wd.feed(empty);
        assert!(wd.is_empty());
    }

    #[test]
    fn feed_empty_vec_stays_empty() {
        let mut wd = WorkData::new();
        let empty: Vec<u8> = vec![];
        wd.feed(&empty);
        assert!(wd.is_empty());
    }

    #[test]
    fn feed_zero_length_array() {
        let mut wd = WorkData::new();
        wd.feed(&[0u8; 0]);
        assert!(wd.is_empty());
    }

    // ── blend_into ─────────────────────────────────────────────────────

    #[test]
    fn blend_into_empty_is_noop() {
        let wd = WorkData::new();
        let mut buf = [1u8, 2, 3, 4];
        wd.blend_into(&mut buf);
        assert_eq!(buf, [1, 2, 3, 4]);
    }

    #[test]
    fn blend_into_xors_single_byte() {
        let mut wd = WorkData::new();
        wd.feed(&[0xFF]);
        let mut buf = [0x0F, 0xF0, 0xAA, 0x55];
        wd.blend_into(&mut buf);
        assert_eq!(buf, [0x0F ^ 0xFF, 0xF0 ^ 0xFF, 0xAA ^ 0xFF, 0x55 ^ 0xFF]);
    }

    #[test]
    fn blend_into_xors_cyclically() {
        let mut wd = WorkData::new();
        wd.feed(&[0xFF, 0x00]);
        let mut buf = [0x0F, 0xF0, 0x0F, 0xF0];
        wd.blend_into(&mut buf);
        assert_eq!(buf, [0xF0, 0xF0, 0xF0, 0xF0]);
    }

    #[test]
    fn blend_into_larger_data_than_buf() {
        let mut wd = WorkData::new();
        wd.feed(&[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
        let mut buf = [0x00, 0x00, 0x00];
        wd.blend_into(&mut buf);
        assert_eq!(buf, [0x11, 0x22, 0x33]);
    }

    #[test]
    fn blend_into_zero_key_is_noop() {
        let mut wd = WorkData::new();
        wd.feed(&[0x00, 0x00, 0x00, 0x00]);
        let mut buf = [0xAB, 0xCD, 0xEF, 0x01];
        wd.blend_into(&mut buf);
        assert_eq!(buf, [0xAB, 0xCD, 0xEF, 0x01]);
    }

    #[test]
    fn blend_into_double_xor_restores_original() {
        let mut wd = WorkData::new();
        wd.feed(&[0xDE, 0xAD, 0xBE, 0xEF]);
        let original = [0x01, 0x02, 0x03, 0x04];
        let mut buf = original;
        wd.blend_into(&mut buf);
        assert_ne!(buf, original);
        wd.blend_into(&mut buf);
        assert_eq!(buf, original);
    }

    #[test]
    fn blend_into_empty_buf() {
        let mut wd = WorkData::new();
        wd.feed(&42u32);
        let mut buf: [u8; 0] = [];
        wd.blend_into(&mut buf);
    }

    // ── blend_seed ─────────────────────────────────────────────────────

    #[test]
    fn blend_seed_empty_is_zero() {
        let wd = WorkData::new();
        assert_eq!(wd.blend_seed(), 0);
    }

    #[test]
    fn blend_seed_deterministic() {
        let mut wd1 = WorkData::new();
        wd1.feed(&42u32);
        let mut wd2 = WorkData::new();
        wd2.feed(&42u32);
        assert_eq!(wd1.blend_seed(), wd2.blend_seed());
    }

    #[test]
    fn blend_seed_differs_for_different_data() {
        let mut wd1 = WorkData::new();
        wd1.feed(&42u32);
        let mut wd2 = WorkData::new();
        wd2.feed(&43u32);
        assert_ne!(wd1.blend_seed(), wd2.blend_seed());
    }

    #[test]
    fn blend_seed_nonzero_for_nonzero_data() {
        let mut wd = WorkData::new();
        wd.feed(&0xDEADBEEFu32);
        assert_ne!(wd.blend_seed(), 0);
    }

    #[test]
    fn blend_seed_single_byte() {
        let mut wd = WorkData::new();
        wd.feed(&0xFFu8);
        assert_ne!(wd.blend_seed(), 0);
    }

    // ── derive_usize ───────────────────────────────────────────────────

    #[test]
    fn derive_usize_empty_is_zero() {
        let wd = WorkData::new();
        assert_eq!(wd.derive_usize(0), 0);
        assert_eq!(wd.derive_usize(1), 0);
        assert_eq!(wd.derive_usize(999), 0);
    }

    #[test]
    fn derive_usize_deterministic() {
        let mut wd = WorkData::new();
        wd.feed(&0xDEADBEEFu64);
        let a = wd.derive_usize(0);
        let b = wd.derive_usize(0);
        assert_eq!(a, b);
    }

    #[test]
    fn derive_usize_varies_by_index() {
        let mut wd = WorkData::new();
        wd.feed(&[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let a = wd.derive_usize(0);
        let b = wd.derive_usize(1);
        assert_ne!(a, b);
    }

    #[test]
    fn derive_usize_works_with_small_data() {
        let mut wd = WorkData::new();
        wd.feed(&1u8);
        let _ = wd.derive_usize(0);
        let _ = wd.derive_usize(100);
    }

    #[test]
    fn derive_usize_large_index_no_panic() {
        let mut wd = WorkData::new();
        wd.feed(&42u32);
        let _ = wd.derive_usize(usize::MAX);
        let _ = wd.derive_usize(usize::MAX / 2);
    }

    // ── Clone independence ─────────────────────────────────────────────

    #[test]
    fn clone_is_independent() {
        let mut wd = WorkData::new();
        wd.feed(&42u32);
        let mut cloned = wd.clone();
        cloned.feed(&99u32);
        assert_eq!(wd.as_bytes().len(), 4);
        assert_eq!(cloned.as_bytes().len(), 8);
    }

    #[test]
    fn clone_has_same_data() {
        let mut wd = WorkData::new();
        wd.feed(&42u32);
        wd.feed("hello");
        let cloned = wd.clone();
        assert_eq!(wd.as_bytes(), cloned.as_bytes());
        assert_eq!(wd.blend_seed(), cloned.blend_seed());
    }
}
