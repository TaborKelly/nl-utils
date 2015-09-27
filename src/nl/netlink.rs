#[allow(dead_code, non_camel_case_types)]
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
    NETLINK_SCSITRANSPORT = 18,
    NETLINK_ECRYPTFS = 19,
    NETLINK_RDMA = 20,
    NETLINK_CRYPTO = 21,
}
impl ::std::str::FromStr for NetlinkFamily {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NETLINK_ROUTE" => Ok(NetlinkFamily::NETLINK_ROUTE),
            "NETLINK_UNUSED" => Ok(NetlinkFamily::NETLINK_UNUSED),
            "NETLINK_USERSOCK" => Ok(NetlinkFamily::NETLINK_USERSOCK),
            "NETLINK_FIREWALL" => Ok(NetlinkFamily::NETLINK_FIREWALL),
            "NETLINK_SOCK_DIAG" => Ok(NetlinkFamily::NETLINK_SOCK_DIAG),
            "NETLINK_NFLOG" => Ok(NetlinkFamily::NETLINK_NFLOG),
            "NETLINK_XFRM" => Ok(NetlinkFamily::NETLINK_XFRM),
            "NETLINK_SELINUX" => Ok(NetlinkFamily::NETLINK_SELINUX),
            "NETLINK_ISCSI" => Ok(NetlinkFamily::NETLINK_ISCSI),
            "NETLINK_AUDIT" => Ok(NetlinkFamily::NETLINK_AUDIT),
            "NETLINK_FIB_LOOKUP" => Ok(NetlinkFamily::NETLINK_FIB_LOOKUP),
            "NETLINK_CONNECTOR" => Ok(NetlinkFamily::NETLINK_CONNECTOR),
            "NETLINK_NETFILTER" => Ok(NetlinkFamily::NETLINK_NETFILTER),
            "NETLINK_IP6_FW" => Ok(NetlinkFamily::NETLINK_IP6_FW),
            "NETLINK_DNRTMSG" => Ok(NetlinkFamily::NETLINK_DNRTMSG),
            "NETLINK_KOBJECT_UEVENT" => Ok(NetlinkFamily::NETLINK_KOBJECT_UEVENT),
            "NETLINK_GENERIC" => Ok(NetlinkFamily::NETLINK_GENERIC),
            "NETLINK_SCSITRANSPORT" => Ok(NetlinkFamily::NETLINK_SCSITRANSPORT),
            "NETLINK_ECRYPTFS" => Ok(NetlinkFamily::NETLINK_ECRYPTFS),
            "NETLINK_RDMA" => Ok(NetlinkFamily::NETLINK_RDMA),
            "NETLINK_CRYPTO" => Ok(NetlinkFamily::NETLINK_CRYPTO),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NetlinkFamily {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NetlinkFamily::NETLINK_ROUTE => write!(f, "NETLINK_ROUTE"),
            NetlinkFamily::NETLINK_UNUSED => write!(f, "NETLINK_UNUSED"),
            NetlinkFamily::NETLINK_USERSOCK => write!(f, "NETLINK_USERSOCK"),
            NetlinkFamily::NETLINK_FIREWALL => write!(f, "NETLINK_FIREWALL"),
            NetlinkFamily::NETLINK_SOCK_DIAG => write!(f, "NETLINK_SOCK_DIAG"),
            NetlinkFamily::NETLINK_NFLOG => write!(f, "NETLINK_NFLOG"),
            NetlinkFamily::NETLINK_XFRM => write!(f, "NETLINK_XFRM"),
            NetlinkFamily::NETLINK_SELINUX => write!(f, "NETLINK_SELINUX"),
            NetlinkFamily::NETLINK_ISCSI => write!(f, "NETLINK_ISCSI"),
            NetlinkFamily::NETLINK_AUDIT => write!(f, "NETLINK_AUDIT"),
            NetlinkFamily::NETLINK_FIB_LOOKUP => write!(f, "NETLINK_FIB_LOOKUP"),
            NetlinkFamily::NETLINK_CONNECTOR => write!(f, "NETLINK_CONNECTOR"),
            NetlinkFamily::NETLINK_NETFILTER => write!(f, "NETLINK_NETFILTER"),
            NetlinkFamily::NETLINK_IP6_FW => write!(f, "NETLINK_IP6_FW"),
            NetlinkFamily::NETLINK_DNRTMSG => write!(f, "NETLINK_DNRTMSG"),
            NetlinkFamily::NETLINK_KOBJECT_UEVENT => write!(f, "NETLINK_KOBJECT_UEVENT"),
            NetlinkFamily::NETLINK_GENERIC => write!(f, "NETLINK_GENERIC"),
            NetlinkFamily::NETLINK_SCSITRANSPORT => write!(f, "NETLINK_SCSITRANSPORT"),
            NetlinkFamily::NETLINK_ECRYPTFS => write!(f, "NETLINK_ECRYPTFS"),
            NetlinkFamily::NETLINK_RDMA => write!(f, "NETLINK_RDMA"),
            NetlinkFamily::NETLINK_CRYPTO => write!(f, "NETLINK_CRYPTO"),
        }
    }
}
impl ::num::traits::FromPrimitive for NetlinkFamily {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(NetlinkFamily::NETLINK_ROUTE),
            1 => Some(NetlinkFamily::NETLINK_UNUSED),
            2 => Some(NetlinkFamily::NETLINK_USERSOCK),
            3 => Some(NetlinkFamily::NETLINK_FIREWALL),
            4 => Some(NetlinkFamily::NETLINK_SOCK_DIAG),
            5 => Some(NetlinkFamily::NETLINK_NFLOG),
            6 => Some(NetlinkFamily::NETLINK_XFRM),
            7 => Some(NetlinkFamily::NETLINK_SELINUX),
            8 => Some(NetlinkFamily::NETLINK_ISCSI),
            9 => Some(NetlinkFamily::NETLINK_AUDIT),
            10 => Some(NetlinkFamily::NETLINK_FIB_LOOKUP),
            11 => Some(NetlinkFamily::NETLINK_CONNECTOR),
            12 => Some(NetlinkFamily::NETLINK_NETFILTER),
            13 => Some(NetlinkFamily::NETLINK_IP6_FW),
            14 => Some(NetlinkFamily::NETLINK_DNRTMSG),
            15 => Some(NetlinkFamily::NETLINK_KOBJECT_UEVENT),
            16 => Some(NetlinkFamily::NETLINK_GENERIC),
            18 => Some(NetlinkFamily::NETLINK_SCSITRANSPORT),
            19 => Some(NetlinkFamily::NETLINK_ECRYPTFS),
            20 => Some(NetlinkFamily::NETLINK_RDMA),
            21 => Some(NetlinkFamily::NETLINK_CRYPTO),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(NetlinkFamily::NETLINK_ROUTE),
            1 => Some(NetlinkFamily::NETLINK_UNUSED),
            2 => Some(NetlinkFamily::NETLINK_USERSOCK),
            3 => Some(NetlinkFamily::NETLINK_FIREWALL),
            4 => Some(NetlinkFamily::NETLINK_SOCK_DIAG),
            5 => Some(NetlinkFamily::NETLINK_NFLOG),
            6 => Some(NetlinkFamily::NETLINK_XFRM),
            7 => Some(NetlinkFamily::NETLINK_SELINUX),
            8 => Some(NetlinkFamily::NETLINK_ISCSI),
            9 => Some(NetlinkFamily::NETLINK_AUDIT),
            10 => Some(NetlinkFamily::NETLINK_FIB_LOOKUP),
            11 => Some(NetlinkFamily::NETLINK_CONNECTOR),
            12 => Some(NetlinkFamily::NETLINK_NETFILTER),
            13 => Some(NetlinkFamily::NETLINK_IP6_FW),
            14 => Some(NetlinkFamily::NETLINK_DNRTMSG),
            15 => Some(NetlinkFamily::NETLINK_KOBJECT_UEVENT),
            16 => Some(NetlinkFamily::NETLINK_GENERIC),
            18 => Some(NetlinkFamily::NETLINK_SCSITRANSPORT),
            19 => Some(NetlinkFamily::NETLINK_ECRYPTFS),
            20 => Some(NetlinkFamily::NETLINK_RDMA),
            21 => Some(NetlinkFamily::NETLINK_CRYPTO),
            _ => None
        }
    }
}
impl Default for NetlinkFamily {
    fn default() -> NetlinkFamily {
        NetlinkFamily::NETLINK_ROUTE
    }
}
