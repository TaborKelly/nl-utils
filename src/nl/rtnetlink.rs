use ::std::io::{Cursor};
use ::byteorder::{NativeEndian, ReadBytesExt};
use ::num::FromPrimitive;
use ::std::fmt;
use nl::{format_indent, NlMsg};

// this is where the NetDeviceFlags enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/net_device_flags.rs"));
// this is where the AddressFamily enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/address_family.rs"));
// this is where the IfaFlags enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/ifa_flags.rs"));
// this is where the Ifa enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/ifa.rs"));
// this is where the Ifla enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/ifla.rs"));
// this is where the Rtn enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rtn.rs"));
// this is where the Rtprot enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rtprot.rs"));
// this is where the RtScope enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rt_scope.rs"));
// this is where the RtTable enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rt_table.rs"));
// this is where the RtmFlags enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rtm_flags.rs"));
// this is where the RtmAttr enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/rtm_attr.rs"));
// this is where the NdState enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/nd_state.rs"));
// this is where the NdFlags enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/nd_flags.rs"));
// this is where the NdAttr enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/nd_attr.rs"));
// this is where the TcAttr enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/tc_attr.rs"));
// this is where the NrMsgType enum was generated by build.rs
include!(concat!(env!("OUT_DIR"), "/nr_msg_type.rs"));

/* TODO:
- concistant naming of messages
- concistant naming of attributes
- better error handling (option vs Err)
- better attribute handling (struct rtnl_link_stats, etc)
- one more indent for all attrs
- move code around, especially generated code, to make things more readable
- more robustness for Rtprot? Theoretically users could use other values.
*/

// A macro for reading and returning None on error.
// r = an expresssion that will return/evaluate to a Result
// s = where to unwrap the result to if it isn't an error
macro_rules! read_and_handle_error {
    ($s:expr, $r:expr) => {{
        let tmp = $r;
        if tmp.is_err() {
            return None;
        }
        $s = tmp.unwrap();
    }}
}

#[derive(Debug, Default, Clone)]
pub struct Rtattr<T> {
     // the length originally encoded in the netlink which includes rta_len,
     // rta_type, and rta_value, but not any padding
    rta_len: u16,
    rta_type: T,
    rta_value: Vec<u8>,
}
impl <T: Default + ::std::fmt::Display + ::num::traits::FromPrimitive> Rtattr<T> {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<Rtattr<T>> {
        let mut s = Rtattr::default();
        read_and_handle_error!(s.rta_len, cursor.read_u16::<NativeEndian>());
        let rta_type: u16;
        read_and_handle_error!(rta_type, cursor.read_u16::<NativeEndian>());
        s.rta_type = T::from_u16(rta_type).unwrap();
        // sizeof(rta_len) + sizeof(rta_type) = 4
        let payload_len: usize = (s.rta_len - 4) as usize;
        let mut vec: Vec<u8> = Vec::with_capacity(payload_len);
        for _ in 0..payload_len {
            let a = cursor.read_u8().unwrap();
            vec.push(a);
        }
        s.rta_value = vec;
        NlMsg::nlmsg_align(cursor);
        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let indent = format_indent(indent);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    rta_len: {},\n", indent, self.rta_len));
        try!(write!(f, "{}    rta_type: {},\n", indent, self.rta_type));
        try!(write!(f, "{}    rta_value: [", indent));
        let mut count: usize = 1;
        for a in self.rta_value.iter() {
            try!(write!(f, " {:#X}", a));
            if count < self.rta_value.len() {
                try!(write!(f, ","));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", indent)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ifinfomsg {
    pub ifi_family: AddressFamily, // AF_UNSPEC
    pub ifi_type: u16,  // Device type
    pub ifi_index: i32, // Interface index
    pub ifi_flags: u32, // Device flags
    pub ifi_change: u32, // change mask
    pub ifi_attr: Vec<Rtattr<Ifla>>,
}
impl Ifinfomsg {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>, nlmsg_len: usize) -> Option<Ifinfomsg> {
        let mut s = Ifinfomsg::default();

        let family: u8;
        read_and_handle_error!(family, cursor.read_u8());
        s.ifi_family = AddressFamily::from_u8(family).unwrap();
        let mut _ifi_pad: u8 = 0;
        read_and_handle_error!(_ifi_pad, cursor.read_u8());
        read_and_handle_error!(s.ifi_type, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ifi_index, cursor.read_i32::<NativeEndian>());
        read_and_handle_error!(s.ifi_flags, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.ifi_change, cursor.read_u32::<NativeEndian>());
        while (cursor.position() as usize) < nlmsg_len {
            let attr = Rtattr::<Ifla>::read(cursor).unwrap();
            s.ifi_attr.push(attr);
        }

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        let i_s_p = format_indent(indent+1);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    ifi_family: {},\n", i_s, self.ifi_family));
        try!(write!(f, "{}    ifi_type: {},\n", i_s, self.ifi_type));
        try!(write!(f, "{}    ifi_index: {},\n", i_s, self.ifi_index));
        try!(write!(f, "{}    ifi_flags: {:#X} (", i_s, self.ifi_flags));
        try!(NetDeviceFlags::pretty_fmt(f, self.ifi_flags));
        try!(write!(f, "),\n{}    ifi_change: {},\n", i_s, self.ifi_change));
        try!(write!(f, "{}    ifi_attr: [ ", i_s));

        let mut count: usize = 1;
        for a in self.ifi_attr.iter() {
            try!(a.pretty_fmt(f, indent+1));
            if count < self.ifi_attr.len() {
                try!(write!(f, ",\n{}", i_s_p));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", i_s)
    }
}
impl ::std::fmt::Display for Ifinfomsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ifaddrmsg {
    pub ifa_family: AddressFamily, // Address type
    pub ifa_prefixlen: u8, // Prefixlength of address
    pub ifa_flags: u8, // Address flags
    pub ifa_scope: u8, // Address scope
    pub ifa_index: u32, // Interface index
    pub ifa_attr: Vec<Rtattr<Ifa>>,
}
impl Ifaddrmsg {
    // Ifaddrmsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>, nlmsg_len: usize) -> Option<Ifaddrmsg> {
        let mut s = Ifaddrmsg::default();

        let family: u8;
        read_and_handle_error!(family, cursor.read_u8());
        s.ifa_family = AddressFamily::from_u8(family).unwrap();
        read_and_handle_error!(s.ifa_prefixlen, cursor.read_u8());
        read_and_handle_error!(s.ifa_flags, cursor.read_u8());
        read_and_handle_error!(s.ifa_scope, cursor.read_u8());
        read_and_handle_error!(s.ifa_index, cursor.read_u32::<NativeEndian>());
        while (cursor.position() as usize) < nlmsg_len {
            let attr = Rtattr::<Ifa>::read(cursor).unwrap();
            s.ifa_attr.push(attr);
        }

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        let i_s_p = format_indent(indent+1);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    ifa_family: {},\n", i_s, self.ifa_family));
        try!(write!(f, "{}    ifa_prefixlen: {},\n", i_s, self.ifa_prefixlen));
        try!(write!(f, "{}    ifa_flags: {:#X} (", i_s, self.ifa_flags));
        try!(IfaFlags::pretty_fmt(f, self.ifa_flags as u32));
        try!(write!(f, "),\n{}    ifa_scope: {},\n", i_s, self.ifa_scope));
        try!(write!(f, "{}    ifa_index: {},\n", i_s, self.ifa_index));
        try!(write!(f, "{}    ifa_attr: [ ", i_s));

        let mut count: usize = 1;
        for a in self.ifa_attr.iter() {
            try!(a.pretty_fmt(f, indent+1));
            if count < self.ifa_attr.len() {
                try!(write!(f, ",\n{}", i_s_p));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", i_s)
    }
}
impl ::std::fmt::Display for Ifaddrmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

impl RtScope {
    fn pretty_fmt(f: &mut ::std::fmt::Formatter, num: u8) -> ::std::fmt::Result {
        let option = RtScope::from_u8(num);
        match option {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "user defined"),
        }
    }
}

impl RtTable {
    fn pretty_fmt(f: &mut ::std::fmt::Formatter, num: u8) -> ::std::fmt::Result {
        let option = RtTable::from_u8(num);
        match option {
            Some(e) => write!(f, "{}", e),
            None => write!(f, "user defined"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Rtmsg {
    pub rtm_family: AddressFamily, // Address family of route
    pub rtm_dst_len: u8, // Length of destination
    pub rtm_src_len: u8, // Length of source
    pub rtm_tos: u8, // TOS filter

    pub rtm_table: u8, // Routing table ID
    pub rtm_protocol: Rtprot, // Routing protocol
    pub rtm_scope: u8,
    pub rtm_type: Rtn,

    pub rtm_flags: u32,
    pub rtm_attr: Vec<Rtattr<RtmAttr>>,
}
impl Rtmsg {
    // Ifaddrmsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>, nlmsg_len: usize) -> Option<Rtmsg> {
        let mut s = Rtmsg::default();

        let family: u8;
        read_and_handle_error!(family, cursor.read_u8());
        s.rtm_family = AddressFamily::from_u8(family).unwrap();
        read_and_handle_error!(s.rtm_dst_len, cursor.read_u8());
        read_and_handle_error!(s.rtm_src_len, cursor.read_u8());
        read_and_handle_error!(s.rtm_tos, cursor.read_u8());

        read_and_handle_error!(s.rtm_table, cursor.read_u8());
        let rtm_protocol: u8;
        read_and_handle_error!(rtm_protocol, cursor.read_u8());
        s.rtm_protocol = Rtprot::from_u8(rtm_protocol).unwrap();
        read_and_handle_error!(s.rtm_scope, cursor.read_u8());
        let rtm_type: u8;
        read_and_handle_error!(rtm_type, cursor.read_u8());
        s.rtm_type = Rtn::from_u8(rtm_type).unwrap();

        read_and_handle_error!(s.rtm_flags, cursor.read_u32::<NativeEndian>());

        while (cursor.position() as usize) < nlmsg_len {
            let attr = Rtattr::<RtmAttr>::read(cursor).unwrap();
            s.rtm_attr.push(attr);
        }

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        let i_s_p = format_indent(indent+1);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    rtm_family: {},\n", i_s, self.rtm_family));
        try!(write!(f, "{}    rtm_dst_len: {},\n", i_s, self.rtm_dst_len));
        try!(write!(f, "{}    rtm_src_len: {},\n", i_s, self.rtm_src_len));
        try!(write!(f, "{}    rtm_tos: {},\n", i_s, self.rtm_tos));
        try!(write!(f, "{}    rtm_table: {} (", i_s, self.rtm_table));
        try!(RtTable::pretty_fmt(f, self.rtm_table));
        try!(write!(f, "),\n{}    rtm_protocol: {},\n", i_s, self.rtm_protocol));
        try!(write!(f, "{}    rtm_scope: {} (", i_s, self.rtm_scope));
        try!(RtScope::pretty_fmt(f, self.rtm_scope));
        try!(write!(f, "),\n{}    rtm_type: {},\n", i_s, self.rtm_type));

        try!(write!(f, "{}    rtm_flags: {:#X} (", i_s, self.rtm_flags));
        try!(RtmFlags::pretty_fmt(f, self.rtm_flags as u32));
        try!(write!(f, "),\n{}    rtm_attr: [ ", i_s));
        let mut count: usize = 1;
        for a in self.rtm_attr.iter() {
            try!(a.pretty_fmt(f, indent+1));
            if count < self.rtm_attr.len() {
                try!(write!(f, ",\n{}", i_s_p));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", i_s)
    }
}
impl ::std::fmt::Display for Rtmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct NdaCacheinfo {
    pub ndm_confirmed: u32,
    pub ndm_used: u32,
    pub ndm_updated: u32,
    pub ndm_flags: u32,
}
impl NdaCacheinfo {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<NdaCacheinfo> {
        let mut s = NdaCacheinfo::default();

        read_and_handle_error!(s.ndm_confirmed, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.ndm_used, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.ndm_updated, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.ndm_flags, cursor.read_u32::<NativeEndian>());

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let indent = format_indent(indent);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    ndm_confirmed: {},\n", indent, self.ndm_confirmed));
        try!(write!(f, "{}    ndm_used: {},\n", indent, self.ndm_used));
        try!(write!(f, "{}    ndm_updated: {:#X},\n", indent, self.ndm_updated));
        try!(write!(f, "{}    ndm_flags: {:#X},\n", indent, self.ndm_flags));
        write!(f, "{}}}", indent)
    }
}
impl ::std::fmt::Display for NdaCacheinfo {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ndmsg {
    pub ndm_family: u8,
    pub ndm_ifindex: i32, // Interface index
    pub ndm_state: u16, // State
    pub ndm_flags: u8, // Flags
    // I *think* this is the right type, but don't have a lot of examples of
    // its use.
    pub ndm_type: NdAttr,
    pub ndm_cacheinfo: Option<NdaCacheinfo>,
    pub ndm_attr: Vec<Rtattr<NdAttr>>,
}
impl Ndmsg {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>, nlmsg_len: usize) -> Option<Ndmsg> {
        let mut s = Ndmsg::default();

        read_and_handle_error!(s.ndm_family, cursor.read_u8());
        let mut _ndm_pad_u8: u8 = 0;
        read_and_handle_error!(_ndm_pad_u8, cursor.read_u8());
        let mut _ndm_pad_u16: u16 = 0;
        read_and_handle_error!(_ndm_pad_u16, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ndm_ifindex, cursor.read_i32::<NativeEndian>());
        read_and_handle_error!(s.ndm_state, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ndm_flags, cursor.read_u8());
        let ndm_type: u8;
        read_and_handle_error!(ndm_type, cursor.read_u8());
        s.ndm_type = NdAttr::from_u8(ndm_type).unwrap();

        if s.ndm_type == NdAttr::NDA_CACHEINFO {
            s.ndm_cacheinfo = NdaCacheinfo::read(cursor);
        }

        while (cursor.position() as usize) < nlmsg_len {
            let attr = Rtattr::<NdAttr>::read(cursor).unwrap();
            s.ndm_attr.push(attr);
        }

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        let i_s_p = format_indent(indent+1);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    ndm_family: {},\n", i_s, self.ndm_family));
        try!(write!(f, "{}    ndm_ifindex: {},\n", i_s, self.ndm_ifindex));
        try!(write!(f, "{}    ndm_state: {:#X} (", i_s, self.ndm_state));
        try!(NdState::pretty_fmt(f, self.ndm_state as u32));
        try!(write!(f, "),\n{}    ndm_flags: {:#X} (", i_s, self.ndm_flags));
        try!(NdFlags::pretty_fmt(f, self.ndm_flags as u32));
        try!(write!(f, "),\n{}    ndm_type: {},\n", i_s, self.ndm_type));
        try!(write!(f, "{}    ndm_cacheinfo: ", i_s));
        match self.ndm_cacheinfo {
            None => try!(write!(f, "None")),
            Some(ref cacheinfo) => try!(cacheinfo.pretty_fmt(f, indent+1)),
        }

        // TODO: macro? Or move into Rtattr?
        try!(write!(f, ",\n{}    ndm_attr: [ ", i_s));
        let mut count: usize = 1;
        for a in self.ndm_attr.iter() {
            try!(a.pretty_fmt(f, indent+1));
            if count < self.ndm_attr.len() {
                try!(write!(f, ",\n{}", i_s_p));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", i_s)
    }
}
impl ::std::fmt::Display for Ndmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Tcmsg {
    pub tcm_family: u8,
    pub tcm_ifindex: i32,
    pub tcm_handle: u32,
    pub tcm_parent: u32,
    pub tcm_info: u32,
    pub tcm_attr: Vec<Rtattr<TcAttr>>,
}
impl Tcmsg {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>, nlmsg_len: usize) -> Option<Tcmsg> {
        let mut s = Tcmsg::default();

        read_and_handle_error!(s.tcm_family, cursor.read_u8());
        let mut _tcm_pad_u8: u8 = 0;
        read_and_handle_error!(_tcm_pad_u8, cursor.read_u8());
        let mut _tcm_pad_u16: u16 = 0;
        read_and_handle_error!(_tcm_pad_u16, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.tcm_ifindex, cursor.read_i32::<NativeEndian>());
        read_and_handle_error!(s.tcm_handle, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.tcm_parent, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.tcm_info, cursor.read_u32::<NativeEndian>());

        // TODO: revisit. Move into Rtattr?
        while (cursor.position() as usize) < nlmsg_len {
            let attr = Rtattr::<TcAttr>::read(cursor).unwrap();
            s.tcm_attr.push(attr);
        }

        Some(s)
    }
    pub fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        let i_s_p = format_indent(indent+1);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    tcm_family: {},\n", i_s, self.tcm_family));
        try!(write!(f, "{}    tcm_ifindex: {},\n", i_s, self.tcm_ifindex));
        try!(write!(f, "{}    tcm_handle: {:#X},\n", i_s, self.tcm_handle));
        try!(write!(f, "{}    tcm_parent: {:#X},\n", i_s, self.tcm_parent));
        try!(write!(f, "{}    tcm_info: {},\n", i_s, self.tcm_info));

        // TODO: macro? Or move into Rtattr?
        try!(write!(f, "{}    tcm_attr: [ ", i_s));
        let mut count: usize = 1;
        for a in self.tcm_attr.iter() {
            try!(a.pretty_fmt(f, indent+1));
            if count < self.tcm_attr.len() {
                try!(write!(f, ",\n{}", i_s_p));
            }
            count = count + 1;
        }
        write!(f, " ],\n{}}}", i_s)
    }
}
impl ::std::fmt::Display for Tcmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}
