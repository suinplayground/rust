// #[macro_export]
// macro_rules! hoge {
//     ($($arg:tt)*) => {{
//         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//         res
//     }}
// }

macro_rules! alphanumeric_only {
    ($value:expr) => {{
        assert!(
            $value.chars().all(|c| c.is_ascii_alphanumeric()),
            "Only alphanumeric characters are allowed!"
        );
        $value
    }};
}

fn main() {
    dbg!(alphanumeric_only!("hoge"));
    alphanumeric_only!("hoge2122");
    alphanumeric_only!("aaa---");
}
