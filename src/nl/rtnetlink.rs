use ::std::io::{Cursor};
use ::byteorder::{NativeEndian, ReadBytesExt};

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

#[derive(Debug, Default, Copy, Clone)]
pub struct Ifinfomsg {
    pub ifi_family: u8, // AF_UNSPEC
    pub ifi_type: u16,  // Device type
    pub ifi_index: i32, // Interface index
    pub ifi_flags: u32, // Device flags
    pub ifi_change: u32, // change mask
}
impl Ifinfomsg {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<Ifinfomsg> {
        let mut s = Ifinfomsg::default();

        read_and_handle_error!(s.ifi_family, cursor.read_u8());
        let mut _ifi_pad: u8 = 0;
        read_and_handle_error!(_ifi_pad, cursor.read_u8());
        read_and_handle_error!(s.ifi_type, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ifi_index, cursor.read_i32::<NativeEndian>());
        read_and_handle_error!(s.ifi_flags, cursor.read_u32::<NativeEndian>());
        read_and_handle_error!(s.ifi_change, cursor.read_u32::<NativeEndian>());

        Some(s)
    }
}
impl ::std::fmt::Display for Ifinfomsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{{\n\tifi_family: {},\n\tifi_type: {},\n\t\
               ifi_index: {},\n\tifi_flags: {:#x},\n\tifi_change: {}\n}}",
               self.ifi_family, self.ifi_type, self.ifi_index,
               self.ifi_flags, self.ifi_change)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Ifaddrmsg {
    pub ifa_family: u8, // Address type
    pub ifa_prefixlen: u8, // Prefixlength of address
    pub ifa_flags: u8, // Address flags
    pub ifa_scope: u8, // Address scope
    pub ifa_index: u32, // Interface index
}
impl Ifaddrmsg {
    // Ifaddrmsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<Ifaddrmsg> {
        let mut s = Ifaddrmsg::default();

        read_and_handle_error!(s.ifa_family, cursor.read_u8());
        read_and_handle_error!(s.ifa_prefixlen, cursor.read_u8());
        read_and_handle_error!(s.ifa_flags, cursor.read_u8());
        read_and_handle_error!(s.ifa_scope, cursor.read_u8());
        read_and_handle_error!(s.ifa_index, cursor.read_u32::<NativeEndian>());

        Some(s)
    }
}
impl ::std::fmt::Display for Ifaddrmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{{\n\tifa_family: {},\n\tifa_prefixlen: {},\n\t\
               ifa_flags: {:#x},\n\tifa_scope: {},\n\tifa_index: {}\n}}",
               self.ifa_family, self.ifa_prefixlen, self.ifa_flags,
               self.ifa_scope, self.ifa_index)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Rtmsg {
    pub rtm_family: u8, // Address family of route
    pub rtm_dst_len: u8, // Length of destination
    pub rtm_src_len: u8, // Length of source
    pub rtm_tos: u8, // TOS filter

    pub rtm_table: u8, // Routing table ID
    pub rtm_protocol: u8, // Routing protocol
    pub rtm_scope: u8,
    pub rtm_type: u8,

    pub rtm_flags: u32,
}
impl Rtmsg {
    // Ifaddrmsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<Rtmsg> {
        let mut s = Rtmsg::default();

        read_and_handle_error!(s.rtm_family, cursor.read_u8());
        read_and_handle_error!(s.rtm_dst_len, cursor.read_u8());
        read_and_handle_error!(s.rtm_src_len, cursor.read_u8());
        read_and_handle_error!(s.rtm_tos, cursor.read_u8());

        read_and_handle_error!(s.rtm_table, cursor.read_u8());
        read_and_handle_error!(s.rtm_protocol, cursor.read_u8());
        read_and_handle_error!(s.rtm_scope, cursor.read_u8());
        read_and_handle_error!(s.rtm_type, cursor.read_u8());

        read_and_handle_error!(s.rtm_flags, cursor.read_u32::<NativeEndian>());

        Some(s)
    }
}
impl ::std::fmt::Display for Rtmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{{\n\trtm_family: {},\n\trtm_dst_len: {},\n\t\
               rtm_src_len: {},\n\trtm_tos: {},\n\trtm_table: {}\n\t\
               rtm_protocol: {},\n\trtm_scope: {},\n\trtm_type: {}\n\t\
               rtm_flags: {:#x}\n}}",
               self.rtm_family, self.rtm_dst_len, self.rtm_src_len,
               self.rtm_tos, self.rtm_table, self.rtm_protocol, self.rtm_scope,
               self.rtm_type, self.rtm_flags)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Ndmsg {
    pub ndm_family: u8,
    pub ndm_ifindex: i32, // Interface index
    pub ndm_state: u16, // State
    pub ndm_flags: u8, // Flags
    pub ndm_type: u8,
    // TODO: option nda_cacheinfo
}
impl Ndmsg {
    // Ifinfomsg header is native endian
    pub fn read(cursor: &mut Cursor<&[u8]>) -> Option<Ndmsg> {
        let mut s = Ndmsg::default();

        read_and_handle_error!(s.ndm_family, cursor.read_u8());
        let mut _ndm_pad_u8: u8 = 0;
        read_and_handle_error!(_ndm_pad_u8, cursor.read_u8());
        let mut _ndm_pad_u16: u16 = 0;
        read_and_handle_error!(_ndm_pad_u16, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ndm_ifindex, cursor.read_i32::<NativeEndian>());
        read_and_handle_error!(s.ndm_state, cursor.read_u16::<NativeEndian>());
        read_and_handle_error!(s.ndm_flags, cursor.read_u8());
        read_and_handle_error!(s.ndm_type, cursor.read_u8());

        // TODO revisit: add support for NDA_CACHEINFO/nda_cacheinfo
        if s.ndm_type == 3 {
            panic!("Add support for NDA_CACHEINFO/nda_cacheinfo!");
        }

        Some(s)
    }
}
impl ::std::fmt::Display for Ndmsg {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{{\n\tndm_family: {},\n\tndm_ifindex: {},\n\t\
               ndm_state: {:#x},\n\tndm_flags: {:#x},\n\tndm_type: {}\n}}",
               self.ndm_family, self.ndm_ifindex, self.ndm_state,
               self.ndm_flags, self.ndm_type)
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NrMsgType {
    RTM_NEWLINK = 16,
    RTM_DELLINK = 17,
    RTM_GETLINK = 18,
    RTM_SETLINK = 19,
    RTM_NEWADDR = 20,
    RTM_DELADDR = 21,
    RTM_GETADDR = 22,
    RTM_NEWROUTE = 24,
    RTM_DELROUTE = 25,
    RTM_GETROUTE = 26,
    RTM_NEWNEIGH = 28,
    RTM_DELNEIGH = 29,
    RTM_GETNEIGH = 30,
    RTM_NEWRULE = 32,
    RTM_DELRULE = 33,
    RTM_GETRULE = 34,
    RTM_NEWQDISC = 36,
    RTM_DELQDISC = 37,
    RTM_GETQDISC = 38,
    RTM_NEWTCLASS = 40,
    RTM_DELTCLASS = 41,
    RTM_GETTCLASS = 42,
    RTM_NEWTFILTER = 44,
    RTM_DELTFILTER = 45,
    RTM_GETTFILTER = 46,
    RTM_NEWACTION = 48,
    RTM_DELACTION = 49,
    RTM_GETACTION = 50,
    RTM_NEWPREFIX = 52,
    RTM_GETMULTICAST = 58,
    RTM_GETANYCAST = 62,
    RTM_NEWNEIGHTBL = 64,
    RTM_GETNEIGHTBL = 66,
    RTM_SETNEIGHTBL = 67,
    RTM_NEWNDUSEROPT = 68,
    RTM_NEWADDRLABEL = 72,
    RTM_DELADDRLABEL = 73,
    RTM_GETADDRLABEL = 74,
    RTM_GETDCB = 78,
    RTM_SETDCB = 79,
    RTM_NEWNETCONF = 80,
    RTM_GETNETCONF = 82,
    RTM_NEWMDB = 84,
    RTM_DELMDB = 85,
    RTM_GETMDB = 86,
}
impl ::std::str::FromStr for NrMsgType {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RTM_NEWLINK" => Ok(NrMsgType::RTM_NEWLINK),
            "RTM_DELLINK" => Ok(NrMsgType::RTM_DELLINK),
            "RTM_GETLINK" => Ok(NrMsgType::RTM_GETLINK),
            "RTM_SETLINK" => Ok(NrMsgType::RTM_SETLINK),
            "RTM_NEWADDR" => Ok(NrMsgType::RTM_NEWADDR),
            "RTM_DELADDR" => Ok(NrMsgType::RTM_DELADDR),
            "RTM_GETADDR" => Ok(NrMsgType::RTM_GETADDR),
            "RTM_NEWROUTE" => Ok(NrMsgType::RTM_NEWROUTE),
            "RTM_DELROUTE" => Ok(NrMsgType::RTM_DELROUTE),
            "RTM_GETROUTE" => Ok(NrMsgType::RTM_GETROUTE),
            "RTM_NEWNEIGH" => Ok(NrMsgType::RTM_NEWNEIGH),
            "RTM_DELNEIGH" => Ok(NrMsgType::RTM_DELNEIGH),
            "RTM_GETNEIGH" => Ok(NrMsgType::RTM_GETNEIGH),
            "RTM_NEWRULE" => Ok(NrMsgType::RTM_NEWRULE),
            "RTM_DELRULE" => Ok(NrMsgType::RTM_DELRULE),
            "RTM_GETRULE" => Ok(NrMsgType::RTM_GETRULE),
            "RTM_NEWQDISC" => Ok(NrMsgType::RTM_NEWQDISC),
            "RTM_DELQDISC" => Ok(NrMsgType::RTM_DELQDISC),
            "RTM_GETQDISC" => Ok(NrMsgType::RTM_GETQDISC),
            "RTM_NEWTCLASS" => Ok(NrMsgType::RTM_NEWTCLASS),
            "RTM_DELTCLASS" => Ok(NrMsgType::RTM_DELTCLASS),
            "RTM_GETTCLASS" => Ok(NrMsgType::RTM_GETTCLASS),
            "RTM_NEWTFILTER" => Ok(NrMsgType::RTM_NEWTFILTER),
            "RTM_DELTFILTER" => Ok(NrMsgType::RTM_DELTFILTER),
            "RTM_GETTFILTER" => Ok(NrMsgType::RTM_GETTFILTER),
            "RTM_NEWACTION" => Ok(NrMsgType::RTM_NEWACTION),
            "RTM_DELACTION" => Ok(NrMsgType::RTM_DELACTION),
            "RTM_GETACTION" => Ok(NrMsgType::RTM_GETACTION),
            "RTM_NEWPREFIX" => Ok(NrMsgType::RTM_NEWPREFIX),
            "RTM_GETMULTICAST" => Ok(NrMsgType::RTM_GETMULTICAST),
            "RTM_GETANYCAST" => Ok(NrMsgType::RTM_GETANYCAST),
            "RTM_NEWNEIGHTBL" => Ok(NrMsgType::RTM_NEWNEIGHTBL),
            "RTM_GETNEIGHTBL" => Ok(NrMsgType::RTM_GETNEIGHTBL),
            "RTM_SETNEIGHTBL" => Ok(NrMsgType::RTM_SETNEIGHTBL),
            "RTM_NEWNDUSEROPT" => Ok(NrMsgType::RTM_NEWNDUSEROPT),
            "RTM_NEWADDRLABEL" => Ok(NrMsgType::RTM_NEWADDRLABEL),
            "RTM_DELADDRLABEL" => Ok(NrMsgType::RTM_DELADDRLABEL),
            "RTM_GETADDRLABEL" => Ok(NrMsgType::RTM_GETADDRLABEL),
            "RTM_GETDCB" => Ok(NrMsgType::RTM_GETDCB),
            "RTM_SETDCB" => Ok(NrMsgType::RTM_SETDCB),
            "RTM_NEWNETCONF" => Ok(NrMsgType::RTM_NEWNETCONF),
            "RTM_GETNETCONF" => Ok(NrMsgType::RTM_GETNETCONF),
            "RTM_NEWMDB" => Ok(NrMsgType::RTM_NEWMDB),
            "RTM_DELMDB" => Ok(NrMsgType::RTM_DELMDB),
            "RTM_GETMDB" => Ok(NrMsgType::RTM_GETMDB),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NrMsgType {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NrMsgType::RTM_NEWLINK => write!(f, "RTM_NEWLINK"),
            NrMsgType::RTM_DELLINK => write!(f, "RTM_DELLINK"),
            NrMsgType::RTM_GETLINK => write!(f, "RTM_GETLINK"),
            NrMsgType::RTM_SETLINK => write!(f, "RTM_SETLINK"),
            NrMsgType::RTM_NEWADDR => write!(f, "RTM_NEWADDR"),
            NrMsgType::RTM_DELADDR => write!(f, "RTM_DELADDR"),
            NrMsgType::RTM_GETADDR => write!(f, "RTM_GETADDR"),
            NrMsgType::RTM_NEWROUTE => write!(f, "RTM_NEWROUTE"),
            NrMsgType::RTM_DELROUTE => write!(f, "RTM_DELROUTE"),
            NrMsgType::RTM_GETROUTE => write!(f, "RTM_GETROUTE"),
            NrMsgType::RTM_NEWNEIGH => write!(f, "RTM_NEWNEIGH"),
            NrMsgType::RTM_DELNEIGH => write!(f, "RTM_DELNEIGH"),
            NrMsgType::RTM_GETNEIGH => write!(f, "RTM_GETNEIGH"),
            NrMsgType::RTM_NEWRULE => write!(f, "RTM_NEWRULE"),
            NrMsgType::RTM_DELRULE => write!(f, "RTM_DELRULE"),
            NrMsgType::RTM_GETRULE => write!(f, "RTM_GETRULE"),
            NrMsgType::RTM_NEWQDISC => write!(f, "RTM_NEWQDISC"),
            NrMsgType::RTM_DELQDISC => write!(f, "RTM_DELQDISC"),
            NrMsgType::RTM_GETQDISC => write!(f, "RTM_GETQDISC"),
            NrMsgType::RTM_NEWTCLASS => write!(f, "RTM_NEWTCLASS"),
            NrMsgType::RTM_DELTCLASS => write!(f, "RTM_DELTCLASS"),
            NrMsgType::RTM_GETTCLASS => write!(f, "RTM_GETTCLASS"),
            NrMsgType::RTM_NEWTFILTER => write!(f, "RTM_NEWTFILTER"),
            NrMsgType::RTM_DELTFILTER => write!(f, "RTM_DELTFILTER"),
            NrMsgType::RTM_GETTFILTER => write!(f, "RTM_GETTFILTER"),
            NrMsgType::RTM_NEWACTION => write!(f, "RTM_NEWACTION"),
            NrMsgType::RTM_DELACTION => write!(f, "RTM_DELACTION"),
            NrMsgType::RTM_GETACTION => write!(f, "RTM_GETACTION"),
            NrMsgType::RTM_NEWPREFIX => write!(f, "RTM_NEWPREFIX"),
            NrMsgType::RTM_GETMULTICAST => write!(f, "RTM_GETMULTICAST"),
            NrMsgType::RTM_GETANYCAST => write!(f, "RTM_GETANYCAST"),
            NrMsgType::RTM_NEWNEIGHTBL => write!(f, "RTM_NEWNEIGHTBL"),
            NrMsgType::RTM_GETNEIGHTBL => write!(f, "RTM_GETNEIGHTBL"),
            NrMsgType::RTM_SETNEIGHTBL => write!(f, "RTM_SETNEIGHTBL"),
            NrMsgType::RTM_NEWNDUSEROPT => write!(f, "RTM_NEWNDUSEROPT"),
            NrMsgType::RTM_NEWADDRLABEL => write!(f, "RTM_NEWADDRLABEL"),
            NrMsgType::RTM_DELADDRLABEL => write!(f, "RTM_DELADDRLABEL"),
            NrMsgType::RTM_GETADDRLABEL => write!(f, "RTM_GETADDRLABEL"),
            NrMsgType::RTM_GETDCB => write!(f, "RTM_GETDCB"),
            NrMsgType::RTM_SETDCB => write!(f, "RTM_SETDCB"),
            NrMsgType::RTM_NEWNETCONF => write!(f, "RTM_NEWNETCONF"),
            NrMsgType::RTM_GETNETCONF => write!(f, "RTM_GETNETCONF"),
            NrMsgType::RTM_NEWMDB => write!(f, "RTM_NEWMDB"),
            NrMsgType::RTM_DELMDB => write!(f, "RTM_DELMDB"),
            NrMsgType::RTM_GETMDB => write!(f, "RTM_GETMDB"),
        }
    }
}
impl ::num::traits::FromPrimitive for NrMsgType {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            16 => Some(NrMsgType::RTM_NEWLINK),
            17 => Some(NrMsgType::RTM_DELLINK),
            18 => Some(NrMsgType::RTM_GETLINK),
            19 => Some(NrMsgType::RTM_SETLINK),
            20 => Some(NrMsgType::RTM_NEWADDR),
            21 => Some(NrMsgType::RTM_DELADDR),
            22 => Some(NrMsgType::RTM_GETADDR),
            24 => Some(NrMsgType::RTM_NEWROUTE),
            25 => Some(NrMsgType::RTM_DELROUTE),
            26 => Some(NrMsgType::RTM_GETROUTE),
            28 => Some(NrMsgType::RTM_NEWNEIGH),
            29 => Some(NrMsgType::RTM_DELNEIGH),
            30 => Some(NrMsgType::RTM_GETNEIGH),
            32 => Some(NrMsgType::RTM_NEWRULE),
            33 => Some(NrMsgType::RTM_DELRULE),
            34 => Some(NrMsgType::RTM_GETRULE),
            36 => Some(NrMsgType::RTM_NEWQDISC),
            37 => Some(NrMsgType::RTM_DELQDISC),
            38 => Some(NrMsgType::RTM_GETQDISC),
            40 => Some(NrMsgType::RTM_NEWTCLASS),
            41 => Some(NrMsgType::RTM_DELTCLASS),
            42 => Some(NrMsgType::RTM_GETTCLASS),
            44 => Some(NrMsgType::RTM_NEWTFILTER),
            45 => Some(NrMsgType::RTM_DELTFILTER),
            46 => Some(NrMsgType::RTM_GETTFILTER),
            48 => Some(NrMsgType::RTM_NEWACTION),
            49 => Some(NrMsgType::RTM_DELACTION),
            50 => Some(NrMsgType::RTM_GETACTION),
            52 => Some(NrMsgType::RTM_NEWPREFIX),
            58 => Some(NrMsgType::RTM_GETMULTICAST),
            62 => Some(NrMsgType::RTM_GETANYCAST),
            64 => Some(NrMsgType::RTM_NEWNEIGHTBL),
            66 => Some(NrMsgType::RTM_GETNEIGHTBL),
            67 => Some(NrMsgType::RTM_SETNEIGHTBL),
            68 => Some(NrMsgType::RTM_NEWNDUSEROPT),
            72 => Some(NrMsgType::RTM_NEWADDRLABEL),
            73 => Some(NrMsgType::RTM_DELADDRLABEL),
            74 => Some(NrMsgType::RTM_GETADDRLABEL),
            78 => Some(NrMsgType::RTM_GETDCB),
            79 => Some(NrMsgType::RTM_SETDCB),
            80 => Some(NrMsgType::RTM_NEWNETCONF),
            82 => Some(NrMsgType::RTM_GETNETCONF),
            84 => Some(NrMsgType::RTM_NEWMDB),
            85 => Some(NrMsgType::RTM_DELMDB),
            86 => Some(NrMsgType::RTM_GETMDB),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            16 => Some(NrMsgType::RTM_NEWLINK),
            17 => Some(NrMsgType::RTM_DELLINK),
            18 => Some(NrMsgType::RTM_GETLINK),
            19 => Some(NrMsgType::RTM_SETLINK),
            20 => Some(NrMsgType::RTM_NEWADDR),
            21 => Some(NrMsgType::RTM_DELADDR),
            22 => Some(NrMsgType::RTM_GETADDR),
            24 => Some(NrMsgType::RTM_NEWROUTE),
            25 => Some(NrMsgType::RTM_DELROUTE),
            26 => Some(NrMsgType::RTM_GETROUTE),
            28 => Some(NrMsgType::RTM_NEWNEIGH),
            29 => Some(NrMsgType::RTM_DELNEIGH),
            30 => Some(NrMsgType::RTM_GETNEIGH),
            32 => Some(NrMsgType::RTM_NEWRULE),
            33 => Some(NrMsgType::RTM_DELRULE),
            34 => Some(NrMsgType::RTM_GETRULE),
            36 => Some(NrMsgType::RTM_NEWQDISC),
            37 => Some(NrMsgType::RTM_DELQDISC),
            38 => Some(NrMsgType::RTM_GETQDISC),
            40 => Some(NrMsgType::RTM_NEWTCLASS),
            41 => Some(NrMsgType::RTM_DELTCLASS),
            42 => Some(NrMsgType::RTM_GETTCLASS),
            44 => Some(NrMsgType::RTM_NEWTFILTER),
            45 => Some(NrMsgType::RTM_DELTFILTER),
            46 => Some(NrMsgType::RTM_GETTFILTER),
            48 => Some(NrMsgType::RTM_NEWACTION),
            49 => Some(NrMsgType::RTM_DELACTION),
            50 => Some(NrMsgType::RTM_GETACTION),
            52 => Some(NrMsgType::RTM_NEWPREFIX),
            58 => Some(NrMsgType::RTM_GETMULTICAST),
            62 => Some(NrMsgType::RTM_GETANYCAST),
            64 => Some(NrMsgType::RTM_NEWNEIGHTBL),
            66 => Some(NrMsgType::RTM_GETNEIGHTBL),
            67 => Some(NrMsgType::RTM_SETNEIGHTBL),
            68 => Some(NrMsgType::RTM_NEWNDUSEROPT),
            72 => Some(NrMsgType::RTM_NEWADDRLABEL),
            73 => Some(NrMsgType::RTM_DELADDRLABEL),
            74 => Some(NrMsgType::RTM_GETADDRLABEL),
            78 => Some(NrMsgType::RTM_GETDCB),
            79 => Some(NrMsgType::RTM_SETDCB),
            80 => Some(NrMsgType::RTM_NEWNETCONF),
            82 => Some(NrMsgType::RTM_GETNETCONF),
            84 => Some(NrMsgType::RTM_NEWMDB),
            85 => Some(NrMsgType::RTM_DELMDB),
            86 => Some(NrMsgType::RTM_GETMDB),
            _ => None
        }
    }
}
// TODO: revisit. Does this make sense?
impl Default for NrMsgType {
    fn default() -> NrMsgType {
        NrMsgType::RTM_NEWLINK
    }
}
