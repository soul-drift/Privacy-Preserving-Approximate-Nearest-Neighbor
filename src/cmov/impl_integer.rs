use super::*;
impl CMov for u8 {
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a;
        cmov_u8(cond, &mut res, b);
        res
    }

    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u8(cond, self, src)
    }
}

impl CMov for u16 {
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a;
        cmov_u16(cond, &mut res, b);
        res
    }

    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u16(cond, self, src)
    }
}

impl CMov for u32 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a;
        cmov_u32(cond, &mut res, b);
        res
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u32(cond, self, src);
    }
}

impl CMov for u64 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a;
        cmov_u64(cond, &mut res, b);
        res
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u64(cond, self, src);
    }
}

impl CMov for usize {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u64;
        cmov_u64(cond, &mut res, &(*b as u64));
        res as usize
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u64(cond, self as *mut usize as *mut u64, &(*src as u64));
    }
}

impl CMov for i8 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u8;
        cmov_u8(cond, &mut res, &(*b as u8));
        res as i8
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u8(cond, self as *mut i8 as *mut u8, &(*src as u8));
    }
}

impl CMov for i16 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u16;
        cmov_u16(cond, &mut res, &(*b as u16));
        res as i16
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u16(cond, self as *mut i16 as *mut u16, &(*src as u16));
    }
}

impl CMov for i32 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u32;
        cmov_u32(cond, &mut res, &(*b as u32));
        res as i32
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u32(cond, self as *mut i32 as *mut u32, &(*src as u32));
    }
}

impl CMov for i64 {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u64;
        cmov_u64(cond, &mut res, &(*b as u64));
        res as i64
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u64(cond, self as *mut i64 as *mut u64, &(*src as u64));
    }
}

impl CMov for isize {
    #[inline(always)]
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a as u64;
        cmov_u64(cond, &mut res, &(*b as u64));
        res as isize
    }

    #[inline(always)]
    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_u64(cond, self as *mut isize as *mut u64, &(*src as u64));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmov_u64() {
        let mut a: u64 = 0;
        a.cmov(true, &1);
        assert_eq!(a, 1);
        a.cmov(false, &2);
        assert_eq!(a, 1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_uszie() {
        let mut a: usize = 0;
        a.cmov(true, &1);
        assert_eq!(a, 1);
        a.cmov(false, &2);
        assert_eq!(a, 1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_u32() {
        let mut a: u32 = 0;
        a.cmov(true, &1);
        assert_eq!(a, 1);
        a.cmov(false, &2);
        assert_eq!(a, 1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_u16() {
        let mut a: u16 = 0;
        a.cmov(true, &1);
        assert_eq!(a, 1);
        a.cmov(false, &2);
        assert_eq!(a, 1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_u8() {
        let mut a: u8 = 0;
        a.cmov(true, &1);
        assert_eq!(a, 1);
        a.cmov(false, &2);
        assert_eq!(a, 1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_iszie() {
        let mut a: isize = 0;
        a.cmov(true, &-1);
        assert_eq!(a, -1);
        a.cmov(false, &2);
        assert_eq!(a, -1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_i64() {
        let mut a: i64 = 0;
        a.cmov(true, &-1);
        assert_eq!(a, -1);
        a.cmov(false, &2);
        assert_eq!(a, -1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_i32() {
        let mut a: i32 = 0;
        a.cmov(true, &-1);
        assert_eq!(a, -1);
        a.cmov(false, &2);
        assert_eq!(a, -1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_i16() {
        let mut a: i16 = 0;
        a.cmov(true, &-1);
        assert_eq!(a, -1);
        a.cmov(false, &2);
        assert_eq!(a, -1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }

    #[test]
    fn test_cmov_i8() {
        let mut a: i8 = 0;
        a.cmov(true, &-1);
        assert_eq!(a, -1);
        a.cmov(false, &2);
        assert_eq!(a, -1);
        a.cmov(true, &3);
        assert_eq!(a, 3);
        a.cmov(false, &4);
        assert_eq!(a, 3);
    }
}