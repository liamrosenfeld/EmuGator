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

#[test]
fn test_bits() {
    let ten = 0b1010;

    assert_eq!(bits!(ten, 0), 0b0);
    assert_eq!(bits!(ten, 1), 0b1);
    assert_eq!(bits!(ten, 2), 0b0);
    assert_eq!(bits!(ten, 3), 0b1);

    assert_eq!(bits!(ten, 0, 2), 0b10);
    assert_eq!(bits!(ten, 1, 3), 0b101);
    assert_eq!(bits!(ten, 3;1), 0b101);
}

#[test]
fn test_bitmask() {
    assert_eq!(bitmask!(0, 5), 0b11111);
    assert_eq!(bitmask!(10;5), 0b11111100000);
    assert_eq!(bitmask!(5), 0b11111);
}
