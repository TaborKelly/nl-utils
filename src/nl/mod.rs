#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod netlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub mod rtnetlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod genetlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod nl80211;

/* TODO:
 - consistant derives for all enums
 - ifinfomsg
 - ifaddrmsg
 - rtmsg
 - ndmsg
 - nda_cacheinfo
 - tcmsg

  Padding
*/

use ::std;
use ::std::io::Cursor;
use ::byteorder::{BigEndian, NativeEndian, ReadBytesExt};
use ::std::fmt;

use ::num::FromPrimitive;

// TODO: implement custom formatters for indented output

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

#[derive(Debug)]
#[derive(Default)]
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

#[derive(Debug)]
#[derive(Default)]
pub struct NlMsg {
    pub netlink_family: netlink::NetlinkFamily,
    pub nlmsghdr: Nlmsghdr,
}
impl NlMsg
{
    pub fn read(data: &[u8]) -> NlMsg {
        let mut nlmsg = NlMsg::default();
        let mut cursor = Cursor::new(data);
        let cookedheader = CookedHeader::read(&mut cursor);
        nlmsg.netlink_family = cookedheader.netlink_family;
        nlmsg.nlmsghdr = Nlmsghdr::read(&mut cursor, cookedheader.netlink_family);
        nlmsg
    }
}
impl fmt::Display for NlMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n\tnetlink_family: ").unwrap();
        self.netlink_family.fmt(f).unwrap();
        write!(f, "\n\tnlmsghdr: ").unwrap();
        self.nlmsghdr.fmt(f).unwrap();
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
