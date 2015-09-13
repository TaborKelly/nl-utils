

/* automatically generated by rust-bindgen */

pub type __s8 = ::libc::c_char;
pub type __u8 = ::libc::c_uchar;
pub type __s16 = ::libc::c_short;
pub type __u16 = ::libc::c_ushort;
pub type __s32 = ::libc::c_int;
pub type __u32 = ::libc::c_uint;
pub type __s64 = ::libc::c_longlong;
pub type __u64 = ::libc::c_ulonglong;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub fds_bits: [::libc::c_ulong; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __kernel_fd_set = Struct_Unnamed1;
pub type __kernel_sighandler_t =
    ::std::option::Option<extern "C" fn(arg1: ::libc::c_int) -> ()>;
pub type __kernel_key_t = ::libc::c_int;
pub type __kernel_mqd_t = ::libc::c_int;
pub type __kernel_old_uid_t = ::libc::c_ushort;
pub type __kernel_old_gid_t = ::libc::c_ushort;
pub type __kernel_old_dev_t = ::libc::c_ulong;
pub type __kernel_long_t = ::libc::c_long;
pub type __kernel_ulong_t = ::libc::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::libc::c_uint;
pub type __kernel_pid_t = ::libc::c_int;
pub type __kernel_ipc_pid_t = ::libc::c_int;
pub type __kernel_uid_t = ::libc::c_uint;
pub type __kernel_gid_t = ::libc::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::libc::c_int;
pub type __kernel_uid32_t = ::libc::c_uint;
pub type __kernel_gid32_t = ::libc::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub val: [::libc::c_int; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __kernel_fsid_t = Struct_Unnamed2;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::libc::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::libc::c_int;
pub type __kernel_clockid_t = ::libc::c_int;
pub type __kernel_caddr_t = *mut ::libc::c_char;
pub type __kernel_uid16_t = ::libc::c_ushort;
pub type __kernel_gid16_t = ::libc::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3usize],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [::libc::c_char; 0usize],
}
impl ::std::clone::Clone for Struct_sysinfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sysinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __kernel_sa_family_t = ::libc::c_ushort;
pub enum Struct_sockaddr { }
#[repr(C)]
#[derive(Copy)]
pub struct Struct___kernel_sockaddr_storage {
    pub ss_family: __kernel_sa_family_t,
    pub __data: [::libc::c_char; 126usize],
}
impl ::std::clone::Clone for Struct___kernel_sockaddr_storage {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct___kernel_sockaddr_storage {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_sockaddr_nl {
    pub nl_family: __kernel_sa_family_t,
    pub nl_pad: ::libc::c_ushort,
    pub nl_pid: __u32,
    pub nl_groups: __u32,
}
impl ::std::clone::Clone for Struct_sockaddr_nl {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_sockaddr_nl {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nlmsghdr {
    pub nlmsg_len: __u32,
    pub nlmsg_type: __u16,
    pub nlmsg_flags: __u16,
    pub nlmsg_seq: __u32,
    pub nlmsg_pid: __u32,
}
impl ::std::clone::Clone for Struct_nlmsghdr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nlmsghdr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nlmsgerr {
    pub error: ::libc::c_int,
    pub msg: Struct_nlmsghdr,
}
impl ::std::clone::Clone for Struct_nlmsgerr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nlmsgerr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nl_pktinfo {
    pub group: __u32,
}
impl ::std::clone::Clone for Struct_nl_pktinfo {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nl_pktinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nl_mmap_req {
    pub nm_block_size: ::libc::c_uint,
    pub nm_block_nr: ::libc::c_uint,
    pub nm_frame_size: ::libc::c_uint,
    pub nm_frame_nr: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_nl_mmap_req {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nl_mmap_req {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nl_mmap_hdr {
    pub nm_status: ::libc::c_uint,
    pub nm_len: ::libc::c_uint,
    pub nm_group: __u32,
    pub nm_pid: __u32,
    pub nm_uid: __u32,
    pub nm_gid: __u32,
}
impl ::std::clone::Clone for Struct_nl_mmap_hdr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nl_mmap_hdr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_nl_mmap_status = ::libc::c_uint;
pub const NL_MMAP_STATUS_UNUSED: ::libc::c_uint = 0;
pub const NL_MMAP_STATUS_RESERVED: ::libc::c_uint = 1;
pub const NL_MMAP_STATUS_VALID: ::libc::c_uint = 2;
pub const NL_MMAP_STATUS_COPY: ::libc::c_uint = 3;
pub const NL_MMAP_STATUS_SKIP: ::libc::c_uint = 4;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const NETLINK_UNCONNECTED: ::libc::c_uint = 0;
pub const NETLINK_CONNECTED: ::libc::c_uint = 1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_nlattr {
    pub nla_len: __u16,
    pub nla_type: __u16,
}
impl ::std::clone::Clone for Struct_nlattr {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_nlattr {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}