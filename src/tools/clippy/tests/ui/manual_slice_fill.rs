#![warn(clippy::manual_slice_fill)]
#![allow(clippy::needless_range_loop, clippy::useless_vec)]

macro_rules! assign_element {
    ($slice:ident, $index:expr) => {
        $slice[$index] = 0;
    };
}

macro_rules! assign_element_2 {
    ($i:expr) => {
        $i = 0;
    };
}

struct NoClone;

fn num() -> usize {
    5
}

fn should_lint() {
    let mut some_slice = [1, 2, 3, 4, 5];

    for i in 0..some_slice.len() {
        //~^ manual_slice_fill
        some_slice[i] = 0;
    }

    let x = 5;
    for i in 0..some_slice.len() {
        //~^ manual_slice_fill
        some_slice[i] = x;
    }

    for i in &mut some_slice {
        //~^ manual_slice_fill
        *i = 0;
    }

    // This should trigger `manual_slice_fill`, but the applicability is `MaybeIncorrect` since comments
    // within the loop might be purely informational.
    for i in 0..some_slice.len() {
        //~^ manual_slice_fill
        some_slice[i] = 0;
        // foo
    }
}

fn should_not_lint() {
    let mut some_slice = [1, 2, 3, 4, 5];

    // Should not lint because we can't determine if the scope of the loop is intended to access all the
    // elements of the slice.
    for i in 0..5 {
        some_slice[i] = 0;
    }

    // Should not lint, as using a function to assign values to elements might be
    // intentional. For example, the contents of `num()` could be temporary and subject to change
    // later.
    for i in 0..some_slice.len() {
        some_slice[i] = num();
    }

    // Should not lint because this loop isn't equivalent to `fill`.
    for i in 0..some_slice.len() {
        some_slice[i] = 0;
        println!("foo");
    }

    // Should not lint because it may be intentional to use a macro to perform an operation equivalent
    // to `fill`.
    for i in 0..some_slice.len() {
        assign_element!(some_slice, i);
    }

    let another_slice = [1, 2, 3];
    // Should not lint because the range is not for `some_slice`.
    for i in 0..another_slice.len() {
        some_slice[i] = 0;
    }

    let mut vec: Vec<Option<NoClone>> = Vec::with_capacity(5);
    // Should not lint because `NoClone` does not have `Clone` trait.
    for i in 0..vec.len() {
        vec[i] = None;
    }

    // Should not lint, as using a function to assign values to elements might be
    // intentional. For example, the contents of `num()` could be temporary and subject to change
    // later.
    for i in &mut some_slice {
        *i = num();
    }

    // Should not lint because this loop isn't equivalent to `fill`.
    for i in &mut some_slice {
        *i = 0;
        println!("foo");
    }

    // Should not lint because it may be intentional to use a macro to perform an operation equivalent
    // to `fill`.
    for i in &mut some_slice {
        assign_element_2!(*i);
    }

    let mut vec: Vec<Option<NoClone>> = Vec::with_capacity(5);
    // Should not lint because `NoClone` does not have `Clone` trait.
    for i in &mut vec {
        *i = None;
    }
}

fn issue_14192() {
    let mut tmp = vec![0; 3];

    for i in 0..tmp.len() {
        tmp[i] = i;
    }

    for i in 0..tmp.len() {
        tmp[i] = 2 + i;
    }

    for i in 0..tmp.len() {
        tmp[0] = i;
    }
}

fn issue14189() {
    // Should not lint because `!*b` is not constant
    let mut tmp = vec![1, 2, 3];
    for b in &mut tmp {
        *b = !*b;
    }
}

mod issue14685 {
    use std::ops::{Index, IndexMut};

    #[derive(Clone)]
    struct ZipList<T>(T);

    impl<T> ZipList<T> {
        fn len(&self) -> usize {
            todo!()
        }

        fn is_empty(&self) -> bool {
            todo!()
        }
    }

    impl<T> Index<usize> for ZipList<T> {
        type Output = T;

        fn index(&self, _: usize) -> &Self::Output {
            todo!()
        }
    }

    impl<T> IndexMut<usize> for ZipList<T> {
        fn index_mut(&mut self, _: usize) -> &mut Self::Output {
            todo!()
        }
    }

    fn index_mut(mut zl: ZipList<usize>) {
        for i in 0..zl.len() {
            zl[i] = 6;
        }
    }
}
