pub const __WORDSIZE: u32 = 64;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 128usize];
    ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 8usize];
    ["Offset of field: __mbstate_t::__mbstate8"]
        [::std::mem::offset_of!(__mbstate_t, __mbstate8) - 0usize];
    ["Offset of field: __mbstate_t::_mbstateL"]
        [::std::mem::offset_of!(__mbstate_t, _mbstateL) - 0usize];
};
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __darwin_pthread_handler_rec"]
        [::std::mem::size_of::<__darwin_pthread_handler_rec>() - 24usize];
    ["Alignment of __darwin_pthread_handler_rec"]
        [::std::mem::align_of::<__darwin_pthread_handler_rec>() - 8usize];
    ["Offset of field: __darwin_pthread_handler_rec::__routine"]
        [::std::mem::offset_of!(__darwin_pthread_handler_rec, __routine) - 0usize];
    ["Offset of field: __darwin_pthread_handler_rec::__arg"]
        [::std::mem::offset_of!(__darwin_pthread_handler_rec, __arg) - 8usize];
    ["Offset of field: __darwin_pthread_handler_rec::__next"]
        [::std::mem::offset_of!(__darwin_pthread_handler_rec, __next) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_attr_t"][::std::mem::size_of::<_opaque_pthread_attr_t>() - 64usize];
    ["Alignment of _opaque_pthread_attr_t"]
        [::std::mem::align_of::<_opaque_pthread_attr_t>() - 8usize];
    ["Offset of field: _opaque_pthread_attr_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_attr_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_attr_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_attr_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_cond_t"][::std::mem::size_of::<_opaque_pthread_cond_t>() - 48usize];
    ["Alignment of _opaque_pthread_cond_t"]
        [::std::mem::align_of::<_opaque_pthread_cond_t>() - 8usize];
    ["Offset of field: _opaque_pthread_cond_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_cond_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_cond_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_cond_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_condattr_t"]
        [::std::mem::size_of::<_opaque_pthread_condattr_t>() - 16usize];
    ["Alignment of _opaque_pthread_condattr_t"]
        [::std::mem::align_of::<_opaque_pthread_condattr_t>() - 8usize];
    ["Offset of field: _opaque_pthread_condattr_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_condattr_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_condattr_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_condattr_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_mutex_t"][::std::mem::size_of::<_opaque_pthread_mutex_t>() - 64usize];
    ["Alignment of _opaque_pthread_mutex_t"]
        [::std::mem::align_of::<_opaque_pthread_mutex_t>() - 8usize];
    ["Offset of field: _opaque_pthread_mutex_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_mutex_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_mutex_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_mutex_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_mutexattr_t"]
        [::std::mem::size_of::<_opaque_pthread_mutexattr_t>() - 16usize];
    ["Alignment of _opaque_pthread_mutexattr_t"]
        [::std::mem::align_of::<_opaque_pthread_mutexattr_t>() - 8usize];
    ["Offset of field: _opaque_pthread_mutexattr_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_mutexattr_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_mutexattr_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_mutexattr_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_once_t"][::std::mem::size_of::<_opaque_pthread_once_t>() - 16usize];
    ["Alignment of _opaque_pthread_once_t"]
        [::std::mem::align_of::<_opaque_pthread_once_t>() - 8usize];
    ["Offset of field: _opaque_pthread_once_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_once_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_once_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_once_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_rwlock_t"]
        [::std::mem::size_of::<_opaque_pthread_rwlock_t>() - 200usize];
    ["Alignment of _opaque_pthread_rwlock_t"]
        [::std::mem::align_of::<_opaque_pthread_rwlock_t>() - 8usize];
    ["Offset of field: _opaque_pthread_rwlock_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_rwlock_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_rwlock_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_rwlock_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_rwlockattr_t"]
        [::std::mem::size_of::<_opaque_pthread_rwlockattr_t>() - 24usize];
    ["Alignment of _opaque_pthread_rwlockattr_t"]
        [::std::mem::align_of::<_opaque_pthread_rwlockattr_t>() - 8usize];
    ["Offset of field: _opaque_pthread_rwlockattr_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_rwlockattr_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_rwlockattr_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_rwlockattr_t, __opaque) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _opaque_pthread_t"][::std::mem::size_of::<_opaque_pthread_t>() - 8192usize];
    ["Alignment of _opaque_pthread_t"][::std::mem::align_of::<_opaque_pthread_t>() - 8usize];
    ["Offset of field: _opaque_pthread_t::__sig"]
        [::std::mem::offset_of!(_opaque_pthread_t, __sig) - 0usize];
    ["Offset of field: _opaque_pthread_t::__cleanup_stack"]
        [::std::mem::offset_of!(_opaque_pthread_t, __cleanup_stack) - 8usize];
    ["Offset of field: _opaque_pthread_t::__opaque"]
        [::std::mem::offset_of!(_opaque_pthread_t, __opaque) - 16usize];
};
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
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
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type __uint128_t = u128;
