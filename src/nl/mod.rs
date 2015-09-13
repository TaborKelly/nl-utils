#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod netlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod genetlink;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod nl80211;

use ::std::io::Cursor;
use ::byteorder::{BigEndian, NativeEndian, ReadBytesExt};
use ::std::fmt;

use ::num::FromPrimitive;

#[derive(Debug)]
#[derive(Default)]
pub struct Nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
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

// Netlink header is native endian
pub fn read_header(data: &[u8]) -> Nlmsghdr {
    let mut s = Nlmsghdr::default();
    let mut cursor = Cursor::new(data);

    s.nlmsg_len = cursor.read_u32::<NativeEndian>().unwrap();
    s.nlmsg_type = cursor.read_u16::<NativeEndian>().unwrap();
    s.nlmsg_flags = cursor.read_u16::<NativeEndian>().unwrap();
    s.nlmsg_seq = cursor.read_u32::<NativeEndian>().unwrap();
    s.nlmsg_pid = cursor.read_u32::<NativeEndian>().unwrap();
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

pub fn read_cooked_header(data: &[u8]) -> CookedHeader {
    let mut c = CookedHeader::default();
    let mut cursor = Cursor::new(data);

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

#[test]
fn test_read_header() {
    let raw_data = [0u8, 4, 3, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 36, 0,
                    0, 0, 26, 0, 5, 3, 89, 7, 185, 85, 249, 2, 128, 0, 32, 0,
                    0, 0, 8, 0, 3, 0, 2, 0, 0, 0, 8, 0, 1, 0, 0, 0, 0, 0];
    let h = read_header(&raw_data[COOKED_HEADER_SIZE ..]);
    println!("h = {:?}", h);

    assert!(h.nlmsg_len == 36);
    assert!(h.nlmsg_type == 26);
    assert!(h.nlmsg_flags == 773);
    assert!(h.nlmsg_seq == 1438189401);
    assert!(h.nlmsg_pid == 8389369);
}

#[test]
fn test_read_cooked_header() {
    let raw_data = [0u8, 4, 3, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 36, 0,
                    0, 0, 26, 0, 5, 3, 89, 7, 185, 85, 249, 2, 128, 0, 32, 0,
                    0, 0, 8, 0, 3, 0, 2, 0, 0, 0, 8, 0, 1, 0, 0, 0, 0, 0];
    let h = read_cooked_header(&raw_data);
    println!("h = {:?}", h);

    assert!(h.header_type == 4);
    assert!(h.arphdr_type == 824);
    assert!(h.address_length == 0);
    for a in h.address.iter() {
        assert!(*a == 0);
    }
    assert!(h.netlink_family == netlink::NetlinkFamily::NETLINK_GENERIC);
}
