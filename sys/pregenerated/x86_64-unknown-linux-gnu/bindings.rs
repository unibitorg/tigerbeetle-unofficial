pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 36;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of max_align_t"][::std::mem::size_of::<max_align_t>() - 32usize];
    ["Alignment of max_align_t"][::std::mem::align_of::<max_align_t>() - 16usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce1"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce2"]
        [::std::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
};
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
    ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
    ["Offset of field: __fsid_t::__val"][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
};
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type tb_uint128_t = __uint128_t;
pub mod TB_ACCOUNT_FLAGS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_ACCOUNT_LINKED: Type = 1;
    pub const TB_ACCOUNT_DEBITS_MUST_NOT_EXCEED_CREDITS: Type = 2;
    pub const TB_ACCOUNT_CREDITS_MUST_NOT_EXCEED_DEBITS: Type = 4;
    pub const TB_ACCOUNT_HISTORY: Type = 8;
    pub const TB_ACCOUNT_IMPORTED: Type = 16;
    pub const TB_ACCOUNT_CLOSED: Type = 32;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_account_t {
    pub id: tb_uint128_t,
    pub debits_pending: tb_uint128_t,
    pub debits_posted: tb_uint128_t,
    pub credits_pending: tb_uint128_t,
    pub credits_posted: tb_uint128_t,
    pub user_data_128: tb_uint128_t,
    pub user_data_64: u64,
    pub user_data_32: u32,
    pub reserved: u32,
    pub ledger: u32,
    pub code: u16,
    pub flags: u16,
    pub timestamp: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_account_t"][::std::mem::size_of::<tb_account_t>() - 128usize];
    ["Alignment of tb_account_t"][::std::mem::align_of::<tb_account_t>() - 16usize];
    ["Offset of field: tb_account_t::id"][::std::mem::offset_of!(tb_account_t, id) - 0usize];
    ["Offset of field: tb_account_t::debits_pending"]
        [::std::mem::offset_of!(tb_account_t, debits_pending) - 16usize];
    ["Offset of field: tb_account_t::debits_posted"]
        [::std::mem::offset_of!(tb_account_t, debits_posted) - 32usize];
    ["Offset of field: tb_account_t::credits_pending"]
        [::std::mem::offset_of!(tb_account_t, credits_pending) - 48usize];
    ["Offset of field: tb_account_t::credits_posted"]
        [::std::mem::offset_of!(tb_account_t, credits_posted) - 64usize];
    ["Offset of field: tb_account_t::user_data_128"]
        [::std::mem::offset_of!(tb_account_t, user_data_128) - 80usize];
    ["Offset of field: tb_account_t::user_data_64"]
        [::std::mem::offset_of!(tb_account_t, user_data_64) - 96usize];
    ["Offset of field: tb_account_t::user_data_32"]
        [::std::mem::offset_of!(tb_account_t, user_data_32) - 104usize];
    ["Offset of field: tb_account_t::reserved"]
        [::std::mem::offset_of!(tb_account_t, reserved) - 108usize];
    ["Offset of field: tb_account_t::ledger"]
        [::std::mem::offset_of!(tb_account_t, ledger) - 112usize];
    ["Offset of field: tb_account_t::code"][::std::mem::offset_of!(tb_account_t, code) - 116usize];
    ["Offset of field: tb_account_t::flags"]
        [::std::mem::offset_of!(tb_account_t, flags) - 118usize];
    ["Offset of field: tb_account_t::timestamp"]
        [::std::mem::offset_of!(tb_account_t, timestamp) - 120usize];
};
pub mod TB_TRANSFER_FLAGS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_TRANSFER_LINKED: Type = 1;
    pub const TB_TRANSFER_PENDING: Type = 2;
    pub const TB_TRANSFER_POST_PENDING_TRANSFER: Type = 4;
    pub const TB_TRANSFER_VOID_PENDING_TRANSFER: Type = 8;
    pub const TB_TRANSFER_BALANCING_DEBIT: Type = 16;
    pub const TB_TRANSFER_BALANCING_CREDIT: Type = 32;
    pub const TB_TRANSFER_CLOSING_DEBIT: Type = 64;
    pub const TB_TRANSFER_CLOSING_CREDIT: Type = 128;
    pub const TB_TRANSFER_IMPORTED: Type = 256;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_transfer_t {
    pub id: tb_uint128_t,
    pub debit_account_id: tb_uint128_t,
    pub credit_account_id: tb_uint128_t,
    pub amount: tb_uint128_t,
    pub pending_id: tb_uint128_t,
    pub user_data_128: tb_uint128_t,
    pub user_data_64: u64,
    pub user_data_32: u32,
    pub timeout: u32,
    pub ledger: u32,
    pub code: u16,
    pub flags: u16,
    pub timestamp: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_transfer_t"][::std::mem::size_of::<tb_transfer_t>() - 128usize];
    ["Alignment of tb_transfer_t"][::std::mem::align_of::<tb_transfer_t>() - 16usize];
    ["Offset of field: tb_transfer_t::id"][::std::mem::offset_of!(tb_transfer_t, id) - 0usize];
    ["Offset of field: tb_transfer_t::debit_account_id"]
        [::std::mem::offset_of!(tb_transfer_t, debit_account_id) - 16usize];
    ["Offset of field: tb_transfer_t::credit_account_id"]
        [::std::mem::offset_of!(tb_transfer_t, credit_account_id) - 32usize];
    ["Offset of field: tb_transfer_t::amount"]
        [::std::mem::offset_of!(tb_transfer_t, amount) - 48usize];
    ["Offset of field: tb_transfer_t::pending_id"]
        [::std::mem::offset_of!(tb_transfer_t, pending_id) - 64usize];
    ["Offset of field: tb_transfer_t::user_data_128"]
        [::std::mem::offset_of!(tb_transfer_t, user_data_128) - 80usize];
    ["Offset of field: tb_transfer_t::user_data_64"]
        [::std::mem::offset_of!(tb_transfer_t, user_data_64) - 96usize];
    ["Offset of field: tb_transfer_t::user_data_32"]
        [::std::mem::offset_of!(tb_transfer_t, user_data_32) - 104usize];
    ["Offset of field: tb_transfer_t::timeout"]
        [::std::mem::offset_of!(tb_transfer_t, timeout) - 108usize];
    ["Offset of field: tb_transfer_t::ledger"]
        [::std::mem::offset_of!(tb_transfer_t, ledger) - 112usize];
    ["Offset of field: tb_transfer_t::code"]
        [::std::mem::offset_of!(tb_transfer_t, code) - 116usize];
    ["Offset of field: tb_transfer_t::flags"]
        [::std::mem::offset_of!(tb_transfer_t, flags) - 118usize];
    ["Offset of field: tb_transfer_t::timestamp"]
        [::std::mem::offset_of!(tb_transfer_t, timestamp) - 120usize];
};
pub mod TB_CREATE_ACCOUNT_RESULT {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_CREATE_ACCOUNT_OK: Type = 0;
    pub const TB_CREATE_ACCOUNT_LINKED_EVENT_FAILED: Type = 1;
    pub const TB_CREATE_ACCOUNT_LINKED_EVENT_CHAIN_OPEN: Type = 2;
    pub const TB_CREATE_ACCOUNT_IMPORTED_EVENT_EXPECTED: Type = 22;
    pub const TB_CREATE_ACCOUNT_IMPORTED_EVENT_NOT_EXPECTED: Type = 23;
    pub const TB_CREATE_ACCOUNT_TIMESTAMP_MUST_BE_ZERO: Type = 3;
    pub const TB_CREATE_ACCOUNT_IMPORTED_EVENT_TIMESTAMP_OUT_OF_RANGE: Type = 24;
    pub const TB_CREATE_ACCOUNT_IMPORTED_EVENT_TIMESTAMP_MUST_NOT_ADVANCE: Type = 25;
    pub const TB_CREATE_ACCOUNT_RESERVED_FIELD: Type = 4;
    pub const TB_CREATE_ACCOUNT_RESERVED_FLAG: Type = 5;
    pub const TB_CREATE_ACCOUNT_ID_MUST_NOT_BE_ZERO: Type = 6;
    pub const TB_CREATE_ACCOUNT_ID_MUST_NOT_BE_INT_MAX: Type = 7;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_FLAGS: Type = 15;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_USER_DATA_128: Type = 16;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_USER_DATA_64: Type = 17;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_USER_DATA_32: Type = 18;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_LEDGER: Type = 19;
    pub const TB_CREATE_ACCOUNT_EXISTS_WITH_DIFFERENT_CODE: Type = 20;
    pub const TB_CREATE_ACCOUNT_EXISTS: Type = 21;
    pub const TB_CREATE_ACCOUNT_FLAGS_ARE_MUTUALLY_EXCLUSIVE: Type = 8;
    pub const TB_CREATE_ACCOUNT_DEBITS_PENDING_MUST_BE_ZERO: Type = 9;
    pub const TB_CREATE_ACCOUNT_DEBITS_POSTED_MUST_BE_ZERO: Type = 10;
    pub const TB_CREATE_ACCOUNT_CREDITS_PENDING_MUST_BE_ZERO: Type = 11;
    pub const TB_CREATE_ACCOUNT_CREDITS_POSTED_MUST_BE_ZERO: Type = 12;
    pub const TB_CREATE_ACCOUNT_LEDGER_MUST_NOT_BE_ZERO: Type = 13;
    pub const TB_CREATE_ACCOUNT_CODE_MUST_NOT_BE_ZERO: Type = 14;
    pub const TB_CREATE_ACCOUNT_IMPORTED_EVENT_TIMESTAMP_MUST_NOT_REGRESS: Type = 26;
}
pub mod TB_CREATE_TRANSFER_RESULT {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_CREATE_TRANSFER_OK: Type = 0;
    pub const TB_CREATE_TRANSFER_LINKED_EVENT_FAILED: Type = 1;
    pub const TB_CREATE_TRANSFER_LINKED_EVENT_CHAIN_OPEN: Type = 2;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_EXPECTED: Type = 56;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_NOT_EXPECTED: Type = 57;
    pub const TB_CREATE_TRANSFER_TIMESTAMP_MUST_BE_ZERO: Type = 3;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMESTAMP_OUT_OF_RANGE: Type = 58;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMESTAMP_MUST_NOT_ADVANCE: Type = 59;
    pub const TB_CREATE_TRANSFER_RESERVED_FLAG: Type = 4;
    pub const TB_CREATE_TRANSFER_ID_MUST_NOT_BE_ZERO: Type = 5;
    pub const TB_CREATE_TRANSFER_ID_MUST_NOT_BE_INT_MAX: Type = 6;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_FLAGS: Type = 36;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_PENDING_ID: Type = 40;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_TIMEOUT: Type = 44;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_DEBIT_ACCOUNT_ID: Type = 37;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_CREDIT_ACCOUNT_ID: Type = 38;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_AMOUNT: Type = 39;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_USER_DATA_128: Type = 41;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_USER_DATA_64: Type = 42;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_USER_DATA_32: Type = 43;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_LEDGER: Type = 67;
    pub const TB_CREATE_TRANSFER_EXISTS_WITH_DIFFERENT_CODE: Type = 45;
    pub const TB_CREATE_TRANSFER_EXISTS: Type = 46;
    pub const TB_CREATE_TRANSFER_ID_ALREADY_FAILED: Type = 68;
    pub const TB_CREATE_TRANSFER_FLAGS_ARE_MUTUALLY_EXCLUSIVE: Type = 7;
    pub const TB_CREATE_TRANSFER_DEBIT_ACCOUNT_ID_MUST_NOT_BE_ZERO: Type = 8;
    pub const TB_CREATE_TRANSFER_DEBIT_ACCOUNT_ID_MUST_NOT_BE_INT_MAX: Type = 9;
    pub const TB_CREATE_TRANSFER_CREDIT_ACCOUNT_ID_MUST_NOT_BE_ZERO: Type = 10;
    pub const TB_CREATE_TRANSFER_CREDIT_ACCOUNT_ID_MUST_NOT_BE_INT_MAX: Type = 11;
    pub const TB_CREATE_TRANSFER_ACCOUNTS_MUST_BE_DIFFERENT: Type = 12;
    pub const TB_CREATE_TRANSFER_PENDING_ID_MUST_BE_ZERO: Type = 13;
    pub const TB_CREATE_TRANSFER_PENDING_ID_MUST_NOT_BE_ZERO: Type = 14;
    pub const TB_CREATE_TRANSFER_PENDING_ID_MUST_NOT_BE_INT_MAX: Type = 15;
    pub const TB_CREATE_TRANSFER_PENDING_ID_MUST_BE_DIFFERENT: Type = 16;
    pub const TB_CREATE_TRANSFER_TIMEOUT_RESERVED_FOR_PENDING_TRANSFER: Type = 17;
    pub const TB_CREATE_TRANSFER_CLOSING_TRANSFER_MUST_BE_PENDING: Type = 64;
    pub const TB_CREATE_TRANSFER_LEDGER_MUST_NOT_BE_ZERO: Type = 19;
    pub const TB_CREATE_TRANSFER_CODE_MUST_NOT_BE_ZERO: Type = 20;
    pub const TB_CREATE_TRANSFER_DEBIT_ACCOUNT_NOT_FOUND: Type = 21;
    pub const TB_CREATE_TRANSFER_CREDIT_ACCOUNT_NOT_FOUND: Type = 22;
    pub const TB_CREATE_TRANSFER_ACCOUNTS_MUST_HAVE_THE_SAME_LEDGER: Type = 23;
    pub const TB_CREATE_TRANSFER_TRANSFER_MUST_HAVE_THE_SAME_LEDGER_AS_ACCOUNTS: Type = 24;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_NOT_FOUND: Type = 25;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_NOT_PENDING: Type = 26;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_HAS_DIFFERENT_DEBIT_ACCOUNT_ID: Type = 27;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_HAS_DIFFERENT_CREDIT_ACCOUNT_ID: Type = 28;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_HAS_DIFFERENT_LEDGER: Type = 29;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_HAS_DIFFERENT_CODE: Type = 30;
    pub const TB_CREATE_TRANSFER_EXCEEDS_PENDING_TRANSFER_AMOUNT: Type = 31;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_HAS_DIFFERENT_AMOUNT: Type = 32;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_ALREADY_POSTED: Type = 33;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_ALREADY_VOIDED: Type = 34;
    pub const TB_CREATE_TRANSFER_PENDING_TRANSFER_EXPIRED: Type = 35;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMESTAMP_MUST_NOT_REGRESS: Type = 60;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMESTAMP_MUST_POSTDATE_DEBIT_ACCOUNT: Type = 61;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMESTAMP_MUST_POSTDATE_CREDIT_ACCOUNT: Type = 62;
    pub const TB_CREATE_TRANSFER_IMPORTED_EVENT_TIMEOUT_MUST_BE_ZERO: Type = 63;
    pub const TB_CREATE_TRANSFER_DEBIT_ACCOUNT_ALREADY_CLOSED: Type = 65;
    pub const TB_CREATE_TRANSFER_CREDIT_ACCOUNT_ALREADY_CLOSED: Type = 66;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_DEBITS_PENDING: Type = 47;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_CREDITS_PENDING: Type = 48;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_DEBITS_POSTED: Type = 49;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_CREDITS_POSTED: Type = 50;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_DEBITS: Type = 51;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_CREDITS: Type = 52;
    pub const TB_CREATE_TRANSFER_OVERFLOWS_TIMEOUT: Type = 53;
    pub const TB_CREATE_TRANSFER_EXCEEDS_CREDITS: Type = 54;
    pub const TB_CREATE_TRANSFER_EXCEEDS_DEBITS: Type = 55;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_create_accounts_result_t {
    pub index: u32,
    pub result: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_create_accounts_result_t"]
        [::std::mem::size_of::<tb_create_accounts_result_t>() - 8usize];
    ["Alignment of tb_create_accounts_result_t"]
        [::std::mem::align_of::<tb_create_accounts_result_t>() - 4usize];
    ["Offset of field: tb_create_accounts_result_t::index"]
        [::std::mem::offset_of!(tb_create_accounts_result_t, index) - 0usize];
    ["Offset of field: tb_create_accounts_result_t::result"]
        [::std::mem::offset_of!(tb_create_accounts_result_t, result) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_create_transfers_result_t {
    pub index: u32,
    pub result: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_create_transfers_result_t"]
        [::std::mem::size_of::<tb_create_transfers_result_t>() - 8usize];
    ["Alignment of tb_create_transfers_result_t"]
        [::std::mem::align_of::<tb_create_transfers_result_t>() - 4usize];
    ["Offset of field: tb_create_transfers_result_t::index"]
        [::std::mem::offset_of!(tb_create_transfers_result_t, index) - 0usize];
    ["Offset of field: tb_create_transfers_result_t::result"]
        [::std::mem::offset_of!(tb_create_transfers_result_t, result) - 4usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_account_filter_t {
    pub account_id: tb_uint128_t,
    pub user_data_128: tb_uint128_t,
    pub user_data_64: u64,
    pub user_data_32: u32,
    pub code: u16,
    pub reserved: [u8; 58usize],
    pub timestamp_min: u64,
    pub timestamp_max: u64,
    pub limit: u32,
    pub flags: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_account_filter_t"][::std::mem::size_of::<tb_account_filter_t>() - 128usize];
    ["Alignment of tb_account_filter_t"][::std::mem::align_of::<tb_account_filter_t>() - 16usize];
    ["Offset of field: tb_account_filter_t::account_id"]
        [::std::mem::offset_of!(tb_account_filter_t, account_id) - 0usize];
    ["Offset of field: tb_account_filter_t::user_data_128"]
        [::std::mem::offset_of!(tb_account_filter_t, user_data_128) - 16usize];
    ["Offset of field: tb_account_filter_t::user_data_64"]
        [::std::mem::offset_of!(tb_account_filter_t, user_data_64) - 32usize];
    ["Offset of field: tb_account_filter_t::user_data_32"]
        [::std::mem::offset_of!(tb_account_filter_t, user_data_32) - 40usize];
    ["Offset of field: tb_account_filter_t::code"]
        [::std::mem::offset_of!(tb_account_filter_t, code) - 44usize];
    ["Offset of field: tb_account_filter_t::reserved"]
        [::std::mem::offset_of!(tb_account_filter_t, reserved) - 46usize];
    ["Offset of field: tb_account_filter_t::timestamp_min"]
        [::std::mem::offset_of!(tb_account_filter_t, timestamp_min) - 104usize];
    ["Offset of field: tb_account_filter_t::timestamp_max"]
        [::std::mem::offset_of!(tb_account_filter_t, timestamp_max) - 112usize];
    ["Offset of field: tb_account_filter_t::limit"]
        [::std::mem::offset_of!(tb_account_filter_t, limit) - 120usize];
    ["Offset of field: tb_account_filter_t::flags"]
        [::std::mem::offset_of!(tb_account_filter_t, flags) - 124usize];
};
pub mod TB_ACCOUNT_FILTER_FLAGS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_ACCOUNT_FILTER_DEBITS: Type = 1;
    pub const TB_ACCOUNT_FILTER_CREDITS: Type = 2;
    pub const TB_ACCOUNT_FILTER_REVERSED: Type = 4;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_account_balance_t {
    pub debits_pending: tb_uint128_t,
    pub debits_posted: tb_uint128_t,
    pub credits_pending: tb_uint128_t,
    pub credits_posted: tb_uint128_t,
    pub timestamp: u64,
    pub reserved: [u8; 56usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_account_balance_t"][::std::mem::size_of::<tb_account_balance_t>() - 128usize];
    ["Alignment of tb_account_balance_t"][::std::mem::align_of::<tb_account_balance_t>() - 16usize];
    ["Offset of field: tb_account_balance_t::debits_pending"]
        [::std::mem::offset_of!(tb_account_balance_t, debits_pending) - 0usize];
    ["Offset of field: tb_account_balance_t::debits_posted"]
        [::std::mem::offset_of!(tb_account_balance_t, debits_posted) - 16usize];
    ["Offset of field: tb_account_balance_t::credits_pending"]
        [::std::mem::offset_of!(tb_account_balance_t, credits_pending) - 32usize];
    ["Offset of field: tb_account_balance_t::credits_posted"]
        [::std::mem::offset_of!(tb_account_balance_t, credits_posted) - 48usize];
    ["Offset of field: tb_account_balance_t::timestamp"]
        [::std::mem::offset_of!(tb_account_balance_t, timestamp) - 64usize];
    ["Offset of field: tb_account_balance_t::reserved"]
        [::std::mem::offset_of!(tb_account_balance_t, reserved) - 72usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Pod, :: bytemuck :: Zeroable)]
pub struct tb_query_filter_t {
    pub user_data_128: tb_uint128_t,
    pub user_data_64: u64,
    pub user_data_32: u32,
    pub ledger: u32,
    pub code: u16,
    pub reserved: [u8; 6usize],
    pub timestamp_min: u64,
    pub timestamp_max: u64,
    pub limit: u32,
    pub flags: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_query_filter_t"][::std::mem::size_of::<tb_query_filter_t>() - 64usize];
    ["Alignment of tb_query_filter_t"][::std::mem::align_of::<tb_query_filter_t>() - 16usize];
    ["Offset of field: tb_query_filter_t::user_data_128"]
        [::std::mem::offset_of!(tb_query_filter_t, user_data_128) - 0usize];
    ["Offset of field: tb_query_filter_t::user_data_64"]
        [::std::mem::offset_of!(tb_query_filter_t, user_data_64) - 16usize];
    ["Offset of field: tb_query_filter_t::user_data_32"]
        [::std::mem::offset_of!(tb_query_filter_t, user_data_32) - 24usize];
    ["Offset of field: tb_query_filter_t::ledger"]
        [::std::mem::offset_of!(tb_query_filter_t, ledger) - 28usize];
    ["Offset of field: tb_query_filter_t::code"]
        [::std::mem::offset_of!(tb_query_filter_t, code) - 32usize];
    ["Offset of field: tb_query_filter_t::reserved"]
        [::std::mem::offset_of!(tb_query_filter_t, reserved) - 34usize];
    ["Offset of field: tb_query_filter_t::timestamp_min"]
        [::std::mem::offset_of!(tb_query_filter_t, timestamp_min) - 40usize];
    ["Offset of field: tb_query_filter_t::timestamp_max"]
        [::std::mem::offset_of!(tb_query_filter_t, timestamp_max) - 48usize];
    ["Offset of field: tb_query_filter_t::limit"]
        [::std::mem::offset_of!(tb_query_filter_t, limit) - 56usize];
    ["Offset of field: tb_query_filter_t::flags"]
        [::std::mem::offset_of!(tb_query_filter_t, flags) - 60usize];
};
pub mod TB_QUERY_FILTER_FLAGS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_QUERY_FILTER_REVERSED: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tb_client_t {
    pub opaque: [u64; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_client_t"][::std::mem::size_of::<tb_client_t>() - 32usize];
    ["Alignment of tb_client_t"][::std::mem::align_of::<tb_client_t>() - 8usize];
    ["Offset of field: tb_client_t::opaque"][::std::mem::offset_of!(tb_client_t, opaque) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tb_packet_t {
    pub user_data: *mut ::std::os::raw::c_void,
    pub data: *mut ::std::os::raw::c_void,
    pub data_size: u32,
    pub user_tag: u16,
    pub operation: u8,
    pub status: u8,
    pub opaque: [u8; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_packet_t"][::std::mem::size_of::<tb_packet_t>() - 88usize];
    ["Alignment of tb_packet_t"][::std::mem::align_of::<tb_packet_t>() - 8usize];
    ["Offset of field: tb_packet_t::user_data"]
        [::std::mem::offset_of!(tb_packet_t, user_data) - 0usize];
    ["Offset of field: tb_packet_t::data"][::std::mem::offset_of!(tb_packet_t, data) - 8usize];
    ["Offset of field: tb_packet_t::data_size"]
        [::std::mem::offset_of!(tb_packet_t, data_size) - 16usize];
    ["Offset of field: tb_packet_t::user_tag"]
        [::std::mem::offset_of!(tb_packet_t, user_tag) - 20usize];
    ["Offset of field: tb_packet_t::operation"]
        [::std::mem::offset_of!(tb_packet_t, operation) - 22usize];
    ["Offset of field: tb_packet_t::status"][::std::mem::offset_of!(tb_packet_t, status) - 23usize];
    ["Offset of field: tb_packet_t::opaque"][::std::mem::offset_of!(tb_packet_t, opaque) - 24usize];
};
pub mod TB_OPERATION {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_OPERATION_PULSE: Type = 128;
    pub const TB_OPERATION_GET_CHANGE_EVENTS: Type = 137;
    pub const TB_OPERATION_CREATE_ACCOUNTS: Type = 138;
    pub const TB_OPERATION_CREATE_TRANSFERS: Type = 139;
    pub const TB_OPERATION_LOOKUP_ACCOUNTS: Type = 140;
    pub const TB_OPERATION_LOOKUP_TRANSFERS: Type = 141;
    pub const TB_OPERATION_GET_ACCOUNT_TRANSFERS: Type = 142;
    pub const TB_OPERATION_GET_ACCOUNT_BALANCES: Type = 143;
    pub const TB_OPERATION_QUERY_ACCOUNTS: Type = 144;
    pub const TB_OPERATION_QUERY_TRANSFERS: Type = 145;
}
pub mod TB_PACKET_STATUS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_PACKET_OK: Type = 0;
    pub const TB_PACKET_TOO_MUCH_DATA: Type = 1;
    pub const TB_PACKET_CLIENT_EVICTED: Type = 2;
    pub const TB_PACKET_CLIENT_RELEASE_TOO_LOW: Type = 3;
    pub const TB_PACKET_CLIENT_RELEASE_TOO_HIGH: Type = 4;
    pub const TB_PACKET_CLIENT_SHUTDOWN: Type = 5;
    pub const TB_PACKET_INVALID_OPERATION: Type = 6;
    pub const TB_PACKET_INVALID_DATA_SIZE: Type = 7;
}
pub mod TB_INIT_STATUS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_INIT_SUCCESS: Type = 0;
    pub const TB_INIT_UNEXPECTED: Type = 1;
    pub const TB_INIT_OUT_OF_MEMORY: Type = 2;
    pub const TB_INIT_ADDRESS_INVALID: Type = 3;
    pub const TB_INIT_ADDRESS_LIMIT_EXCEEDED: Type = 4;
    pub const TB_INIT_SYSTEM_RESOURCES: Type = 5;
    pub const TB_INIT_NETWORK_SUBSYSTEM: Type = 6;
}
pub mod TB_CLIENT_STATUS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_CLIENT_OK: Type = 0;
    pub const TB_CLIENT_INVALID: Type = 1;
}
pub mod TB_REGISTER_LOG_CALLBACK_STATUS {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_REGISTER_LOG_CALLBACK_SUCCESS: Type = 0;
    pub const TB_REGISTER_LOG_CALLBACK_ALREADY_REGISTERED: Type = 1;
    pub const TB_REGISTER_LOG_CALLBACK_NOT_REGISTERED: Type = 2;
}
pub mod TB_LOG_LEVEL {
    pub type Type = ::std::os::raw::c_uint;
    pub const TB_LOG_ERR: Type = 0;
    pub const TB_LOG_WARN: Type = 1;
    pub const TB_LOG_INFO: Type = 2;
    pub const TB_LOG_DEBUG: Type = 3;
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone, :: bytemuck :: Zeroable)]
pub struct tb_init_parameters_t {
    pub cluster_id: tb_uint128_t,
    pub client_id: tb_uint128_t,
    pub addresses_ptr: *mut u8,
    pub addresses_len: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tb_init_parameters_t"][::std::mem::size_of::<tb_init_parameters_t>() - 48usize];
    ["Alignment of tb_init_parameters_t"][::std::mem::align_of::<tb_init_parameters_t>() - 16usize];
    ["Offset of field: tb_init_parameters_t::cluster_id"]
        [::std::mem::offset_of!(tb_init_parameters_t, cluster_id) - 0usize];
    ["Offset of field: tb_init_parameters_t::client_id"]
        [::std::mem::offset_of!(tb_init_parameters_t, client_id) - 16usize];
    ["Offset of field: tb_init_parameters_t::addresses_ptr"]
        [::std::mem::offset_of!(tb_init_parameters_t, addresses_ptr) - 32usize];
    ["Offset of field: tb_init_parameters_t::addresses_len"]
        [::std::mem::offset_of!(tb_init_parameters_t, addresses_len) - 40usize];
};
pub type tb_completion_t = ::std::option::Option<
    unsafe extern "C" fn(
        userdata: usize,
        packet: *mut tb_packet_t,
        timestamp: u64,
        result: *const u8,
        result_size: u32,
    ),
>;
unsafe extern "C" {
    pub fn tb_client_init(
        client_out: *mut tb_client_t,
        cluster_id: *const u8,
        address_ptr: *const ::std::os::raw::c_char,
        address_len: u32,
        completion_ctx: usize,
        completion_callback: tb_completion_t,
    ) -> TB_INIT_STATUS::Type;
}
unsafe extern "C" {
    pub fn tb_client_init_echo(
        client_out: *mut tb_client_t,
        cluster_id: *const u8,
        address_ptr: *const ::std::os::raw::c_char,
        address_len: u32,
        completion_ctx: usize,
        completion_callback: tb_completion_t,
    ) -> TB_INIT_STATUS::Type;
}
unsafe extern "C" {
    #[allow(improper_ctypes)]
    pub fn tb_client_init_parameters(
        client: *mut tb_client_t,
        init_parameters_out: *mut tb_init_parameters_t,
    ) -> TB_CLIENT_STATUS::Type;
}
unsafe extern "C" {
    pub fn tb_client_completion_context(
        client: *mut tb_client_t,
        completion_ctx_out: *mut usize,
    ) -> TB_CLIENT_STATUS::Type;
}
unsafe extern "C" {
    pub fn tb_client_submit(
        client: *mut tb_client_t,
        packet: *mut tb_packet_t,
    ) -> TB_CLIENT_STATUS::Type;
}
unsafe extern "C" {
    pub fn tb_client_deinit(client: *mut tb_client_t) -> TB_CLIENT_STATUS::Type;
}
unsafe extern "C" {
    pub fn tb_client_register_log_callback(
        callback: ::std::option::Option<
            unsafe extern "C" fn(arg1: TB_LOG_LEVEL::Type, arg2: *const u8, arg3: u32),
        >,
        debug: bool,
    ) -> TB_REGISTER_LOG_CALLBACK_STATUS::Type;
}
pub type __uint128_t = u128;
