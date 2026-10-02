fn bigger(a: i32, b: i32) -> i32 {
    // if a > b { a } else { b }
    a.max(b)
}

fn main() {
    // You can optionally experiment here.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

// This error occurs when an expression was used in a place where the compiler
// expected an expression of a different type. It can occur in several cases, the
// most common being when calling a function and passing an argument which has a
// different type than the matching type in the function declaration.