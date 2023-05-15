#[macro_export]
macro_rules! assert_err {
    ($expression:expr) => {
        match $expression {
            Err(()) => (),
            result => panic!("Assertion failed: expected Err(()), but got {:?}", result),
        }
    };
}

#[macro_export]
macro_rules! assert_ok {
    ($expression:expr) => {
        match $expression {
            Ok(()) => (),
            result => panic!("Assertion failed: expected Ok(()), but got {:?}", result),
        }
    };
}