#[macro_export]
macro_rules! choice {
    ($x:expr, $y:expr, $z:expr) => {
        ($x & $y) ^ ((!$x) & $z)
    };
}

#[macro_export]
macro_rules! median {
    ($x:expr, $y:expr, $z:expr) => {
        ($x & $y) | ($x & $z) | ($y & $z)
    };
}

#[macro_export]
macro_rules! rotate {
    ($x:expr, $n:expr) => {
          (($x) >> (($n) % 64))
    };
}
