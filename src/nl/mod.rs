#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod netlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod rtnetlink;

/* TODO:
 - nda_cacheinfo
 - tcmsg
 - attributes
 - multiple message bodies per packet
*/

use ::std;
use ::std::io::{Cursor, Seek, SeekFrom};
use ::byteorder::{BigEndian, NativeEndian, ReadBytesExt};
use ::std::fmt;

use ::num::FromPrimitive;

// TODO: implement custom formatters for indented output
// &mut std::io::Cursor<&[u8]>
fn get_size(cursor: &mut std::io::Cursor<&[u8]>) -> u64 {
    let pos = cursor.position();
    let end = cursor.seek(SeekFrom::End(0));
    cursor.set_position(pos);
    end.unwrap()
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
        write!(f, "{{\n\theader_type: {},\n\tarphdr_type: {},\n\t\
               address_length: {}\n\taddress = [",
               self.header_type, self.arphdr_type,
               self.address_length).unwrap();
        let mut count: usize = 1;
        for a in self.address.iter() {
            write!(f, " {}", a).unwrap();
            if count < self.address.len() {
                write!(f, ",").unwrap();
            }
            count = count + 1;
        }
        write!(f, " ],\n\tnetlink_family: {}\n}}", self.netlink_family)
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
        write!(f, "{{\n\tnlmsg_len: {},\n\tnlmsg_type: {},\n\t\
               nlmsg_flags: {:#x},\n\tnlmsg_seq: {},\n\tnlmsg_pid: {}\n}}",
               self.nlmsg_len, self.nlmsg_type, self.nlmsg_flags,
               self.nlmsg_seq, self.nlmsg_pid)
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
}

// TODO: revisti name... NlMsgBody?
#[derive(Debug, Copy, Clone)]
pub enum NlMsgEnum {
    None, // no body expected
    Unsupported, // we don't support this body type
    MalfromedPacket, // the packet was malformed
    Ifinfomsg(rtnetlink::Ifinfomsg),
    Ifaddrmsg(rtnetlink::Ifaddrmsg),
    Rtmsg(rtnetlink::Rtmsg),
    Ndmsg(rtnetlink::Ndmsg),
}
impl NlMsgEnum {
    // Netlink header is native endian
    pub fn read(cursor: &mut std::io::Cursor<&[u8]>,
               nlmsg_type: NlMsgTypeEnum) -> NlMsgEnum {

        match nlmsg_type {
            NlMsgTypeEnum::NrMsgType(ref u) => {
                if *u == rtnetlink::NrMsgType::RTM_NEWLINK ||
                   *u == rtnetlink::NrMsgType::RTM_DELLINK ||
                   *u == rtnetlink::NrMsgType::RTM_GETLINK {
                    let o = rtnetlink::Ifinfomsg::read(cursor);
                    match o {
                        Some(msg) => NlMsgEnum::Ifinfomsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWADDR ||
                   *u == rtnetlink::NrMsgType::RTM_DELADDR ||
                   *u == rtnetlink::NrMsgType::RTM_GETADDR {
                    let o = rtnetlink::Ifaddrmsg::read(cursor);
                    match o {
                        Some(msg) => NlMsgEnum::Ifaddrmsg(msg),
                        None => NlMsgEnum::MalfromedPacket
                    }
                }
                else if *u == rtnetlink::NrMsgType::RTM_NEWROUTE ||
                   *u == rtnetlink::NrMsgType::RTM_DELROUTE ||
                   *u == rtnetlink::NrMsgType::RTM_GETROUTE {
                    let o = rtnetlink::Rtmsg::read(cursor);
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
                else {
                    NlMsgEnum::default()
                }
            },
            NlMsgTypeEnum::NlMsgType(_) => NlMsgEnum::None,
            _ => NlMsgEnum::default()
        }
    }
}
impl ::std::fmt::Display for NlMsgEnum {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NlMsgEnum::None => write!(f, "None"),
            NlMsgEnum::Unsupported => write!(f, "Unsupported"),
            NlMsgEnum::MalfromedPacket => write!(f, "MalfromedPacket"),
            NlMsgEnum::Ifinfomsg(ref u) => write!(f, "Ifinfomsg({})", u),
            NlMsgEnum::Ifaddrmsg(ref u) => write!(f, "Ifaddrmsg({})", u),
            NlMsgEnum::Rtmsg(ref u) => write!(f, "Rtmsg({})", u),
            NlMsgEnum::Ndmsg(ref u) => write!(f, "Ndmsg({})", u),
        }
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
        nlmsg.nlmsg = NlMsgEnum::read(&mut cursor, nlmsg.nlmsghdr.nlmsg_type);
        nlmsg
    }
    /// This function lets you align the cursor to the next NLMSG_ALIGNTO (4)
    /// byte boundry.
    fn nlmsg_align(cursor: &mut std::io::Cursor<&[u8]>) {
        let mut pos = cursor.position();
        pos = ((pos)+NLMSG_ALIGNTO-1) & !(NLMSG_ALIGNTO-1);
        cursor.set_position(pos);
    }
}
impl fmt::Display for NlMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n\tnetlink_family: ").unwrap();
        self.netlink_family.fmt(f).unwrap();
        write!(f, "\n\tnlmsghdr: ").unwrap();
        self.nlmsghdr.fmt(f).unwrap();
        write!(f, "\n\tnlmsg: ").unwrap();
        self.nlmsg.fmt(f).unwrap();
        write!(f, "}}\n")
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
