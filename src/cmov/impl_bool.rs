use super::*;

impl CMov for bool{
    fn cnd_select(cond: bool, a: &Self, b: &Self) -> Self {
        let mut res = *a;
        cmov_bool(cond, &mut res, b);
        res
    }

    fn cmov(&mut self, cond: bool, src: &Self) {
        cmov_bool(cond, self, src);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmov_bool() {
        let mut a: bool = true;
        a.cmov(true, &false);
        assert_eq!(a, false);
        a.cmov(false, &true);
        assert_eq!(a, false);
        a.cmov(true, &true);
        assert_eq!(a, true);
        a.cmov(false, &false);
        assert_eq!(a, true);
    }
    
}