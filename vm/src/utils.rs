#[macro_export]
macro_rules! slice_to_array {
    ($slice:expr, $type:ty, $n:expr) => {
        {
            let slice = $slice;
            if slice.len() < $n {
                None
            }
            else {
                let array: [$type; $n] = slice[..$n].try_into().unwrap();
                Some(array)
            }
        }
    }
}

