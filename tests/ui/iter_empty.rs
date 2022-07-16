// run-rustfix
#![warn(clippy::iter_empty)]
#![allow(clippy::iter_next_slice, clippy::redundant_clone)]

fn array() {
    assert_eq!([].into_iter().next(), Option::<i32>::None);
    assert_eq!([].iter_mut().next(), Option::<&mut i32>::None);
    assert_eq!([].iter().next(), Option::<&i32>::None);
    assert_eq!(None.into_iter().next(), Option::<i32>::None);
    assert_eq!(None.iter_mut().next(), Option::<&mut i32>::None);
    assert_eq!(None.iter().next(), Option::<&i32>::None);

    // Don't trigger on non-iter methods
    let _: Option<String> = None.clone();
    let _: [String; 0] = [].clone();
}

macro_rules! in_macros {
    () => {
        assert_eq!([].into_iter().next(), Option::<i32>::None);
        assert_eq!([].iter_mut().next(), Option::<&mut i32>::None);
        assert_eq!([].iter().next(), Option::<&i32>::None);
        assert_eq!(None.into_iter().next(), Option::<i32>::None);
        assert_eq!(None.iter_mut().next(), Option::<&mut i32>::None);
        assert_eq!(None.iter().next(), Option::<&i32>::None);
    };
}

fn main() {
    array();
    in_macros!();
}