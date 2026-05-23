#![feature(stdsimd)]
use core::arch::asm;
use core::arch::x86_64::*;
use core::ffi::c_void;
mod impl_bool;
mod impl_integer;
pub trait CMov: Clone {
    // Returns the value of `a` if `cond` is false, otherwise, returns the value of `b`.
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self;

    // If `cond` is true, copies the value of `src` into `self`.
    fn cmov(&mut self, cond: bool, src: &Self);
    
    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        self.cmov(choice, other);
    }
}

#[inline(always)]
fn cswp_u64(cond: bool, val1: &mut u64, val2: &mut u64) {
    let cond = cond as u64;
    unsafe {
        asm!(
            "test {0}, {0}",
            "mov r9, {1}",
            "cmovnz {1}, {2}",
            "cmovnz {2}, r9",
            in(reg) cond,
            inout(reg) *val1,
            inout(reg) *val2,
            options(nostack, nomem),
        );
    }
}

#[inline(always)]
fn cmov_u64(cond: bool, val1: *mut u64, val2: *const u64) {
    let cond = cond as u64;
    unsafe {
        asm!(
            "test {0}, {0}",
            "cmovnz {1}, {2}",
            in(reg) cond,
            inout(reg) *val1,
            in(reg) *val2,
        );
    }
}


#[inline(always)]
fn cmov_u32(cond: bool, val1: *mut u32, val2: *const u32) {
    let cond = cond as u64;
    unsafe {
        asm!(
            "test {0}, {0}",
            "cmovnz {1:e}, {2:e}",
            in(reg) cond,
            inout(reg) *val1,
            in(reg) *val2,
        );
    }
}

#[inline(always)]
fn cmov_u8(cond: bool, val1: *mut u8, val2: *const u8) {
    unsafe {
        let mut r1: u32 =(0 | *val1) as u32;
        let r2: u32 =(0 | *val2) as u32;
        cmov_u32(cond, &mut r1, &r2);
        *val1 = (r1 & 0xff) as u8;
    }
}

#[inline(always)]
fn cmov_u16(cond: bool, val1: *mut u16, val2: *const u16) {
    unsafe {
        let mut r1: u32 =(0 | *val1) as u32;
        let r2: u32 =(0 | *val2) as u32;
        cmov_u32(cond, &mut r1, &r2);
        *val1 = (r1 & 0xffff) as u16;
    }
}

#[inline(always)]
fn cmov_bool(cond: bool, val1: *mut bool, val2: *const bool) {
    unsafe {
        let mut r1: u32 = *val1 as u32;
        cmov_u32(cond, &mut r1, &(*val2 as u32));
        *val1 = r1 != 0;
    }
}

#[inline(always)]
pub fn cnd_swap_copy<T: Copy + CMov> (cond: bool, val1: &mut T, val2: &mut T) {
    let c = *val1;
    val1.cmov(cond, val2);
    val2.cmov(cond, &c);
}

#[inline(always)]
pub fn oblivious_conditonal_swap<T> (cond: bool, val1: &mut T, val2: &mut T) {

    let mut curr1 = val1 as *mut T as *mut __m512i;
    let mut curr2 = val2 as *mut T as *mut __m512i;

    let size = core::mem::size_of::<T>();

    for _ in 0..size / 64 {
        unsafe {
            cxchg_internal::<64>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            curr1 = curr1.add(1);
            curr2 = curr2.add(1);
        }
    }

    let rem_size = size % 64;
    if rem_size > 0 {
        match rem_size {
            63 => unsafe {
                cxchg_internal::<63>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            62 => unsafe {
                cxchg_internal::<62>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            61 => unsafe {
                cxchg_internal::<61>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            60 => unsafe {
                cxchg_internal::<60>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            59 => unsafe {
                cxchg_internal::<59>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            58 => unsafe {
                cxchg_internal::<58>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            57 => unsafe {
                cxchg_internal::<57>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            56 => unsafe {
                cxchg_internal::<56>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            55 => unsafe {
                cxchg_internal::<55>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            54 => unsafe {
                cxchg_internal::<54>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            53 => unsafe {
                cxchg_internal::<53>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            52 => unsafe {
                cxchg_internal::<52>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            51 => unsafe {
                cxchg_internal::<51>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            50 => unsafe {
                cxchg_internal::<50>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            49 => unsafe {
                cxchg_internal::<49>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            48 => unsafe {
                cxchg_internal::<48>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            47 => unsafe {
                cxchg_internal::<47>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            46 => unsafe {
                cxchg_internal::<46>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            45 => unsafe {
                cxchg_internal::<45>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            44 => unsafe {
                cxchg_internal::<44>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            43 => unsafe {
                cxchg_internal::<43>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            42 => unsafe {
                cxchg_internal::<42>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            41 => unsafe {
                cxchg_internal::<41>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            40 => unsafe {
                cxchg_internal::<40>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            39 => unsafe {
                cxchg_internal::<39>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            38 => unsafe {
                cxchg_internal::<38>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            37 => unsafe {
                cxchg_internal::<37>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            36 => unsafe {
                cxchg_internal::<36>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            35 => unsafe {
                cxchg_internal::<35>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            34 => unsafe {
                cxchg_internal::<34>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            33 => unsafe {
                cxchg_internal::<33>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            32 => unsafe {
                cxchg_internal::<32>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            31 => unsafe {
                cxchg_internal::<31>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            30 => unsafe {
                cxchg_internal::<30>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            29 => unsafe {
                cxchg_internal::<29>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            28 => unsafe {
                cxchg_internal::<28>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            27 => unsafe {
                cxchg_internal::<27>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            26 => unsafe {
                cxchg_internal::<26>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            25 => unsafe {
                cxchg_internal::<25>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            24 => unsafe {
                cxchg_internal::<24>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            23 => unsafe {
                cxchg_internal::<23>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            22 => unsafe {
                cxchg_internal::<22>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            21 => unsafe {
                cxchg_internal::<21>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            20 => unsafe {
                cxchg_internal::<20>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            19 => unsafe {
                cxchg_internal::<19>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            18 => unsafe {
                cxchg_internal::<18>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            17 => unsafe {
                cxchg_internal::<17>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            16 => unsafe {
                cxchg_internal::<16>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            15 => unsafe {
                cxchg_internal::<15>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            14 => unsafe {
                cxchg_internal::<14>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            13 => unsafe {
                cxchg_internal::<13>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            12 => unsafe {
                cxchg_internal::<12>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            11 => unsafe {
                cxchg_internal::<11>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            10 => unsafe {
                cxchg_internal::<10>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            9 => unsafe {
                cxchg_internal::<9>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            8 => unsafe {
                cxchg_internal::<8>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            7 => unsafe {
                cxchg_internal::<7>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            6 => unsafe {
                cxchg_internal::<6>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            5 => unsafe {
                cxchg_internal::<5>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            4 => unsafe {
                cxchg_internal::<4>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            3 => unsafe {
                cxchg_internal::<3>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            2 => unsafe {
                cxchg_internal::<2>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            1 => unsafe {
                cxchg_internal::<1>(cond, curr1 as *mut c_void, curr2 as *mut c_void);
            },
            _ => {
                panic!("Invalid size, please add a new implementation for size = {} in this function", rem_size);
            }
        }
    }
}

unsafe fn cxchg_internal<const SZ: usize>(cond: bool, vec1: *mut c_void, vec2: *mut c_void) {
    assert!(SZ <= 64, "SZ must be less than or equal to 64");
    #[cfg(target_feature = "avx512vl")] 
    let blend_mask:__mmask8 = (!cond as __mmask8).wrapping_sub(1);
    if SZ == 64 {
        #[cfg(all(not(feature = "force_avx512_off"), target_feature = "avx512vl"))] 
        {
            let mut vec1_temp: __m512i = _mm512_set1_epi32(0);
            let mut vec2_temp: __m512i = _mm512_set1_epi32(0);
            let mut temp: __m512i;
            core::ptr::copy_nonoverlapping(vec1, &mut vec1_temp as *mut __m512i as *mut c_void, 64);
            core::ptr::copy_nonoverlapping(vec2, &mut vec2_temp as *mut __m512i as *mut c_void, 64);

            let musk: __m512i = _mm512_set1_epi32(-(cond as i32));
            temp = _mm512_xor_si512(vec1_temp, vec2_temp);
            temp = _mm512_and_si512(temp, musk);

            vec1_temp = _mm512_xor_si512(vec1_temp, temp);
            vec2_temp = _mm512_xor_si512(vec2_temp, temp);

            core::ptr::copy_nonoverlapping(&vec1_temp as *const __m512i as *const c_void, vec1, 64);
            core::ptr::copy_nonoverlapping(&vec2_temp as *const __m512i as *const c_void, vec2, 64);
        }
        #[cfg(any(feature = "force_avx512_off", not(target_feature = "avx512vl")))] {
            cxchg_internal::<32>(cond, vec1, vec2);
            cxchg_internal::<32>(cond, (vec1 as *mut u8).add(32) as *mut c_void, (vec2 as *mut u8).add(32) as *mut c_void);
        }

        return;
    }

    if SZ >= 32 {
        #[cfg(target_feature = "avx512vl")] 
        {
            let mut vec1_temp: __m256d = _mm256_set1_pd(0.0);
            let mut vec2_temp: __m256d = _mm256_set1_pd(0.0);

            core::ptr::copy_nonoverlapping(vec1, &mut vec1_temp as *mut __m256d as *mut c_void, 32);
            core::ptr::copy_nonoverlapping(vec2, &mut vec2_temp as *mut __m256d as *mut c_void, 32);

            let vec1_after_swap = _mm256_mask_blend_pd(blend_mask, vec1_temp, vec2_temp);
            let vec2_after_swap = _mm256_mask_blend_pd(blend_mask, vec2_temp, vec1_temp);

            core::ptr::copy_nonoverlapping(&vec1_after_swap as *const __m256d as *const c_void, vec1, 32);
            core::ptr::copy_nonoverlapping(&vec2_after_swap as *const __m256d as *const c_void, vec2, 32);
        }
        #[cfg(not(target_feature = "avx512vl"))]
        {
            let mut vec1_temp: __m256i = _mm256_set1_epi32(0);
            let mut vec2_temp: __m256i = _mm256_set1_epi32(0);
            let mut temp: __m256i;


            core::ptr::copy_nonoverlapping(vec1, &mut vec1_temp as *mut __m256i as *mut c_void, 32);
            core::ptr::copy_nonoverlapping(vec2, &mut vec2_temp as *mut __m256i as *mut c_void, 32);

            let mask = _mm256_set1_epi32(-(cond as i32));
            temp = _mm256_xor_si256(vec1_temp, vec2_temp);
            temp = _mm256_and_si256(temp, mask);

            vec1_temp = _mm256_xor_si256(vec1_temp, temp);
            vec2_temp = _mm256_xor_si256(vec2_temp, temp);

            core::ptr::copy_nonoverlapping(&vec1_temp as *const __m256i as *const c_void, vec1, 32);
            core::ptr::copy_nonoverlapping(&vec2_temp as *const __m256i as *const c_void, vec2, 32);
        }

        // #[cfg(not(any(target_feature = "avx512vl", target_feature = "avx2")))]
        // {
        //     cxchg_internal::<16>(cond, vec1, vec2);
        //     cxchg_internal::<16>(cond, (vec1 as *mut u8).add(16) as *mut c_void, (vec1 as *mut u8).add(16) as *mut c_void);
        // }
    }

    if SZ % 32 >= 16 {
        let offset = 4 * (SZ / 32);
        #[cfg(target_feature = "avx512vl")] 
        {
            let mut vec1_temp: __m128d = _mm_set1_pd(0.0);
            let mut vec2_temp: __m128d = _mm_set1_pd(0.0);

            core::ptr::copy_nonoverlapping((vec1 as *mut u64).add(offset) as *mut c_void, &mut vec1_temp as *mut __m128d as *mut c_void, 16);
            core::ptr::copy_nonoverlapping((vec2 as *mut u64).add(offset) as *mut c_void, &mut vec2_temp as *mut __m128d as *mut c_void, 16);

            let vec1_after_swap = _mm_mask_blend_pd(blend_mask, vec1_temp, vec2_temp);
            let vec2_after_swap = _mm_mask_blend_pd(blend_mask, vec2_temp, vec1_temp);

            core::ptr::copy_nonoverlapping(&vec1_after_swap as *const __m128d as *const c_void, (vec1 as *mut u64).add(offset) as *mut c_void, 16);
            core::ptr::copy_nonoverlapping(&vec2_after_swap as *const __m128d as *const c_void, (vec2 as *mut u64).add(offset) as *mut c_void, 16);
        }

        #[cfg(all(not(target_feature = "avx512vl"), target_feature = "sse2"))]
        {
            let mut vec1_temp: __m128i = _mm_set1_epi32(0);
            let mut vec2_temp: __m128i = _mm_set1_epi32(0);
            let mut temp: __m128i;

            core::ptr::copy_nonoverlapping((vec1 as *mut u64).add(offset) as *mut c_void, &mut vec1_temp as *mut __m128i as *mut c_void, 16);
            core::ptr::copy_nonoverlapping((vec2 as *mut u64).add(offset) as *mut c_void, &mut vec2_temp as *mut __m128i as *mut c_void, 16);

            let mask = _mm_set1_epi16(-(cond as i16));
            temp = _mm_xor_si128(vec1_temp, vec2_temp);
            temp = _mm_and_si128(temp, mask);

            vec1_temp = _mm_xor_si128(vec1_temp, temp);
            vec2_temp = _mm_xor_si128(vec2_temp, temp);

            core::ptr::copy_nonoverlapping(&vec1_temp as *const __m128i as *const c_void, (vec1 as *mut u64).add(offset) as *mut c_void, 16);
            core::ptr::copy_nonoverlapping(&vec2_temp as *const __m128i as *const c_void, (vec2 as *mut u64).add(offset) as *mut c_void, 16);
        }
        #[cfg(not(any(target_feature = "avx512vl", target_feature = "sse2")))]
        {
            cxchg_internal::<8>(cond, (vec1 as *mut u64).add(offset) as *mut c_void, (vec2 as *mut u64).add(offset) as *mut c_void);
            cxchg_internal::<8>(cond, (vec1 as *mut u8).add(8 * offset + 8) as *mut c_void, (vec2 as *mut u8).add(8 * offset + 8) as *mut c_void);
        }
    }

    if SZ % 16 >= 8 {
        let offset = 2 * (SZ / 16);
        let curr1_64 = (vec1 as *mut u64).add(offset);
        let curr2_64 = (vec2 as *mut u64).add(offset);
        cnd_swap_copy(cond, &mut *curr1_64, &mut *curr2_64)
    }

    if SZ % 8 >= 4 {
        let offset = 2 * (SZ / 8) as usize;
        let curr1_32 = (vec1 as *mut u32).add(offset);
        let curr2_32 = (vec2 as *mut u32).add(offset );
        cnd_swap_copy(cond, &mut *curr1_32, &mut *curr2_32)
    }

    if SZ % 4 >= 2 {
        let offset = 2 * (SZ / 4) as usize;
        let curr1_16 = (vec1 as *mut u16).add(offset);
        let curr2_16 = (vec2 as *mut u16).add(offset);
        cnd_swap_copy(cond, &mut *curr1_16, &mut *curr2_16)
    }

    if SZ % 2 >= 1 {
        let offset = 2 * (SZ / 2) as usize;
        let curr1_8 = (vec1 as *mut u8).add(offset);
        let curr2_8 = (vec2 as *mut u8).add(offset);
        cnd_swap_copy(cond, &mut *curr1_8, &mut *curr2_8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cswp_u64() {
        let mut a: u64 = 1;
        let mut b: u64 = 2;

        cswp_u64(true, &mut a, &mut b);
        assert!(a == 2 && b == 1);

        cswp_u64(false, &mut a, &mut b);
        assert!(a == 2 && b == 1);
        println!("a: {}, b: {}", a, b);
    }

    #[test]
    fn test_cmov_u64() {
        let mut a: u64 = 1;
        let b: u64 = 2;

        a.cmov(true, &b);
        assert!(a == 2 && b == 2);

        a.cmov(false, &1);
        assert!(a == 2);
        println!("a: {}, b: {}", a, b);
    }

    #[test]
    fn test_cmov_u32() {
        let mut a: u32 = 1;
        let b: u32 = 2;

        a.cmov(true, &b);
        assert!(a == 2 && b == 2);

        a.cmov(false, &1);
        assert!(a == 2);
        println!("a: {}, b: {}", a, b);
    }

    #[test]
    fn test_obli_swap_vec() {
        let mut a: Vec<f64> = vec![0.0;256];
        let mut b: Vec<f64> = vec![1.0;256];

        oblivious_conditonal_swap(true, &mut a, &mut b);
        oblivious_conditonal_swap(false, &mut a, &mut b);

        println!("a: {:?}", a);
        println!();
        println!("b: {:?}", b);
    }

    #[test]
    fn test_obli_swap_tuple() {
        let a: Vec<f64> = vec![0.0;256];
        let b: Vec<f64> = vec![0.0;256];
        let mut c = (a, b);

        let a: Vec<f64> = vec![1.0;256];
        let b: Vec<f64> = vec![1.0;256];
        let mut d = (a, b);

        oblivious_conditonal_swap(true, &mut c, &mut d);

        println!("c: {:?}", c);
        println!();
        println!("d: {:?}", d);

        oblivious_conditonal_swap(false, &mut c, &mut d);

        println!("c: {:?}", c);
        println!();
        println!("d: {:?}", d);
    }

    #[test]
    fn test_struct_swap() {
        #[derive(Clone, Copy, Debug)]
        struct Test {
            a: u64,
            b: u64,
            c: u64,
            d: u64,
            e: u64,
            // f: u64,
            // g: u64,
            // h: u64,
        }

        let mut a = Test {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
            // f: 6,
            // g: 7,
            // h: 8,
        };

        let mut b = Test {
            a: 100,
            b: 101,
            c: 102,
            d: 103,
            e: 104,
            // f: 105,
            // g: 106,
            // h: 107,
        };

        oblivious_conditonal_swap(true, &mut a, &mut b);

        println!("a: {:?}", a);
        println!();
        println!("b: {:?}", b);

        oblivious_conditonal_swap(false, &mut a, &mut b);

        println!("a: {:?}", a);
        println!();
        println!("b: {:?}", b);
    }
}