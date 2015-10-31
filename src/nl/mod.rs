#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod netlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod rtnetlink;

/* TODO:
 - nlmsg_flags
 - attributes
 - multiple message bodies per packet
 - ',' consistency in output
*/

use ::std;
use ::std::io::{Cursor, Seek, SeekFrom};
use ::byteorder::{BigEndian, NativeEndian, ReadBytesExt};
use ::std::fmt;

use ::num::FromPrimitive;

// given a cursor, this will tell you how big it is
fn get_size(cursor: &mut std::io::Cursor<&[u8]>) -> u64 {
    let pos = cursor.position();
    let end = cursor.seek(SeekFrom::End(0));
    cursor.set_position(pos);
    end.unwrap()
}

fn format_indent(indent: i32) -> String {
    let mut s = String::new();
    for _ in 0..indent {
        s.push_str("    ");
    }
    s
}

// Cooked SLL header is big endian (network byte order)
// http://www.tcpdump.org/linktypes/LINKTYPE_LINUX_SLL.html
#[derive(Debug)]
pub struct CookedHeader {
    header_type: u16,
    arphdr_type: u16,
    address_length: u16,
    address: [u8; 8],
    netlink_family: netlink::NetlinkFamily, // NETLINK_ROUTE .. NETLINK_INET_DIAG
}
pub const COOKED_HEADER_SIZE: usize = 16;
impl Default for CookedHeader {
    fn default() -> CookedHeader {
        CookedHeader { header_type: 0,
                      arphdr_type: 0,
                      address_length: 0,
                      address: [0; 8],
                      netlink_family: netlink::NetlinkFamily::NETLINK_ROUTE }
    }
}
impl fmt::Display for CookedHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_fmt(f, 0)
    }
}
impl CookedHeader {
    // Netlink header is native endian
    pub fn read(cursor: &mut std::io::Cursor<&[u8]>) -> CookedHeader {
        let mut c = CookedHeader::default();

        c.header_type = cursor.read_u16::<BigEndian>().unwrap();
        c.arphdr_type = cursor.read_u16::<BigEndian>().unwrap();
        c.address_length = cursor.read_u16::<BigEndian>().unwrap();
        for a in c.address.iter_mut() {
            *a = cursor.read_u8().unwrap();
        }
        let family = cursor.read_u16::<BigEndian>().unwrap();
        c.netlink_family = netlink::NetlinkFamily::from_u16(family).unwrap();
        assert!(cursor.position() as usize == COOKED_HEADER_SIZE);

        c
    }
    fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let indent = format_indent(indent);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    header_type: {},\n", indent, self.header_type));
        try!(write!(f, "{}    arphdr_type: {},\n", indent, self.arphdr_type));
        try!(write!(f, "{}    address_length: {},\n", indent, self.address_length));
        try!(write!(f, "{}    address = [", indent));
        let mut count: usize = 1;
        for a in self.address.iter() {
            try!(write!(f, " {}", a));
            if count < self.address.len() {
                try!(write!(f, ","));
            }
            count = count + 1;
        }
        try!(write!(f, " ]\n{}    netlink_family: {},\n", indent, self.netlink_family));
        write!(f, "{}}}", indent)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NlMsgTypeEnum {
    Raw(u16),
    NlMsgType(netlink::NlMsgType),
    NrMsgType(rtnetlink::NrMsgType),
}
impl ::std::fmt::Display for NlMsgTypeEnum {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NlMsgTypeEnum::Raw(ref u) => write!(f, "Raw({})", u),
            NlMsgTypeEnum::NlMsgType(ref u) => write!(f, "NlMsgType({})", u),
            NlMsgTypeEnum::NrMsgType(ref u) => write!(f, "NrMsgType({})", u),
        }
    }
}
impl Default for NlMsgTypeEnum {
    fn default() -> NlMsgTypeEnum {
        NlMsgTypeEnum::Raw(0)
    }
}

#[derive(Debug, Default)]
pub struct Nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: NlMsgTypeEnum,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}
impl fmt::Display for Nlmsghdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_fmt(f, 0)
    }
}
impl Nlmsghdr {
    // Netlink header is native endian
    pub fn read(cursor: &mut std::io::Cursor<&[u8]>,
               family: netlink::NetlinkFamily) -> Nlmsghdr {
        let mut s = Nlmsghdr::default();

        s.nlmsg_len = cursor.read_u32::<NativeEndian>().unwrap();
        let nlmsg_type = cursor.read_u16::<NativeEndian>().unwrap();
        s.nlmsg_type = match nlmsg_type {
            // TODO: revisit magic numbers
            1 ... 4 => NlMsgTypeEnum::NlMsgType(netlink::NlMsgType::from_u64(nlmsg_type as u64).unwrap()),
            _ => match family {
                // TODO: revisit syntax
                netlink::NetlinkFamily::NETLINK_ROUTE => NlMsgTypeEnum::NrMsgType(rtnetlink::NrMsgType::from_u64(nlmsg_type as u64).unwrap()),
                _ => NlMsgTypeEnum::Raw(nlmsg_type),
            }
        };
        s.nlmsg_flags = cursor.read_u16::<NativeEndian>().unwrap();
        s.nlmsg_seq = cursor.read_u32::<NativeEndian>().unwrap();
        s.nlmsg_pid = cursor.read_u32::<NativeEndian>().unwrap();
        s
    }
    fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let indent = format_indent(indent);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    nlmsg_len: {},\n", indent, self.nlmsg_len));
        try!(write!(f, "{}    nlmsg_type: {},\n", indent, self.nlmsg_type));
        try!(write!(f, "{}    nlmsg_flags: {:#X},\n", indent, self.nlmsg_flags));
        try!(write!(f, "{}    nlmsg_seq: {},\n", indent, self.nlmsg_seq));
        try!(write!(f, "{}    nlmsg_pid: {},\n", indent, self.nlmsg_pid));
        write!(f, "{}}}", indent)
    }
}

// TODO: revisti name... NlMsgBody?
#[derive(Debug, Clone)]
pub enum NlMsgEnum {
    None, // no body expected
    Unsupported, // we don't support this body type
    MalfromedPacket, // the packet was malformed
    Ifinfomsg(rtnetlink::Ifinfomsg),
    Ifaddrmsg(rtnetlink::Ifaddrmsg),
    Rtmsg(rtnetlink::Rtmsg),
    Ndmsg(rtnetlink::Ndmsg),
    Tcmsg(rtnetlink::Tcmsg),
}
impl NlMsgEnum {
    // Netlink header is native endian
    pub fn read(cursor: &mut std::io::Cursor<&[u8]>,
               nlmsg_type: NlMsgTypeEnum,
               nlmsg_len: usize) -> NlMsgEnum {

        match nlmsg_type {
            NlMsgTypeEnum::NrMsgType(ref u) => {
                if *u == rtnetlink::NrMsgType::RTM_NEWLINK ||
                   *u == rtnetlink::NrMsgType::RTM_DELLINK ||
                   *u == rtnetlink::NrMsgType::RTM_GETLINK {
                    let o = rtnetlink::Ifinfomsg::read(cursor, nlmsg_len);
                    match o {
                        Some(msg) => NlMsgEnum::Ifinfomsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWADDR ||
                   *u == rtnetlink::NrMsgType::RTM_DELADDR ||
                   *u == rtnetlink::NrMsgType::RTM_GETADDR {
                    let o = rtnetlink::Ifaddrmsg::read(cursor, nlmsg_len);
                    match o {
                        Some(msg) => NlMsgEnum::Ifaddrmsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWROUTE ||
                   *u == rtnetlink::NrMsgType::RTM_DELROUTE ||
                   *u == rtnetlink::NrMsgType::RTM_GETROUTE {
                    let o = rtnetlink::Rtmsg::read(cursor, nlmsg_len);
                    match o {
                        Some(msg) => NlMsgEnum::Rtmsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWNEIGH ||
                   *u == rtnetlink::NrMsgType::RTM_DELNEIGH ||
                   *u == rtnetlink::NrMsgType::RTM_GETNEIGH {
                    let o = rtnetlink::Ndmsg::read(cursor);
                    match o {
                        Some(msg) => NlMsgEnum::Ndmsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWQDISC ||
                   *u == rtnetlink::NrMsgType::RTM_DELQDISC ||
                   *u == rtnetlink::NrMsgType::RTM_GETQDISC ||
                   *u == rtnetlink::NrMsgType::RTM_NEWTCLASS ||
                   *u == rtnetlink::NrMsgType::RTM_DELTCLASS ||
                   *u == rtnetlink::NrMsgType::RTM_GETTCLASS ||
                   *u == rtnetlink::NrMsgType::RTM_NEWTFILTER ||
                   *u == rtnetlink::NrMsgType::RTM_DELTFILTER ||
                   *u == rtnetlink::NrMsgType::RTM_GETTFILTER {
                    let o = rtnetlink::Tcmsg::read(cursor);
                    match o {
                        Some(msg) => NlMsgEnum::Tcmsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else {
                    NlMsgEnum::default()
                }
            },
            NlMsgTypeEnum::NlMsgType(_) => NlMsgEnum::None,
            _ => NlMsgEnum::default()
        }
    }
    fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        // Take care of the simple cases first
        match *self {
            NlMsgEnum::None => return write!(f, "None"),
            NlMsgEnum::Unsupported => return write!(f, "Unsupported"),
            NlMsgEnum::MalfromedPacket => return write!(f, "MalfromedPacket"),
            _ => {},
        }

        match *self {
            NlMsgEnum::Ifinfomsg(ref u) => {
                try!(write!(f, "Ifinfomsg("));
                try!(u.pretty_fmt(f, indent+1));
            }
            NlMsgEnum::Ifaddrmsg(ref u) => {
                try!(write!(f, "Ifaddrmsg("));
                try!(u.pretty_fmt(f, indent+1));
            }
            NlMsgEnum::Rtmsg(ref u) => {
                try!(write!(f, "Rtmsg("));
                try!(u.pretty_fmt(f, indent+1));
            }
            NlMsgEnum::Ndmsg(ref u) => {
                try!(write!(f, "Ndmsg("));
                try!(u.pretty_fmt(f, indent+1));
            }
            NlMsgEnum::Tcmsg(ref u) => {
                try!(write!(f, "Tcmsg("));
                try!(u.pretty_fmt(f, indent+1));
            }
            _ => {},
        }
        write!(f, ")")
    }
}
impl ::std::fmt::Display for NlMsgEnum {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}
impl Default for NlMsgEnum {
    fn default() -> NlMsgEnum {
        NlMsgEnum::Unsupported
    }
}

// I would prefer to use an associated constant but they are still experimental
const NLMSG_ALIGNTO: u64 = 4;

// TODO: names
#[derive(Debug, Default)]
pub struct NlMsg {
    pub netlink_family: netlink::NetlinkFamily,
    pub nlmsghdr: Nlmsghdr,
    pub nlmsg: NlMsgEnum,
}
impl NlMsg
{
    pub fn read(data: &[u8]) -> NlMsg {
        let mut nlmsg = NlMsg::default();
        let mut cursor = Cursor::new(data);
        let cookedheader = CookedHeader::read(&mut cursor);
        nlmsg.netlink_family = cookedheader.netlink_family;
        nlmsg.nlmsghdr = Nlmsghdr::read(&mut cursor, cookedheader.netlink_family);
        nlmsg.nlmsg = NlMsgEnum::read(&mut cursor, nlmsg.nlmsghdr.nlmsg_type,
                                      nlmsg.nlmsghdr.nlmsg_len as usize);
        nlmsg
    }
    /// This function lets you align the cursor to the next NLMSG_ALIGNTO (4)
    /// byte boundry.
    fn nlmsg_align(cursor: &mut std::io::Cursor<&[u8]>) {
        let mut pos = cursor.position();
        pos = ((pos)+NLMSG_ALIGNTO-1) & !(NLMSG_ALIGNTO-1);
        cursor.set_position(pos);
    }
    fn pretty_fmt(&self, f: &mut fmt::Formatter, indent: i32) -> fmt::Result {
        let i_s = format_indent(indent);
        try!(write!(f, "{{\n"));
        try!(write!(f, "{}    netlink_family: {}\n", i_s, self.netlink_family));
        try!(write!(f, "{}    nlmsghdr: ", i_s));
        try!(self.nlmsghdr.pretty_fmt(f, indent+1));
        try!(write!(f, "\n{}    nlmsg: {}\n", i_s, self.nlmsg));
        write!(f, "}}")
    }
}
impl fmt::Display for NlMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[test]
fn test_cookedheader_read() {
    let raw_data = [0u8, 4, 3, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 36, 0,
                    0, 0, 26, 0, 5, 3, 89, 7, 185, 85, 249, 2, 128, 0, 32, 0,
                    0, 0, 8, 0, 3, 0, 2, 0, 0, 0, 8, 0, 1, 0, 0, 0, 0, 0];
    let mut cursor = Cursor::new(&raw_data as &[u8]);
    let h = CookedHeader::read(&mut cursor);
    println!("h = {:?}", h);

    assert!(h.header_type == 4);
    assert!(h.arphdr_type == 824);
    assert!(h.address_length == 0);
    for a in h.address.iter() {
        assert!(*a == 0);
    }
    assert!(h.netlink_family == netlink::NetlinkFamily::NETLINK_GENERIC);
}

#[test]
fn test_nlmsghdr_read() {
    let raw_data = [0u8, 4, 3, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 36, 0,
                    0, 0, 26, 0, 5, 3, 89, 7, 185, 85, 249, 2, 128, 0, 32, 0,
                    0, 0, 8, 0, 3, 0, 2, 0, 0, 0, 8, 0, 1, 0, 0, 0, 0, 0];
    let mut cursor = Cursor::new(&raw_data[COOKED_HEADER_SIZE ..] as &[u8]);
    let h = Nlmsghdr::read(&mut cursor, netlink::NetlinkFamily::NETLINK_GENERIC);
    println!("h = {:?}", h);

    assert!(h.nlmsg_len == 36);
    assert!(h.nlmsg_type == NlMsgTypeEnum::Raw(26));
    assert!(h.nlmsg_flags == 773);
    assert!(h.nlmsg_seq == 1438189401);
    assert!(h.nlmsg_pid == 8389369);
}

#[test]
fn test_NlMsg_read() {
    let raw_data = [0u8, 4, 3, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 36, 0,
                    0, 0, 26, 0, 5, 3, 89, 7, 185, 85, 249, 2, 128, 0, 32, 0,
                    0, 0, 8, 0, 3, 0, 2, 0, 0, 0, 8, 0, 1, 0, 0, 0, 0, 0];
    let msg = NlMsg::read(&raw_data);
    println!("msg = {:?}", msg);

    assert!(msg.netlink_family == netlink::NetlinkFamily::NETLINK_GENERIC);
    assert!(msg.nlmsghdr.nlmsg_len == 36);
    assert!(msg.nlmsghdr.nlmsg_type == NlMsgTypeEnum::Raw(26));
    assert!(msg.nlmsghdr.nlmsg_flags == 773);
    assert!(msg.nlmsghdr.nlmsg_seq == 1438189401);
    assert!(msg.nlmsghdr.nlmsg_pid == 8389369);
}

#[test]
fn test_NlMsg_nlmsg_align() {
    let raw_data = [0u8, 0, 0, 0, 0, 0, 0, 0];
    let mut cursor = Cursor::new(&raw_data as &[u8]);

    assert!(cursor.position() == 0);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 0);

    cursor.set_position(1);
    assert!(cursor.position() == 1);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 4);

    cursor.set_position(4);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 4);

    cursor.set_position(5);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 8);

    cursor.set_position(6);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 8);

    cursor.set_position(7);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 8);

    cursor.set_position(8);
    NlMsg::nlmsg_align(&mut cursor);
    assert!(cursor.position() == 8);
}
