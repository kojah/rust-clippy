//@no-rustfix: overlapping suggestions
#![warn(clippy::needless_for_each)]
#![expect(clippy::needless_return)]

fn main() {
    let v: Vec<i32> = Vec::new();
    // This is unfixable because the closure includes `return`.
    v.iter().for_each(|v| {
        //~^ needless_for_each

        if *v == 10 {
            return;
        } else {
            println!("{v}");
        }
    });
}

fn issue9912() {
    let mut i = 0;
    // Changing this to a `for` loop would break type inference
    [].iter().for_each(move |_: &i32| {
        //~^ needless_for_each
        i += 1;
    });

    // Changing this would actually be okay, but we still suggest `MaybeIncorrect`ly
    [1i32].iter().for_each(move |_: &i32| {
        //~^ needless_for_each
        i += 1;
    });
}

fn issue17856() {
    use std::collections::VecDeque;

    // `VecDeque`'s iterators specialize `fold`/`try_fold` (two-slice iteration),
    // so `for_each` is faster than the `for` loop this lint would suggest: do not lint.
    let v: VecDeque<i32> = VecDeque::new();
    v.iter().for_each(|x| {
        println!("{x}");
    });
    v.into_iter().for_each(|x| {
        println!("{x}");
    });

    let mut v: VecDeque<i32> = VecDeque::new();
    v.iter_mut().for_each(|x| {
        *x += 1;
    });
}
