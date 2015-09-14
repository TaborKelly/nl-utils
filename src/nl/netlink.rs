use ::std::fmt;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
pub enum NetlinkFamily {
    NETLINK_ROUTE = 0,
    NETLINK_UNUSED = 1,
    NETLINK_USERSOCK = 2,
    NETLINK_FIREWALL = 3,
    NETLINK_SOCK_DIAG = 4,
    NETLINK_NFLOG = 5,
    NETLINK_XFRM = 6,
    NETLINK_SELINUX = 7,
    NETLINK_ISCSI = 8,
    NETLINK_AUDIT = 9,
    NETLINK_FIB_LOOKUP = 10,
    NETLINK_CONNECTOR = 11,
    NETLINK_NETFILTER = 12,
    NETLINK_IP6_FW = 13,
    NETLINK_DNRTMSG = 14,
    NETLINK_KOBJECT_UEVENT = 15,
    NETLINK_GENERIC = 16,
    /* leave room for NETLINK_DM (DM Events) */
    NETLINK_SCSITRANSPORT = 18,
    NETLINK_ECRYPTFS = 19,
    NETLINK_RDMA = 20,
    NETLINK_CRYPTO = 21,
}
} // enum_from_primitive
impl fmt::Display for NetlinkFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}
impl Default for NetlinkFamily {
    fn default() -> NetlinkFamily {
        NetlinkFamily::NETLINK_ROUTE
    }
}
