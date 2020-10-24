//https://github.com/JaniM/variant-go-server/blob/4f7b8206f605887a1d0e6bb5a10b6d4ae895e4dd/client/src/utils.rs#L30
#[macro_use]
macro_rules! if_html {
    (let $pat:pat = $cond:expr => $($body:tt)+) => {
        if let $pat = $cond {
            html!($($body)+)
        } else {
            html!()
        }
    };
    ($cond:expr => $($body:tt)+) => {
        if $cond {
            html!($($body)+)
        } else {
            html!()
        }
    };
}
