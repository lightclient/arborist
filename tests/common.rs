#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::<u128, [u8; 32]>::new();
            $(
                m.insert($key, val!($value));
            )+
            m
        }
     };
);

#[macro_export]
macro_rules! val(
    ($v:expr) => {{
        let mut buf = [0u8; 32];
        buf[0] = $v;
        buf
    }};
);
