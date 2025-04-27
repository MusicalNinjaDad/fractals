pub(super) fn unbounded<T,F,E>(max_iterations: usize, mut f: F, escape: E) -> Option<usize>
where
    F: FnMut(T) -> T,
    E: Fn(T) -> bool,
    T: PartialEq + Default + Copy,
{
    let mut value = T::default();
    for i in 0..=max_iterations {
        value = f(value);
        if escape(value) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbounded_escapes() {
        let escapes = unbounded(100, |x: u32| {x+1}, |x| {x>=5});
        assert_eq!(escapes, Some(4))
    }

    #[test]
    fn unbounded_just_escapes() {
        let escapes = unbounded(4, |x: u32| {x+1}, |x| {x>=5});
        assert_eq!(escapes, Some(4))
    }

    #[test]
    fn unbounded_low_iterations() {
        let escapes = unbounded(3, |x: u32| {x+1}, |x| {x>=5});
        assert_eq!(escapes, None)
    }

    #[test]
    fn unbounded_bounded() {
        let escapes = unbounded(100, |x: f64| {x/2.0}, |x| {x>1.0});
        assert_eq!(escapes, None)
    }
}