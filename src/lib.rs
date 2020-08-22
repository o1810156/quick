fn mid_index<T: Ord>(array: &[T], a: usize, b: usize, c: usize) -> usize {
    let av = array.get(a);
    let bv = array.get(b);
    let cv = array.get(c);

    if av < bv {
        if cv < av {
            a
        } else {
            if cv < bv {
                c
            } else {
                b
            }
        }
    } else {
        if cv < bv {
            b
        } else {
            if cv < av {
                c
            } else {
                a
            }
        }
    }
}

pub fn quick_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    if len < 2 { return; }

    let m = mid_index(array, 0, len / 2, len - 1);
    array.swap(m, len - 1);

    let mut i = 0;
    let mut j = len - 2;

    loop {
        {
            let p = array.get(len-1);
            loop {
                if array.get(i) >= p { break; }
                i += 1;
            }
            loop {
                if j == 0 || array.get(j) < p { break; }
                j -= 1;
            }
        }

        if j <= i { break; }
        array.swap(i, j);
    }

    array.swap(i, len-1);

    quick_sort(&mut array[..i]);
    quick_sort(&mut array[i+1..]);
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn test1() {
        let mut a = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut a);
        assert_eq!(a, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test2() {
        let mut b = [
            "Rust",
            "Python",
            "Ruby",
            "Ada",
            "Fortran",
            "Go",
            "C",
            "C++",
            "C#",
            "JavaScript",
            "R",
            "Java",
            "BASIC",
            "Lisp",
            "Haskell",
            "Erlang",
            "Assembly",
            "F#",
            "Nim",
            "Kotlin",
            "Elisp",
            "Perl",
            "Pascal",
            "PHP",
            "Scala",
            "Swift",
            "Dart",
            "D",
        ];
        quick_sort(&mut b);
        assert_eq!(b, ["Ada", "Assembly", "BASIC", "C", "C#", "C++", "D", "Dart", "Elisp", "Erlang", "F#", "Fortran", "Go", "Haskell", "Java", "JavaScript", "Kotlin", "Lisp", "Nim", "PHP", "Pascal", "Perl", "Python", "R", "Ruby", "Rust", "Scala", "Swift"]);
    }
}
