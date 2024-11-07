// Macro to create a bitmask
#[macro_export]
macro_rules! bitmask {
    ( $start_bit:expr,$width:expr ) => {{
        ((1 << $width) - 1) << $start_bit
    }};
    ( $end_bit:expr;$start_bit:expr ) => {
        bitmask!($start_bit, $end_bit - $start_bit + 1)
    };
    ( $width:expr ) => {
        bitmask!(0, $width)
    };
}

// Macro to extract bits from a value
#[macro_export]
macro_rules! bits {
    ( $val:expr,$start_bit:expr,$width:expr ) => {{
        ($val >> $start_bit) & ((1 << $width) - 1)
    }};
    ( $val:expr,$end_bit:expr;$start_bit:expr ) => {
        bits!($val, $start_bit, $end_bit - $start_bit + 1)
    };
    ( $val:expr,$bit:expr ) => {
        bits!($val, $bit, 1)
    };
}

/// helper macro to include test files
#[macro_export]
macro_rules! include_test_file {
    ($file_name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test-files/",
            $file_name
        ))
    };
}
