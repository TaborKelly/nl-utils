use ::std::io::{Cursor};
use ::byteorder::{NativeEndian, ReadBytesExt};
use ::num::FromPrimitive;
use ::std::fmt;
use nl::{format_indent, NlMsg};

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

// A macro to implement pretty_fmt() for an enum
// $t - The enum type
// $v - The largest value in the enum
// $m - The method to call to get the flag from a u32
// TODO: figure out how to eliminate $m and just use $t::from_u32().
//       or possilby replace it with generated code
macro_rules! impl_pretty_flag_fmt {
    ($t:path, $v:path, $m:path) => {
        impl $t {
            fn pretty_fmt(f: &mut ::std::fmt::Formatter, flags: u32) -> ::std::fmt::Result {
                let mut shift: u32 = 0;
                let mut result: u32 = 1<<shift;
                let mut found = false;
                while result <= $v as u32 {
                    let tmp = result & flags;
                    if tmp > 0 {
                        if found {
                            try!(write!(f, "|"));
                        }
                        let flag = $m(tmp).unwrap();
                        try!(write!(f, "{}", flag));
                        found = true;
                    }

                    // keep looking
                    shift += 1;
                    result = 1<<shift;
                }
                write!(f, "")
            }
        }
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NetDeviceFlags {
    IFF_UP = 0x1,
    IFF_BROADCAST = 0x2,
    IFF_DEBUG = 0x4,
    IFF_LOOPBACK = 0x8,
    IFF_POINTOPOINT = 0x10,
    IFF_NOTRAILERS = 0x20,
    IFF_RUNNING = 0x40,
    IFF_NOARP = 0x80,
    IFF_PROMISC = 0x100,
    IFF_ALLMULTI = 0x200,
    IFF_MASTER = 0x400,
    IFF_SLAVE = 0x800,
    IFF_MULTICAST = 0x1000,
    IFF_PORTSEL = 0x2000,
    IFF_AUTOMEDIA = 0x4000,
    IFF_DYNAMIC = 0x8000,
    IFF_LOWER_UP = 0x10000,
    IFF_DORMANT = 0x20000,
    IFF_ECHO = 0x40000,
}
impl ::std::str::FromStr for NetDeviceFlags {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IFF_UP" => Ok(NetDeviceFlags::IFF_UP),
            "IFF_BROADCAST" => Ok(NetDeviceFlags::IFF_BROADCAST),
            "IFF_DEBUG" => Ok(NetDeviceFlags::IFF_DEBUG),
            "IFF_LOOPBACK" => Ok(NetDeviceFlags::IFF_LOOPBACK),
            "IFF_POINTOPOINT" => Ok(NetDeviceFlags::IFF_POINTOPOINT),
            "IFF_NOTRAILERS" => Ok(NetDeviceFlags::IFF_NOTRAILERS),
            "IFF_RUNNING" => Ok(NetDeviceFlags::IFF_RUNNING),
            "IFF_NOARP" => Ok(NetDeviceFlags::IFF_NOARP),
            "IFF_PROMISC" => Ok(NetDeviceFlags::IFF_PROMISC),
            "IFF_ALLMULTI" => Ok(NetDeviceFlags::IFF_ALLMULTI),
            "IFF_MASTER" => Ok(NetDeviceFlags::IFF_MASTER),
            "IFF_SLAVE" => Ok(NetDeviceFlags::IFF_SLAVE),
            "IFF_MULTICAST" => Ok(NetDeviceFlags::IFF_MULTICAST),
            "IFF_PORTSEL" => Ok(NetDeviceFlags::IFF_PORTSEL),
            "IFF_AUTOMEDIA" => Ok(NetDeviceFlags::IFF_AUTOMEDIA),
            "IFF_DYNAMIC" => Ok(NetDeviceFlags::IFF_DYNAMIC),
            "IFF_LOWER_UP" => Ok(NetDeviceFlags::IFF_LOWER_UP),
            "IFF_DORMANT" => Ok(NetDeviceFlags::IFF_DORMANT),
            "IFF_ECHO" => Ok(NetDeviceFlags::IFF_ECHO),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NetDeviceFlags {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NetDeviceFlags::IFF_UP => write!(f, "IFF_UP"),
            NetDeviceFlags::IFF_BROADCAST => write!(f, "IFF_BROADCAST"),
            NetDeviceFlags::IFF_DEBUG => write!(f, "IFF_DEBUG"),
            NetDeviceFlags::IFF_LOOPBACK => write!(f, "IFF_LOOPBACK"),
            NetDeviceFlags::IFF_POINTOPOINT => write!(f, "IFF_POINTOPOINT"),
            NetDeviceFlags::IFF_NOTRAILERS => write!(f, "IFF_NOTRAILERS"),
            NetDeviceFlags::IFF_RUNNING => write!(f, "IFF_RUNNING"),
            NetDeviceFlags::IFF_NOARP => write!(f, "IFF_NOARP"),
            NetDeviceFlags::IFF_PROMISC => write!(f, "IFF_PROMISC"),
            NetDeviceFlags::IFF_ALLMULTI => write!(f, "IFF_ALLMULTI"),
            NetDeviceFlags::IFF_MASTER => write!(f, "IFF_MASTER"),
            NetDeviceFlags::IFF_SLAVE => write!(f, "IFF_SLAVE"),
            NetDeviceFlags::IFF_MULTICAST => write!(f, "IFF_MULTICAST"),
            NetDeviceFlags::IFF_PORTSEL => write!(f, "IFF_PORTSEL"),
            NetDeviceFlags::IFF_AUTOMEDIA => write!(f, "IFF_AUTOMEDIA"),
            NetDeviceFlags::IFF_DYNAMIC => write!(f, "IFF_DYNAMIC"),
            NetDeviceFlags::IFF_LOWER_UP => write!(f, "IFF_LOWER_UP"),
            NetDeviceFlags::IFF_DORMANT => write!(f, "IFF_DORMANT"),
            NetDeviceFlags::IFF_ECHO => write!(f, "IFF_ECHO"),
        }
    }
}
impl ::num::traits::FromPrimitive for NetDeviceFlags {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x1 => Some(NetDeviceFlags::IFF_UP),
            0x2 => Some(NetDeviceFlags::IFF_BROADCAST),
            0x4 => Some(NetDeviceFlags::IFF_DEBUG),
            0x8 => Some(NetDeviceFlags::IFF_LOOPBACK),
            0x10 => Some(NetDeviceFlags::IFF_POINTOPOINT),
            0x20 => Some(NetDeviceFlags::IFF_NOTRAILERS),
            0x40 => Some(NetDeviceFlags::IFF_RUNNING),
            0x80 => Some(NetDeviceFlags::IFF_NOARP),
            0x100 => Some(NetDeviceFlags::IFF_PROMISC),
            0x200 => Some(NetDeviceFlags::IFF_ALLMULTI),
            0x400 => Some(NetDeviceFlags::IFF_MASTER),
            0x800 => Some(NetDeviceFlags::IFF_SLAVE),
            0x1000 => Some(NetDeviceFlags::IFF_MULTICAST),
            0x2000 => Some(NetDeviceFlags::IFF_PORTSEL),
            0x4000 => Some(NetDeviceFlags::IFF_AUTOMEDIA),
            0x8000 => Some(NetDeviceFlags::IFF_DYNAMIC),
            0x10000 => Some(NetDeviceFlags::IFF_LOWER_UP),
            0x20000 => Some(NetDeviceFlags::IFF_DORMANT),
            0x40000 => Some(NetDeviceFlags::IFF_ECHO),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x1 => Some(NetDeviceFlags::IFF_UP),
            0x2 => Some(NetDeviceFlags::IFF_BROADCAST),
            0x4 => Some(NetDeviceFlags::IFF_DEBUG),
            0x8 => Some(NetDeviceFlags::IFF_LOOPBACK),
            0x10 => Some(NetDeviceFlags::IFF_POINTOPOINT),
            0x20 => Some(NetDeviceFlags::IFF_NOTRAILERS),
            0x40 => Some(NetDeviceFlags::IFF_RUNNING),
            0x80 => Some(NetDeviceFlags::IFF_NOARP),
            0x100 => Some(NetDeviceFlags::IFF_PROMISC),
            0x200 => Some(NetDeviceFlags::IFF_ALLMULTI),
            0x400 => Some(NetDeviceFlags::IFF_MASTER),
            0x800 => Some(NetDeviceFlags::IFF_SLAVE),
            0x1000 => Some(NetDeviceFlags::IFF_MULTICAST),
            0x2000 => Some(NetDeviceFlags::IFF_PORTSEL),
            0x4000 => Some(NetDeviceFlags::IFF_AUTOMEDIA),
            0x8000 => Some(NetDeviceFlags::IFF_DYNAMIC),
            0x10000 => Some(NetDeviceFlags::IFF_LOWER_UP),
            0x20000 => Some(NetDeviceFlags::IFF_DORMANT),
            0x40000 => Some(NetDeviceFlags::IFF_ECHO),
            _ => None
        }
    }
}
impl_pretty_flag_fmt!(NetDeviceFlags, NetDeviceFlags::IFF_ECHO, NetDeviceFlags::from_u32);

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum AddressFamily {
    AF_UNSPEC = 0,
    AF_LOCAL = 1, // aka AF_UNIX
    AF_INET = 2,
    AF_AX25 = 3,
    AF_IPX = 4,
    AF_APPLETALK = 5,
    AF_NETROM = 6,
    AF_BRIDGE = 7,
    AF_ATMPVC = 8,
    AF_X25 = 9,
    AF_INET6 = 10,
    AF_ROSE = 11,
    AF_DECnet = 12,
    AF_NETBEUI = 13,
    AF_SECURITY = 14,
    AF_KEY = 15,
    AF_NETLINK = 16, // aka AF_ROUTE
    AF_PACKET = 17,
    AF_ASH = 18,
    AF_ECONET = 19,
    AF_ATMSVC = 20,
    AF_RDS = 21,
    AF_SNA = 22,
    AF_IRDA = 23,
    AF_PPPOX = 24,
    AF_WANPIPE = 25,
    AF_LLC = 26,
    AF_IB = 27,
    AF_MPLS = 28,
    AF_CAN = 29,
    AF_TIPC = 30,
    AF_BLUETOOTH = 31,
    AF_IUCV = 32,
    AF_RXRPC = 33,
    AF_ISDN = 34,
    AF_PHONET = 35,
    AF_IEEE802154 = 36,
    AF_CAIF = 37,
    AF_ALG = 38,
    AF_NFC = 39,
    AF_VSOCK = 40,
    AF_MAX = 41,
}
impl ::std::str::FromStr for AddressFamily {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AF_UNSPEC" => Ok(AddressFamily::AF_UNSPEC),
            "AF_LOCAL" => Ok(AddressFamily::AF_LOCAL),
            "AF_INET" => Ok(AddressFamily::AF_INET),
            "AF_AX25" => Ok(AddressFamily::AF_AX25),
            "AF_IPX" => Ok(AddressFamily::AF_IPX),
            "AF_APPLETALK" => Ok(AddressFamily::AF_APPLETALK),
            "AF_NETROM" => Ok(AddressFamily::AF_NETROM),
            "AF_BRIDGE" => Ok(AddressFamily::AF_BRIDGE),
            "AF_ATMPVC" => Ok(AddressFamily::AF_ATMPVC),
            "AF_X25" => Ok(AddressFamily::AF_X25),
            "AF_INET6" => Ok(AddressFamily::AF_INET6),
            "AF_ROSE" => Ok(AddressFamily::AF_ROSE),
            "AF_DECnet" => Ok(AddressFamily::AF_DECnet),
            "AF_NETBEUI" => Ok(AddressFamily::AF_NETBEUI),
            "AF_SECURITY" => Ok(AddressFamily::AF_SECURITY),
            "AF_KEY" => Ok(AddressFamily::AF_KEY),
            "AF_NETLINK" => Ok(AddressFamily::AF_NETLINK),
            "AF_PACKET" => Ok(AddressFamily::AF_PACKET),
            "AF_ASH" => Ok(AddressFamily::AF_ASH),
            "AF_ECONET" => Ok(AddressFamily::AF_ECONET),
            "AF_ATMSVC" => Ok(AddressFamily::AF_ATMSVC),
            "AF_RDS" => Ok(AddressFamily::AF_RDS),
            "AF_SNA" => Ok(AddressFamily::AF_SNA),
            "AF_IRDA" => Ok(AddressFamily::AF_IRDA),
            "AF_PPPOX" => Ok(AddressFamily::AF_PPPOX),
            "AF_WANPIPE" => Ok(AddressFamily::AF_WANPIPE),
            "AF_LLC" => Ok(AddressFamily::AF_LLC),
            "AF_IB" => Ok(AddressFamily::AF_IB),
            "AF_MPLS" => Ok(AddressFamily::AF_MPLS),
            "AF_CAN" => Ok(AddressFamily::AF_CAN),
            "AF_TIPC" => Ok(AddressFamily::AF_TIPC),
            "AF_BLUETOOTH" => Ok(AddressFamily::AF_BLUETOOTH),
            "AF_IUCV" => Ok(AddressFamily::AF_IUCV),
            "AF_RXRPC" => Ok(AddressFamily::AF_RXRPC),
            "AF_ISDN" => Ok(AddressFamily::AF_ISDN),
            "AF_PHONET" => Ok(AddressFamily::AF_PHONET),
            "AF_IEEE802154" => Ok(AddressFamily::AF_IEEE802154),
            "AF_CAIF" => Ok(AddressFamily::AF_CAIF),
            "AF_ALG" => Ok(AddressFamily::AF_ALG),
            "AF_NFC" => Ok(AddressFamily::AF_NFC),
            "AF_VSOCK" => Ok(AddressFamily::AF_VSOCK),
            "AF_MAX" => Ok(AddressFamily::AF_MAX),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for AddressFamily {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AddressFamily::AF_UNSPEC => write!(f, "AF_UNSPEC"),
            AddressFamily::AF_LOCAL => write!(f, "AF_LOCAL"),
            AddressFamily::AF_INET => write!(f, "AF_INET"),
            AddressFamily::AF_AX25 => write!(f, "AF_AX25"),
            AddressFamily::AF_IPX => write!(f, "AF_IPX"),
            AddressFamily::AF_APPLETALK => write!(f, "AF_APPLETALK"),
            AddressFamily::AF_NETROM => write!(f, "AF_NETROM"),
            AddressFamily::AF_BRIDGE => write!(f, "AF_BRIDGE"),
            AddressFamily::AF_ATMPVC => write!(f, "AF_ATMPVC"),
            AddressFamily::AF_X25 => write!(f, "AF_X25"),
            AddressFamily::AF_INET6 => write!(f, "AF_INET6"),
            AddressFamily::AF_ROSE => write!(f, "AF_ROSE"),
            AddressFamily::AF_DECnet => write!(f, "AF_DECnet"),
            AddressFamily::AF_NETBEUI => write!(f, "AF_NETBEUI"),
            AddressFamily::AF_SECURITY => write!(f, "AF_SECURITY"),
            AddressFamily::AF_KEY => write!(f, "AF_KEY"),
            AddressFamily::AF_NETLINK => write!(f, "AF_NETLINK"),
            AddressFamily::AF_PACKET => write!(f, "AF_PACKET"),
            AddressFamily::AF_ASH => write!(f, "AF_ASH"),
            AddressFamily::AF_ECONET => write!(f, "AF_ECONET"),
            AddressFamily::AF_ATMSVC => write!(f, "AF_ATMSVC"),
            AddressFamily::AF_RDS => write!(f, "AF_RDS"),
            AddressFamily::AF_SNA => write!(f, "AF_SNA"),
            AddressFamily::AF_IRDA => write!(f, "AF_IRDA"),
            AddressFamily::AF_PPPOX => write!(f, "AF_PPPOX"),
            AddressFamily::AF_WANPIPE => write!(f, "AF_WANPIPE"),
            AddressFamily::AF_LLC => write!(f, "AF_LLC"),
            AddressFamily::AF_IB => write!(f, "AF_IB"),
            AddressFamily::AF_MPLS => write!(f, "AF_MPLS"),
            AddressFamily::AF_CAN => write!(f, "AF_CAN"),
            AddressFamily::AF_TIPC => write!(f, "AF_TIPC"),
            AddressFamily::AF_BLUETOOTH => write!(f, "AF_BLUETOOTH"),
            AddressFamily::AF_IUCV => write!(f, "AF_IUCV"),
            AddressFamily::AF_RXRPC => write!(f, "AF_RXRPC"),
            AddressFamily::AF_ISDN => write!(f, "AF_ISDN"),
            AddressFamily::AF_PHONET => write!(f, "AF_PHONET"),
            AddressFamily::AF_IEEE802154 => write!(f, "AF_IEEE802154"),
            AddressFamily::AF_CAIF => write!(f, "AF_CAIF"),
            AddressFamily::AF_ALG => write!(f, "AF_ALG"),
            AddressFamily::AF_NFC => write!(f, "AF_NFC"),
            AddressFamily::AF_VSOCK => write!(f, "AF_VSOCK"),
            AddressFamily::AF_MAX => write!(f, "AF_MAX"),
        }
    }
}
impl ::num::traits::FromPrimitive for AddressFamily {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(AddressFamily::AF_UNSPEC),
            1 => Some(AddressFamily::AF_LOCAL),
            2 => Some(AddressFamily::AF_INET),
            3 => Some(AddressFamily::AF_AX25),
            4 => Some(AddressFamily::AF_IPX),
            5 => Some(AddressFamily::AF_APPLETALK),
            6 => Some(AddressFamily::AF_NETROM),
            7 => Some(AddressFamily::AF_BRIDGE),
            8 => Some(AddressFamily::AF_ATMPVC),
            9 => Some(AddressFamily::AF_X25),
            10 => Some(AddressFamily::AF_INET6),
            11 => Some(AddressFamily::AF_ROSE),
            12 => Some(AddressFamily::AF_DECnet),
            13 => Some(AddressFamily::AF_NETBEUI),
            14 => Some(AddressFamily::AF_SECURITY),
            15 => Some(AddressFamily::AF_KEY),
            16 => Some(AddressFamily::AF_NETLINK),
            17 => Some(AddressFamily::AF_PACKET),
            18 => Some(AddressFamily::AF_ASH),
            19 => Some(AddressFamily::AF_ECONET),
            20 => Some(AddressFamily::AF_ATMSVC),
            21 => Some(AddressFamily::AF_RDS),
            22 => Some(AddressFamily::AF_SNA),
            23 => Some(AddressFamily::AF_IRDA),
            24 => Some(AddressFamily::AF_PPPOX),
            25 => Some(AddressFamily::AF_WANPIPE),
            26 => Some(AddressFamily::AF_LLC),
            27 => Some(AddressFamily::AF_IB),
            28 => Some(AddressFamily::AF_MPLS),
            29 => Some(AddressFamily::AF_CAN),
            30 => Some(AddressFamily::AF_TIPC),
            31 => Some(AddressFamily::AF_BLUETOOTH),
            32 => Some(AddressFamily::AF_IUCV),
            33 => Some(AddressFamily::AF_RXRPC),
            34 => Some(AddressFamily::AF_ISDN),
            35 => Some(AddressFamily::AF_PHONET),
            36 => Some(AddressFamily::AF_IEEE802154),
            37 => Some(AddressFamily::AF_CAIF),
            38 => Some(AddressFamily::AF_ALG),
            39 => Some(AddressFamily::AF_NFC),
            40 => Some(AddressFamily::AF_VSOCK),
            41 => Some(AddressFamily::AF_MAX),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(AddressFamily::AF_UNSPEC),
            1 => Some(AddressFamily::AF_LOCAL),
            2 => Some(AddressFamily::AF_INET),
            3 => Some(AddressFamily::AF_AX25),
            4 => Some(AddressFamily::AF_IPX),
            5 => Some(AddressFamily::AF_APPLETALK),
            6 => Some(AddressFamily::AF_NETROM),
            7 => Some(AddressFamily::AF_BRIDGE),
            8 => Some(AddressFamily::AF_ATMPVC),
            9 => Some(AddressFamily::AF_X25),
            10 => Some(AddressFamily::AF_INET6),
            11 => Some(AddressFamily::AF_ROSE),
            12 => Some(AddressFamily::AF_DECnet),
            13 => Some(AddressFamily::AF_NETBEUI),
            14 => Some(AddressFamily::AF_SECURITY),
            15 => Some(AddressFamily::AF_KEY),
            16 => Some(AddressFamily::AF_NETLINK),
            17 => Some(AddressFamily::AF_PACKET),
            18 => Some(AddressFamily::AF_ASH),
            19 => Some(AddressFamily::AF_ECONET),
            20 => Some(AddressFamily::AF_ATMSVC),
            21 => Some(AddressFamily::AF_RDS),
            22 => Some(AddressFamily::AF_SNA),
            23 => Some(AddressFamily::AF_IRDA),
            24 => Some(AddressFamily::AF_PPPOX),
            25 => Some(AddressFamily::AF_WANPIPE),
            26 => Some(AddressFamily::AF_LLC),
            27 => Some(AddressFamily::AF_IB),
            28 => Some(AddressFamily::AF_MPLS),
            29 => Some(AddressFamily::AF_CAN),
            30 => Some(AddressFamily::AF_TIPC),
            31 => Some(AddressFamily::AF_BLUETOOTH),
            32 => Some(AddressFamily::AF_IUCV),
            33 => Some(AddressFamily::AF_RXRPC),
            34 => Some(AddressFamily::AF_ISDN),
            35 => Some(AddressFamily::AF_PHONET),
            36 => Some(AddressFamily::AF_IEEE802154),
            37 => Some(AddressFamily::AF_CAIF),
            38 => Some(AddressFamily::AF_ALG),
            39 => Some(AddressFamily::AF_NFC),
            40 => Some(AddressFamily::AF_VSOCK),
            41 => Some(AddressFamily::AF_MAX),
            _ => None
        }
    }
}
impl Default for AddressFamily {
    fn default() -> AddressFamily {
        AddressFamily::AF_UNSPEC
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Ifla {
    IFLA_UNSPEC = 0,
    IFLA_ADDRESS = 1,
    IFLA_BROADCAST = 2,
    IFLA_IFNAME = 3,
    IFLA_MTU = 4,
    IFLA_LINK = 5,
    IFLA_QDISC = 6,
    IFLA_STATS = 7,
    IFLA_COST = 8,
    IFLA_PRIORITY = 9,
    IFLA_MASTER = 10,
    IFLA_WIRELESS = 11,
    IFLA_PROTINFO = 12,
    IFLA_TXQLEN = 13,
    IFLA_MAP = 14,
    IFLA_WEIGHT = 15,
    IFLA_OPERSTATE = 16,
    IFLA_LINKMODE = 17,
    IFLA_LINKINFO = 18,
    IFLA_NET_NS_PID = 19,
    IFLA_IFALIAS = 20,
    IFLA_NUM_VF = 21,
    IFLA_VFINFO_LIST = 22,
    IFLA_STATS64 = 23,
    IFLA_VF_PORTS = 24,
    IFLA_PORT_SELF = 25,
    IFLA_AF_SPEC = 26,
    IFLA_GROUP = 27,
    IFLA_NET_NS_FD = 28,
    IFLA_EXT_MASK = 29,
    IFLA_PROMISCUITY = 30,
    IFLA_NUM_TX_QUEUES = 31,
    IFLA_NUM_RX_QUEUES = 32,
    IFLA_CARRIER = 33,
    IFLA_PHYS_PORT_ID = 34,
    IFLA_CARRIER_CHANGES = 35,
    IFLA_PHYS_SWITCH_ID = 36,
}
impl ::std::str::FromStr for Ifla {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IFLA_UNSPEC" => Ok(Ifla::IFLA_UNSPEC),
            "IFLA_ADDRESS" => Ok(Ifla::IFLA_ADDRESS),
            "IFLA_BROADCAST" => Ok(Ifla::IFLA_BROADCAST),
            "IFLA_IFNAME" => Ok(Ifla::IFLA_IFNAME),
            "IFLA_MTU" => Ok(Ifla::IFLA_MTU),
            "IFLA_LINK" => Ok(Ifla::IFLA_LINK),
            "IFLA_QDISC" => Ok(Ifla::IFLA_QDISC),
            "IFLA_STATS" => Ok(Ifla::IFLA_STATS),
            "IFLA_COST" => Ok(Ifla::IFLA_COST),
            "IFLA_PRIORITY" => Ok(Ifla::IFLA_PRIORITY),
            "IFLA_MASTER" => Ok(Ifla::IFLA_MASTER),
            "IFLA_WIRELESS" => Ok(Ifla::IFLA_WIRELESS),
            "IFLA_PROTINFO" => Ok(Ifla::IFLA_PROTINFO),
            "IFLA_TXQLEN" => Ok(Ifla::IFLA_TXQLEN),
            "IFLA_MAP" => Ok(Ifla::IFLA_MAP),
            "IFLA_WEIGHT" => Ok(Ifla::IFLA_WEIGHT),
            "IFLA_OPERSTATE" => Ok(Ifla::IFLA_OPERSTATE),
            "IFLA_LINKMODE" => Ok(Ifla::IFLA_LINKMODE),
            "IFLA_LINKINFO" => Ok(Ifla::IFLA_LINKINFO),
            "IFLA_NET_NS_PID" => Ok(Ifla::IFLA_NET_NS_PID),
            "IFLA_IFALIAS" => Ok(Ifla::IFLA_IFALIAS),
            "IFLA_NUM_VF" => Ok(Ifla::IFLA_NUM_VF),
            "IFLA_VFINFO_LIST" => Ok(Ifla::IFLA_VFINFO_LIST),
            "IFLA_STATS64" => Ok(Ifla::IFLA_STATS64),
            "IFLA_VF_PORTS" => Ok(Ifla::IFLA_VF_PORTS),
            "IFLA_PORT_SELF" => Ok(Ifla::IFLA_PORT_SELF),
            "IFLA_AF_SPEC" => Ok(Ifla::IFLA_AF_SPEC),
            "IFLA_GROUP" => Ok(Ifla::IFLA_GROUP),
            "IFLA_NET_NS_FD" => Ok(Ifla::IFLA_NET_NS_FD),
            "IFLA_EXT_MASK" => Ok(Ifla::IFLA_EXT_MASK),
            "IFLA_PROMISCUITY" => Ok(Ifla::IFLA_PROMISCUITY),
            "IFLA_NUM_TX_QUEUES" => Ok(Ifla::IFLA_NUM_TX_QUEUES),
            "IFLA_NUM_RX_QUEUES" => Ok(Ifla::IFLA_NUM_RX_QUEUES),
            "IFLA_CARRIER" => Ok(Ifla::IFLA_CARRIER),
            "IFLA_PHYS_PORT_ID" => Ok(Ifla::IFLA_PHYS_PORT_ID),
            "IFLA_CARRIER_CHANGES" => Ok(Ifla::IFLA_CARRIER_CHANGES),
            "IFLA_PHYS_SWITCH_ID" => Ok(Ifla::IFLA_PHYS_SWITCH_ID),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for Ifla {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Ifla::IFLA_UNSPEC => write!(f, "IFLA_UNSPEC"),
            Ifla::IFLA_ADDRESS => write!(f, "IFLA_ADDRESS"),
            Ifla::IFLA_BROADCAST => write!(f, "IFLA_BROADCAST"),
            Ifla::IFLA_IFNAME => write!(f, "IFLA_IFNAME"),
            Ifla::IFLA_MTU => write!(f, "IFLA_MTU"),
            Ifla::IFLA_LINK => write!(f, "IFLA_LINK"),
            Ifla::IFLA_QDISC => write!(f, "IFLA_QDISC"),
            Ifla::IFLA_STATS => write!(f, "IFLA_STATS"),
            Ifla::IFLA_COST => write!(f, "IFLA_COST"),
            Ifla::IFLA_PRIORITY => write!(f, "IFLA_PRIORITY"),
            Ifla::IFLA_MASTER => write!(f, "IFLA_MASTER"),
            Ifla::IFLA_WIRELESS => write!(f, "IFLA_WIRELESS"),
            Ifla::IFLA_PROTINFO => write!(f, "IFLA_PROTINFO"),
            Ifla::IFLA_TXQLEN => write!(f, "IFLA_TXQLEN"),
            Ifla::IFLA_MAP => write!(f, "IFLA_MAP"),
            Ifla::IFLA_WEIGHT => write!(f, "IFLA_WEIGHT"),
            Ifla::IFLA_OPERSTATE => write!(f, "IFLA_OPERSTATE"),
            Ifla::IFLA_LINKMODE => write!(f, "IFLA_LINKMODE"),
            Ifla::IFLA_LINKINFO => write!(f, "IFLA_LINKINFO"),
            Ifla::IFLA_NET_NS_PID => write!(f, "IFLA_NET_NS_PID"),
            Ifla::IFLA_IFALIAS => write!(f, "IFLA_IFALIAS"),
            Ifla::IFLA_NUM_VF => write!(f, "IFLA_NUM_VF"),
            Ifla::IFLA_VFINFO_LIST => write!(f, "IFLA_VFINFO_LIST"),
            Ifla::IFLA_STATS64 => write!(f, "IFLA_STATS64"),
            Ifla::IFLA_VF_PORTS => write!(f, "IFLA_VF_PORTS"),
            Ifla::IFLA_PORT_SELF => write!(f, "IFLA_PORT_SELF"),
            Ifla::IFLA_AF_SPEC => write!(f, "IFLA_AF_SPEC"),
            Ifla::IFLA_GROUP => write!(f, "IFLA_GROUP"),
            Ifla::IFLA_NET_NS_FD => write!(f, "IFLA_NET_NS_FD"),
            Ifla::IFLA_EXT_MASK => write!(f, "IFLA_EXT_MASK"),
            Ifla::IFLA_PROMISCUITY => write!(f, "IFLA_PROMISCUITY"),
            Ifla::IFLA_NUM_TX_QUEUES => write!(f, "IFLA_NUM_TX_QUEUES"),
            Ifla::IFLA_NUM_RX_QUEUES => write!(f, "IFLA_NUM_RX_QUEUES"),
            Ifla::IFLA_CARRIER => write!(f, "IFLA_CARRIER"),
            Ifla::IFLA_PHYS_PORT_ID => write!(f, "IFLA_PHYS_PORT_ID"),
            Ifla::IFLA_CARRIER_CHANGES => write!(f, "IFLA_CARRIER_CHANGES"),
            Ifla::IFLA_PHYS_SWITCH_ID => write!(f, "IFLA_PHYS_SWITCH_ID"),
        }
    }
}
impl ::num::traits::FromPrimitive for Ifla {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Ifla::IFLA_UNSPEC),
            1 => Some(Ifla::IFLA_ADDRESS),
            2 => Some(Ifla::IFLA_BROADCAST),
            3 => Some(Ifla::IFLA_IFNAME),
            4 => Some(Ifla::IFLA_MTU),
            5 => Some(Ifla::IFLA_LINK),
            6 => Some(Ifla::IFLA_QDISC),
            7 => Some(Ifla::IFLA_STATS),
            8 => Some(Ifla::IFLA_COST),
            9 => Some(Ifla::IFLA_PRIORITY),
            10 => Some(Ifla::IFLA_MASTER),
            11 => Some(Ifla::IFLA_WIRELESS),
            12 => Some(Ifla::IFLA_PROTINFO),
            13 => Some(Ifla::IFLA_TXQLEN),
            14 => Some(Ifla::IFLA_MAP),
            15 => Some(Ifla::IFLA_WEIGHT),
            16 => Some(Ifla::IFLA_OPERSTATE),
            17 => Some(Ifla::IFLA_LINKMODE),
            18 => Some(Ifla::IFLA_LINKINFO),
            19 => Some(Ifla::IFLA_NET_NS_PID),
            20 => Some(Ifla::IFLA_IFALIAS),
            21 => Some(Ifla::IFLA_NUM_VF),
            22 => Some(Ifla::IFLA_VFINFO_LIST),
            23 => Some(Ifla::IFLA_STATS64),
            24 => Some(Ifla::IFLA_VF_PORTS),
            25 => Some(Ifla::IFLA_PORT_SELF),
            26 => Some(Ifla::IFLA_AF_SPEC),
            27 => Some(Ifla::IFLA_GROUP),
            28 => Some(Ifla::IFLA_NET_NS_FD),
            29 => Some(Ifla::IFLA_EXT_MASK),
            30 => Some(Ifla::IFLA_PROMISCUITY),
            31 => Some(Ifla::IFLA_NUM_TX_QUEUES),
            32 => Some(Ifla::IFLA_NUM_RX_QUEUES),
            33 => Some(Ifla::IFLA_CARRIER),
            34 => Some(Ifla::IFLA_PHYS_PORT_ID),
            35 => Some(Ifla::IFLA_CARRIER_CHANGES),
            36 => Some(Ifla::IFLA_PHYS_SWITCH_ID),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Ifla::IFLA_UNSPEC),
            1 => Some(Ifla::IFLA_ADDRESS),
            2 => Some(Ifla::IFLA_BROADCAST),
            3 => Some(Ifla::IFLA_IFNAME),
            4 => Some(Ifla::IFLA_MTU),
            5 => Some(Ifla::IFLA_LINK),
            6 => Some(Ifla::IFLA_QDISC),
            7 => Some(Ifla::IFLA_STATS),
            8 => Some(Ifla::IFLA_COST),
            9 => Some(Ifla::IFLA_PRIORITY),
            10 => Some(Ifla::IFLA_MASTER),
            11 => Some(Ifla::IFLA_WIRELESS),
            12 => Some(Ifla::IFLA_PROTINFO),
            13 => Some(Ifla::IFLA_TXQLEN),
            14 => Some(Ifla::IFLA_MAP),
            15 => Some(Ifla::IFLA_WEIGHT),
            16 => Some(Ifla::IFLA_OPERSTATE),
            17 => Some(Ifla::IFLA_LINKMODE),
            18 => Some(Ifla::IFLA_LINKINFO),
            19 => Some(Ifla::IFLA_NET_NS_PID),
            20 => Some(Ifla::IFLA_IFALIAS),
            21 => Some(Ifla::IFLA_NUM_VF),
            22 => Some(Ifla::IFLA_VFINFO_LIST),
            23 => Some(Ifla::IFLA_STATS64),
            24 => Some(Ifla::IFLA_VF_PORTS),
            25 => Some(Ifla::IFLA_PORT_SELF),
            26 => Some(Ifla::IFLA_AF_SPEC),
            27 => Some(Ifla::IFLA_GROUP),
            28 => Some(Ifla::IFLA_NET_NS_FD),
            29 => Some(Ifla::IFLA_EXT_MASK),
            30 => Some(Ifla::IFLA_PROMISCUITY),
            31 => Some(Ifla::IFLA_NUM_TX_QUEUES),
            32 => Some(Ifla::IFLA_NUM_RX_QUEUES),
            33 => Some(Ifla::IFLA_CARRIER),
            34 => Some(Ifla::IFLA_PHYS_PORT_ID),
            35 => Some(Ifla::IFLA_CARRIER_CHANGES),
            36 => Some(Ifla::IFLA_PHYS_SWITCH_ID),
            _ => None
        }
    }
}
impl Default for Ifla {
    fn default() -> Ifla {
        Ifla::IFLA_UNSPEC
    }
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

#[allow(dead_code, non_camel_case_types)]
pub enum IfaFlags {
    IFA_F_SECONDARY = 0x1,
    IFA_F_NODAD = 0x2,
    IFA_F_OPTIMISTIC = 0x4,
    IFA_F_DADFAILED = 0x8,
    IFA_F_HOMEADDRESS = 0x10,
    IFA_F_DEPRECATED = 0x20,
    IFA_F_TENTATIVE = 0x40,
    IFA_F_PERMANENT = 0x80,
    IFA_F_MANAGETEMPADDR = 0x100,
    IFA_F_NOPREFIXROUTE = 0x200,
}
impl ::std::str::FromStr for IfaFlags {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IFA_F_SECONDARY" => Ok(IfaFlags::IFA_F_SECONDARY),
            "IFA_F_NODAD" => Ok(IfaFlags::IFA_F_NODAD),
            "IFA_F_OPTIMISTIC" => Ok(IfaFlags::IFA_F_OPTIMISTIC),
            "IFA_F_DADFAILED" => Ok(IfaFlags::IFA_F_DADFAILED),
            "IFA_F_HOMEADDRESS" => Ok(IfaFlags::IFA_F_HOMEADDRESS),
            "IFA_F_DEPRECATED" => Ok(IfaFlags::IFA_F_DEPRECATED),
            "IFA_F_TENTATIVE" => Ok(IfaFlags::IFA_F_TENTATIVE),
            "IFA_F_PERMANENT" => Ok(IfaFlags::IFA_F_PERMANENT),
            "IFA_F_MANAGETEMPADDR" => Ok(IfaFlags::IFA_F_MANAGETEMPADDR),
            "IFA_F_NOPREFIXROUTE" => Ok(IfaFlags::IFA_F_NOPREFIXROUTE),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for IfaFlags {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            IfaFlags::IFA_F_SECONDARY => write!(f, "IFA_F_SECONDARY"),
            IfaFlags::IFA_F_NODAD => write!(f, "IFA_F_NODAD"),
            IfaFlags::IFA_F_OPTIMISTIC => write!(f, "IFA_F_OPTIMISTIC"),
            IfaFlags::IFA_F_DADFAILED => write!(f, "IFA_F_DADFAILED"),
            IfaFlags::IFA_F_HOMEADDRESS => write!(f, "IFA_F_HOMEADDRESS"),
            IfaFlags::IFA_F_DEPRECATED => write!(f, "IFA_F_DEPRECATED"),
            IfaFlags::IFA_F_TENTATIVE => write!(f, "IFA_F_TENTATIVE"),
            IfaFlags::IFA_F_PERMANENT => write!(f, "IFA_F_PERMANENT"),
            IfaFlags::IFA_F_MANAGETEMPADDR => write!(f, "IFA_F_MANAGETEMPADDR"),
            IfaFlags::IFA_F_NOPREFIXROUTE => write!(f, "IFA_F_NOPREFIXROUTE"),
        }
    }
}
impl ::num::traits::FromPrimitive for IfaFlags {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x1 => Some(IfaFlags::IFA_F_SECONDARY),
            0x2 => Some(IfaFlags::IFA_F_NODAD),
            0x4 => Some(IfaFlags::IFA_F_OPTIMISTIC),
            0x8 => Some(IfaFlags::IFA_F_DADFAILED),
            0x10 => Some(IfaFlags::IFA_F_HOMEADDRESS),
            0x20 => Some(IfaFlags::IFA_F_DEPRECATED),
            0x40 => Some(IfaFlags::IFA_F_TENTATIVE),
            0x80 => Some(IfaFlags::IFA_F_PERMANENT),
            0x100 => Some(IfaFlags::IFA_F_MANAGETEMPADDR),
            0x200 => Some(IfaFlags::IFA_F_NOPREFIXROUTE),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x1 => Some(IfaFlags::IFA_F_SECONDARY),
            0x2 => Some(IfaFlags::IFA_F_NODAD),
            0x4 => Some(IfaFlags::IFA_F_OPTIMISTIC),
            0x8 => Some(IfaFlags::IFA_F_DADFAILED),
            0x10 => Some(IfaFlags::IFA_F_HOMEADDRESS),
            0x20 => Some(IfaFlags::IFA_F_DEPRECATED),
            0x40 => Some(IfaFlags::IFA_F_TENTATIVE),
            0x80 => Some(IfaFlags::IFA_F_PERMANENT),
            0x100 => Some(IfaFlags::IFA_F_MANAGETEMPADDR),
            0x200 => Some(IfaFlags::IFA_F_NOPREFIXROUTE),
            _ => None
        }
    }
}
impl_pretty_flag_fmt!(IfaFlags, IfaFlags::IFA_F_NOPREFIXROUTE, IfaFlags::from_u32);

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Ifa {
    IFA_UNSPEC = 0,
    IFA_ADDRESS = 1,
    IFA_LOCAL = 2,
    IFA_LABEL = 3,
    IFA_BROADCAST = 4,
    IFA_ANYCAST = 5,
    IFA_CACHEINFO = 6,
    IFA_MULTICAST = 7,
    IFA_FLAGS = 8,
}
impl ::std::str::FromStr for Ifa {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IFA_UNSPEC" => Ok(Ifa::IFA_UNSPEC),
            "IFA_ADDRESS" => Ok(Ifa::IFA_ADDRESS),
            "IFA_LOCAL" => Ok(Ifa::IFA_LOCAL),
            "IFA_LABEL" => Ok(Ifa::IFA_LABEL),
            "IFA_BROADCAST" => Ok(Ifa::IFA_BROADCAST),
            "IFA_ANYCAST" => Ok(Ifa::IFA_ANYCAST),
            "IFA_CACHEINFO" => Ok(Ifa::IFA_CACHEINFO),
            "IFA_MULTICAST" => Ok(Ifa::IFA_MULTICAST),
            "IFA_FLAGS" => Ok(Ifa::IFA_FLAGS),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for Ifa {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Ifa::IFA_UNSPEC => write!(f, "IFA_UNSPEC"),
            Ifa::IFA_ADDRESS => write!(f, "IFA_ADDRESS"),
            Ifa::IFA_LOCAL => write!(f, "IFA_LOCAL"),
            Ifa::IFA_LABEL => write!(f, "IFA_LABEL"),
            Ifa::IFA_BROADCAST => write!(f, "IFA_BROADCAST"),
            Ifa::IFA_ANYCAST => write!(f, "IFA_ANYCAST"),
            Ifa::IFA_CACHEINFO => write!(f, "IFA_CACHEINFO"),
            Ifa::IFA_MULTICAST => write!(f, "IFA_MULTICAST"),
            Ifa::IFA_FLAGS => write!(f, "IFA_FLAGS"),
        }
    }
}
impl ::num::traits::FromPrimitive for Ifa {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Ifa::IFA_UNSPEC),
            1 => Some(Ifa::IFA_ADDRESS),
            2 => Some(Ifa::IFA_LOCAL),
            3 => Some(Ifa::IFA_LABEL),
            4 => Some(Ifa::IFA_BROADCAST),
            5 => Some(Ifa::IFA_ANYCAST),
            6 => Some(Ifa::IFA_CACHEINFO),
            7 => Some(Ifa::IFA_MULTICAST),
            8 => Some(Ifa::IFA_FLAGS),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Ifa::IFA_UNSPEC),
            1 => Some(Ifa::IFA_ADDRESS),
            2 => Some(Ifa::IFA_LOCAL),
            3 => Some(Ifa::IFA_LABEL),
            4 => Some(Ifa::IFA_BROADCAST),
            5 => Some(Ifa::IFA_ANYCAST),
            6 => Some(Ifa::IFA_CACHEINFO),
            7 => Some(Ifa::IFA_MULTICAST),
            8 => Some(Ifa::IFA_FLAGS),
            _ => None
        }
    }
}
impl Default for Ifa {
    fn default() -> Ifa {
        Ifa::IFA_UNSPEC
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
        try!(write!(f, ")\n{}    ifa_scope: {},\n", i_s, self.ifa_scope));
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

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Rtn {
    RTN_UNSPEC = 0,
    RTN_UNICAST = 1,
    RTN_LOCAL = 2,
    RTN_BROADCAST = 3,
    RTN_ANYCAST = 4,
    RTN_MULTICAST = 5,
    RTN_BLACKHOLE = 6,
    RTN_UNREACHABLE = 7,
    RTN_PROHIBIT = 8,
    RTN_THROW = 9,
    RTN_NAT = 10,
    RTN_XRESOLVE = 11,
}
impl ::std::str::FromStr for Rtn {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RTN_UNSPEC" => Ok(Rtn::RTN_UNSPEC),
            "RTN_UNICAST" => Ok(Rtn::RTN_UNICAST),
            "RTN_LOCAL" => Ok(Rtn::RTN_LOCAL),
            "RTN_BROADCAST" => Ok(Rtn::RTN_BROADCAST),
            "RTN_ANYCAST" => Ok(Rtn::RTN_ANYCAST),
            "RTN_MULTICAST" => Ok(Rtn::RTN_MULTICAST),
            "RTN_BLACKHOLE" => Ok(Rtn::RTN_BLACKHOLE),
            "RTN_UNREACHABLE" => Ok(Rtn::RTN_UNREACHABLE),
            "RTN_PROHIBIT" => Ok(Rtn::RTN_PROHIBIT),
            "RTN_THROW" => Ok(Rtn::RTN_THROW),
            "RTN_NAT" => Ok(Rtn::RTN_NAT),
            "RTN_XRESOLVE" => Ok(Rtn::RTN_XRESOLVE),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for Rtn {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Rtn::RTN_UNSPEC => write!(f, "RTN_UNSPEC"),
            Rtn::RTN_UNICAST => write!(f, "RTN_UNICAST"),
            Rtn::RTN_LOCAL => write!(f, "RTN_LOCAL"),
            Rtn::RTN_BROADCAST => write!(f, "RTN_BROADCAST"),
            Rtn::RTN_ANYCAST => write!(f, "RTN_ANYCAST"),
            Rtn::RTN_MULTICAST => write!(f, "RTN_MULTICAST"),
            Rtn::RTN_BLACKHOLE => write!(f, "RTN_BLACKHOLE"),
            Rtn::RTN_UNREACHABLE => write!(f, "RTN_UNREACHABLE"),
            Rtn::RTN_PROHIBIT => write!(f, "RTN_PROHIBIT"),
            Rtn::RTN_THROW => write!(f, "RTN_THROW"),
            Rtn::RTN_NAT => write!(f, "RTN_NAT"),
            Rtn::RTN_XRESOLVE => write!(f, "RTN_XRESOLVE"),
        }
    }
}
impl ::num::traits::FromPrimitive for Rtn {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Rtn::RTN_UNSPEC),
            1 => Some(Rtn::RTN_UNICAST),
            2 => Some(Rtn::RTN_LOCAL),
            3 => Some(Rtn::RTN_BROADCAST),
            4 => Some(Rtn::RTN_ANYCAST),
            5 => Some(Rtn::RTN_MULTICAST),
            6 => Some(Rtn::RTN_BLACKHOLE),
            7 => Some(Rtn::RTN_UNREACHABLE),
            8 => Some(Rtn::RTN_PROHIBIT),
            9 => Some(Rtn::RTN_THROW),
            10 => Some(Rtn::RTN_NAT),
            11 => Some(Rtn::RTN_XRESOLVE),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Rtn::RTN_UNSPEC),
            1 => Some(Rtn::RTN_UNICAST),
            2 => Some(Rtn::RTN_LOCAL),
            3 => Some(Rtn::RTN_BROADCAST),
            4 => Some(Rtn::RTN_ANYCAST),
            5 => Some(Rtn::RTN_MULTICAST),
            6 => Some(Rtn::RTN_BLACKHOLE),
            7 => Some(Rtn::RTN_UNREACHABLE),
            8 => Some(Rtn::RTN_PROHIBIT),
            9 => Some(Rtn::RTN_THROW),
            10 => Some(Rtn::RTN_NAT),
            11 => Some(Rtn::RTN_XRESOLVE),
            _ => None
        }
    }
}
impl Default for Rtn {
    fn default() -> Rtn {
        Rtn::RTN_UNSPEC
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum Rtprot {
    RTPROT_UNSPEC = 0,
    RTPROT_REDIRECT = 1,
    RTPROT_KERNEL = 2,
    RTPROT_BOOT = 3,
    RTPROT_STATIC = 4,
    RTPROT_GATED = 8,
    RTPROT_RA = 9,
    RTPROT_MRT = 10,
    RTPROT_ZEBRA = 11,
    RTPROT_BIRD = 12,
    RTPROT_DNROUTED = 13,
    RTPROT_XORP = 14,
    RTPROT_NTK = 15,
    RTPROT_DHCP = 16,
    RTPROT_MROUTED = 17,
    RTPROT_BABEL = 42,
}
impl ::std::str::FromStr for Rtprot {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RTPROT_UNSPEC" => Ok(Rtprot::RTPROT_UNSPEC),
            "RTPROT_REDIRECT" => Ok(Rtprot::RTPROT_REDIRECT),
            "RTPROT_KERNEL" => Ok(Rtprot::RTPROT_KERNEL),
            "RTPROT_BOOT" => Ok(Rtprot::RTPROT_BOOT),
            "RTPROT_STATIC" => Ok(Rtprot::RTPROT_STATIC),
            "RTPROT_GATED" => Ok(Rtprot::RTPROT_GATED),
            "RTPROT_RA" => Ok(Rtprot::RTPROT_RA),
            "RTPROT_MRT" => Ok(Rtprot::RTPROT_MRT),
            "RTPROT_ZEBRA" => Ok(Rtprot::RTPROT_ZEBRA),
            "RTPROT_BIRD" => Ok(Rtprot::RTPROT_BIRD),
            "RTPROT_DNROUTED" => Ok(Rtprot::RTPROT_DNROUTED),
            "RTPROT_XORP" => Ok(Rtprot::RTPROT_XORP),
            "RTPROT_NTK" => Ok(Rtprot::RTPROT_NTK),
            "RTPROT_DHCP" => Ok(Rtprot::RTPROT_DHCP),
            "RTPROT_MROUTED" => Ok(Rtprot::RTPROT_MROUTED),
            "RTPROT_BABEL" => Ok(Rtprot::RTPROT_BABEL),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for Rtprot {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Rtprot::RTPROT_UNSPEC => write!(f, "RTPROT_UNSPEC"),
            Rtprot::RTPROT_REDIRECT => write!(f, "RTPROT_REDIRECT"),
            Rtprot::RTPROT_KERNEL => write!(f, "RTPROT_KERNEL"),
            Rtprot::RTPROT_BOOT => write!(f, "RTPROT_BOOT"),
            Rtprot::RTPROT_STATIC => write!(f, "RTPROT_STATIC"),
            Rtprot::RTPROT_GATED => write!(f, "RTPROT_GATED"),
            Rtprot::RTPROT_RA => write!(f, "RTPROT_RA"),
            Rtprot::RTPROT_MRT => write!(f, "RTPROT_MRT"),
            Rtprot::RTPROT_ZEBRA => write!(f, "RTPROT_ZEBRA"),
            Rtprot::RTPROT_BIRD => write!(f, "RTPROT_BIRD"),
            Rtprot::RTPROT_DNROUTED => write!(f, "RTPROT_DNROUTED"),
            Rtprot::RTPROT_XORP => write!(f, "RTPROT_XORP"),
            Rtprot::RTPROT_NTK => write!(f, "RTPROT_NTK"),
            Rtprot::RTPROT_DHCP => write!(f, "RTPROT_DHCP"),
            Rtprot::RTPROT_MROUTED => write!(f, "RTPROT_MROUTED"),
            Rtprot::RTPROT_BABEL => write!(f, "RTPROT_BABEL"),
        }
    }
}
impl ::num::traits::FromPrimitive for Rtprot {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Rtprot::RTPROT_UNSPEC),
            1 => Some(Rtprot::RTPROT_REDIRECT),
            2 => Some(Rtprot::RTPROT_KERNEL),
            3 => Some(Rtprot::RTPROT_BOOT),
            4 => Some(Rtprot::RTPROT_STATIC),
            8 => Some(Rtprot::RTPROT_GATED),
            9 => Some(Rtprot::RTPROT_RA),
            10 => Some(Rtprot::RTPROT_MRT),
            11 => Some(Rtprot::RTPROT_ZEBRA),
            12 => Some(Rtprot::RTPROT_BIRD),
            13 => Some(Rtprot::RTPROT_DNROUTED),
            14 => Some(Rtprot::RTPROT_XORP),
            15 => Some(Rtprot::RTPROT_NTK),
            16 => Some(Rtprot::RTPROT_DHCP),
            17 => Some(Rtprot::RTPROT_MROUTED),
            42 => Some(Rtprot::RTPROT_BABEL),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(Rtprot::RTPROT_UNSPEC),
            1 => Some(Rtprot::RTPROT_REDIRECT),
            2 => Some(Rtprot::RTPROT_KERNEL),
            3 => Some(Rtprot::RTPROT_BOOT),
            4 => Some(Rtprot::RTPROT_STATIC),
            8 => Some(Rtprot::RTPROT_GATED),
            9 => Some(Rtprot::RTPROT_RA),
            10 => Some(Rtprot::RTPROT_MRT),
            11 => Some(Rtprot::RTPROT_ZEBRA),
            12 => Some(Rtprot::RTPROT_BIRD),
            13 => Some(Rtprot::RTPROT_DNROUTED),
            14 => Some(Rtprot::RTPROT_XORP),
            15 => Some(Rtprot::RTPROT_NTK),
            16 => Some(Rtprot::RTPROT_DHCP),
            17 => Some(Rtprot::RTPROT_MROUTED),
            42 => Some(Rtprot::RTPROT_BABEL),
            _ => None
        }
    }
}
impl Default for Rtprot {
    fn default() -> Rtprot {
        Rtprot::RTPROT_UNSPEC
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum RtScope {
    RT_SCOPE_UNIVERSE = 0,
    RT_SCOPE_SITE = 200,
    RT_SCOPE_LINK = 253,
    RT_SCOPE_HOST = 254,
}
impl ::std::str::FromStr for RtScope {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RT_SCOPE_UNIVERSE" => Ok(RtScope::RT_SCOPE_UNIVERSE),
            "RT_SCOPE_SITE" => Ok(RtScope::RT_SCOPE_SITE),
            "RT_SCOPE_LINK" => Ok(RtScope::RT_SCOPE_LINK),
            "RT_SCOPE_HOST" => Ok(RtScope::RT_SCOPE_HOST),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for RtScope {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RtScope::RT_SCOPE_UNIVERSE => write!(f, "RT_SCOPE_UNIVERSE"),
            RtScope::RT_SCOPE_SITE => write!(f, "RT_SCOPE_SITE"),
            RtScope::RT_SCOPE_LINK => write!(f, "RT_SCOPE_LINK"),
            RtScope::RT_SCOPE_HOST => write!(f, "RT_SCOPE_HOST"),
        }
    }
}
impl ::num::traits::FromPrimitive for RtScope {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(RtScope::RT_SCOPE_UNIVERSE),
            200 => Some(RtScope::RT_SCOPE_SITE),
            253 => Some(RtScope::RT_SCOPE_LINK),
            254 => Some(RtScope::RT_SCOPE_HOST),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(RtScope::RT_SCOPE_UNIVERSE),
            200 => Some(RtScope::RT_SCOPE_SITE),
            253 => Some(RtScope::RT_SCOPE_LINK),
            254 => Some(RtScope::RT_SCOPE_HOST),
            _ => None
        }
    }
}
impl Default for RtScope {
    fn default() -> RtScope {
        RtScope::RT_SCOPE_UNIVERSE
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

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum RtTable {
    RT_TABLE_UNSPEC = 0,
    RT_TABLE_COMPAT = 252,
    RT_TABLE_DEFAULT = 253,
    RT_TABLE_MAIN = 254,
    RT_TABLE_LOCAL = 255,
}
impl ::std::str::FromStr for RtTable {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RT_TABLE_UNSPEC" => Ok(RtTable::RT_TABLE_UNSPEC),
            "RT_TABLE_COMPAT" => Ok(RtTable::RT_TABLE_COMPAT),
            "RT_TABLE_DEFAULT" => Ok(RtTable::RT_TABLE_DEFAULT),
            "RT_TABLE_MAIN" => Ok(RtTable::RT_TABLE_MAIN),
            "RT_TABLE_LOCAL" => Ok(RtTable::RT_TABLE_LOCAL),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for RtTable {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RtTable::RT_TABLE_UNSPEC => write!(f, "RT_TABLE_UNSPEC"),
            RtTable::RT_TABLE_COMPAT => write!(f, "RT_TABLE_COMPAT"),
            RtTable::RT_TABLE_DEFAULT => write!(f, "RT_TABLE_DEFAULT"),
            RtTable::RT_TABLE_MAIN => write!(f, "RT_TABLE_MAIN"),
            RtTable::RT_TABLE_LOCAL => write!(f, "RT_TABLE_LOCAL"),
        }
    }
}
impl ::num::traits::FromPrimitive for RtTable {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(RtTable::RT_TABLE_UNSPEC),
            252 => Some(RtTable::RT_TABLE_COMPAT),
            253 => Some(RtTable::RT_TABLE_DEFAULT),
            254 => Some(RtTable::RT_TABLE_MAIN),
            255 => Some(RtTable::RT_TABLE_LOCAL),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(RtTable::RT_TABLE_UNSPEC),
            252 => Some(RtTable::RT_TABLE_COMPAT),
            253 => Some(RtTable::RT_TABLE_DEFAULT),
            254 => Some(RtTable::RT_TABLE_MAIN),
            255 => Some(RtTable::RT_TABLE_LOCAL),
            _ => None
        }
    }
}
impl Default for RtTable {
    fn default() -> RtTable {
        RtTable::RT_TABLE_UNSPEC
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

#[allow(dead_code, non_camel_case_types)]
pub enum RtmFlags {
    RTM_F_NOTIFY = 0x100,
    RTM_F_CLONED = 0x200,
    RTM_F_EQUALIZE = 0x400,
    RTM_F_PREFIX = 0x800,
}
impl ::std::str::FromStr for RtmFlags {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RTM_F_NOTIFY" => Ok(RtmFlags::RTM_F_NOTIFY),
            "RTM_F_CLONED" => Ok(RtmFlags::RTM_F_CLONED),
            "RTM_F_EQUALIZE" => Ok(RtmFlags::RTM_F_EQUALIZE),
            "RTM_F_PREFIX" => Ok(RtmFlags::RTM_F_PREFIX),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for RtmFlags {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RtmFlags::RTM_F_NOTIFY => write!(f, "RTM_F_NOTIFY"),
            RtmFlags::RTM_F_CLONED => write!(f, "RTM_F_CLONED"),
            RtmFlags::RTM_F_EQUALIZE => write!(f, "RTM_F_EQUALIZE"),
            RtmFlags::RTM_F_PREFIX => write!(f, "RTM_F_PREFIX"),
        }
    }
}
impl ::num::traits::FromPrimitive for RtmFlags {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x100 => Some(RtmFlags::RTM_F_NOTIFY),
            0x200 => Some(RtmFlags::RTM_F_CLONED),
            0x400 => Some(RtmFlags::RTM_F_EQUALIZE),
            0x800 => Some(RtmFlags::RTM_F_PREFIX),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x100 => Some(RtmFlags::RTM_F_NOTIFY),
            0x200 => Some(RtmFlags::RTM_F_CLONED),
            0x400 => Some(RtmFlags::RTM_F_EQUALIZE),
            0x800 => Some(RtmFlags::RTM_F_PREFIX),
            _ => None
        }
    }
}
impl_pretty_flag_fmt!(RtmFlags, RtmFlags::RTM_F_PREFIX, RtmFlags::from_u32);

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum RtmAttr {
    RTA_UNSPEC = 0,
    RTA_DST = 1,
    RTA_SRC = 2,
    RTA_IIF = 3,
    RTA_OIF = 4,
    RTA_GATEWAY = 5,
    RTA_PRIORITY = 6,
    RTA_PREFSRC = 7,
    RTA_METRICS = 8,
    RTA_MULTIPATH = 9,
    RTA_PROTOINFO = 10,
    RTA_FLOW = 11,
    RTA_CACHEINFO = 12,
    RTA_SESSION = 13,
    RTA_MP_ALGO = 14,
    RTA_TABLE = 15,
    RTA_MARK = 16,
    RTA_MFC_STATS = 17,
}
impl ::std::str::FromStr for RtmAttr {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RTA_UNSPEC" => Ok(RtmAttr::RTA_UNSPEC),
            "RTA_DST" => Ok(RtmAttr::RTA_DST),
            "RTA_SRC" => Ok(RtmAttr::RTA_SRC),
            "RTA_IIF" => Ok(RtmAttr::RTA_IIF),
            "RTA_OIF" => Ok(RtmAttr::RTA_OIF),
            "RTA_GATEWAY" => Ok(RtmAttr::RTA_GATEWAY),
            "RTA_PRIORITY" => Ok(RtmAttr::RTA_PRIORITY),
            "RTA_PREFSRC" => Ok(RtmAttr::RTA_PREFSRC),
            "RTA_METRICS" => Ok(RtmAttr::RTA_METRICS),
            "RTA_MULTIPATH" => Ok(RtmAttr::RTA_MULTIPATH),
            "RTA_PROTOINFO" => Ok(RtmAttr::RTA_PROTOINFO),
            "RTA_FLOW" => Ok(RtmAttr::RTA_FLOW),
            "RTA_CACHEINFO" => Ok(RtmAttr::RTA_CACHEINFO),
            "RTA_SESSION" => Ok(RtmAttr::RTA_SESSION),
            "RTA_MP_ALGO" => Ok(RtmAttr::RTA_MP_ALGO),
            "RTA_TABLE" => Ok(RtmAttr::RTA_TABLE),
            "RTA_MARK" => Ok(RtmAttr::RTA_MARK),
            "RTA_MFC_STATS" => Ok(RtmAttr::RTA_MFC_STATS),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for RtmAttr {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RtmAttr::RTA_UNSPEC => write!(f, "RTA_UNSPEC"),
            RtmAttr::RTA_DST => write!(f, "RTA_DST"),
            RtmAttr::RTA_SRC => write!(f, "RTA_SRC"),
            RtmAttr::RTA_IIF => write!(f, "RTA_IIF"),
            RtmAttr::RTA_OIF => write!(f, "RTA_OIF"),
            RtmAttr::RTA_GATEWAY => write!(f, "RTA_GATEWAY"),
            RtmAttr::RTA_PRIORITY => write!(f, "RTA_PRIORITY"),
            RtmAttr::RTA_PREFSRC => write!(f, "RTA_PREFSRC"),
            RtmAttr::RTA_METRICS => write!(f, "RTA_METRICS"),
            RtmAttr::RTA_MULTIPATH => write!(f, "RTA_MULTIPATH"),
            RtmAttr::RTA_PROTOINFO => write!(f, "RTA_PROTOINFO"),
            RtmAttr::RTA_FLOW => write!(f, "RTA_FLOW"),
            RtmAttr::RTA_CACHEINFO => write!(f, "RTA_CACHEINFO"),
            RtmAttr::RTA_SESSION => write!(f, "RTA_SESSION"),
            RtmAttr::RTA_MP_ALGO => write!(f, "RTA_MP_ALGO"),
            RtmAttr::RTA_TABLE => write!(f, "RTA_TABLE"),
            RtmAttr::RTA_MARK => write!(f, "RTA_MARK"),
            RtmAttr::RTA_MFC_STATS => write!(f, "RTA_MFC_STATS"),
        }
    }
}
impl ::num::traits::FromPrimitive for RtmAttr {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(RtmAttr::RTA_UNSPEC),
            1 => Some(RtmAttr::RTA_DST),
            2 => Some(RtmAttr::RTA_SRC),
            3 => Some(RtmAttr::RTA_IIF),
            4 => Some(RtmAttr::RTA_OIF),
            5 => Some(RtmAttr::RTA_GATEWAY),
            6 => Some(RtmAttr::RTA_PRIORITY),
            7 => Some(RtmAttr::RTA_PREFSRC),
            8 => Some(RtmAttr::RTA_METRICS),
            9 => Some(RtmAttr::RTA_MULTIPATH),
            10 => Some(RtmAttr::RTA_PROTOINFO),
            11 => Some(RtmAttr::RTA_FLOW),
            12 => Some(RtmAttr::RTA_CACHEINFO),
            13 => Some(RtmAttr::RTA_SESSION),
            14 => Some(RtmAttr::RTA_MP_ALGO),
            15 => Some(RtmAttr::RTA_TABLE),
            16 => Some(RtmAttr::RTA_MARK),
            17 => Some(RtmAttr::RTA_MFC_STATS),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(RtmAttr::RTA_UNSPEC),
            1 => Some(RtmAttr::RTA_DST),
            2 => Some(RtmAttr::RTA_SRC),
            3 => Some(RtmAttr::RTA_IIF),
            4 => Some(RtmAttr::RTA_OIF),
            5 => Some(RtmAttr::RTA_GATEWAY),
            6 => Some(RtmAttr::RTA_PRIORITY),
            7 => Some(RtmAttr::RTA_PREFSRC),
            8 => Some(RtmAttr::RTA_METRICS),
            9 => Some(RtmAttr::RTA_MULTIPATH),
            10 => Some(RtmAttr::RTA_PROTOINFO),
            11 => Some(RtmAttr::RTA_FLOW),
            12 => Some(RtmAttr::RTA_CACHEINFO),
            13 => Some(RtmAttr::RTA_SESSION),
            14 => Some(RtmAttr::RTA_MP_ALGO),
            15 => Some(RtmAttr::RTA_TABLE),
            16 => Some(RtmAttr::RTA_MARK),
            17 => Some(RtmAttr::RTA_MFC_STATS),
            _ => None
        }
    }
}
impl Default for RtmAttr {
    fn default() -> RtmAttr {
        RtmAttr::RTA_UNSPEC
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
        try!(write!(f, "{}    rtm_src_len: {}\n", i_s, self.rtm_src_len));
        try!(write!(f, "{}    rtm_tos: {},\n", i_s, self.rtm_tos));
        try!(write!(f, "{}    rtm_table: {} (", i_s, self.rtm_table));
        try!(RtTable::pretty_fmt(f, self.rtm_table));
        try!(write!(f, "),\n{}    rtm_protocol: {},\n", i_s, self.rtm_protocol));
        try!(write!(f, "{}    rtm_scope: {} (", i_s, self.rtm_scope));
        try!(RtScope::pretty_fmt(f, self.rtm_scope));
        try!(write!(f, "),\n{}    rtm_type: {},\n", i_s, self.rtm_type));

        try!(write!(f, "{}    rtm_flags: {:#X} (", i_s, self.rtm_flags));
        try!(RtmFlags::pretty_fmt(f, self.rtm_flags as u32));
        try!(write!(f, ")\n{}    rtm_attr: [ ", i_s));
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

#[allow(dead_code, non_camel_case_types)]
pub enum NdState {
    NUD_NONE = 0x0,
    NUD_INCOMPLETE = 0x1,
    NUD_REACHABLE = 0x2,
    NUD_STALE = 0x4,
    NUD_DELAY = 0x8,
    NUD_PROBE = 0x10,
    NUD_FAILED = 0x20,
    NUD_NOARP = 0x40,
    NUD_PERMANENT = 0x80,
}
impl ::std::str::FromStr for NdState {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NUD_NONE" => Ok(NdState::NUD_NONE),
            "NUD_INCOMPLETE" => Ok(NdState::NUD_INCOMPLETE),
            "NUD_REACHABLE" => Ok(NdState::NUD_REACHABLE),
            "NUD_STALE" => Ok(NdState::NUD_STALE),
            "NUD_DELAY" => Ok(NdState::NUD_DELAY),
            "NUD_PROBE" => Ok(NdState::NUD_PROBE),
            "NUD_FAILED" => Ok(NdState::NUD_FAILED),
            "NUD_NOARP" => Ok(NdState::NUD_NOARP),
            "NUD_PERMANENT" => Ok(NdState::NUD_PERMANENT),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NdState {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NdState::NUD_NONE => write!(f, "NUD_NONE"),
            NdState::NUD_INCOMPLETE => write!(f, "NUD_INCOMPLETE"),
            NdState::NUD_REACHABLE => write!(f, "NUD_REACHABLE"),
            NdState::NUD_STALE => write!(f, "NUD_STALE"),
            NdState::NUD_DELAY => write!(f, "NUD_DELAY"),
            NdState::NUD_PROBE => write!(f, "NUD_PROBE"),
            NdState::NUD_FAILED => write!(f, "NUD_FAILED"),
            NdState::NUD_NOARP => write!(f, "NUD_NOARP"),
            NdState::NUD_PERMANENT => write!(f, "NUD_PERMANENT"),
        }
    }
}
impl ::num::traits::FromPrimitive for NdState {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x0 => Some(NdState::NUD_NONE),
            0x1 => Some(NdState::NUD_INCOMPLETE),
            0x2 => Some(NdState::NUD_REACHABLE),
            0x4 => Some(NdState::NUD_STALE),
            0x8 => Some(NdState::NUD_DELAY),
            0x10 => Some(NdState::NUD_PROBE),
            0x20 => Some(NdState::NUD_FAILED),
            0x40 => Some(NdState::NUD_NOARP),
            0x80 => Some(NdState::NUD_PERMANENT),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x0 => Some(NdState::NUD_NONE),
            0x1 => Some(NdState::NUD_INCOMPLETE),
            0x2 => Some(NdState::NUD_REACHABLE),
            0x4 => Some(NdState::NUD_STALE),
            0x8 => Some(NdState::NUD_DELAY),
            0x10 => Some(NdState::NUD_PROBE),
            0x20 => Some(NdState::NUD_FAILED),
            0x40 => Some(NdState::NUD_NOARP),
            0x80 => Some(NdState::NUD_PERMANENT),
            _ => None
        }
    }
}
impl Default for NdState {
    fn default() -> NdState {
        NdState::NUD_NONE
    }
}
impl_pretty_flag_fmt!(NdState, NdState::NUD_PERMANENT, NdState::from_u32);

#[allow(dead_code, non_camel_case_types)]
pub enum NdFlags {
    NTF_NONE = 0x0,
    NTF_USE = 0x1,
    NTF_SELF = 0x2,
    NTF_MASTER = 0x4,
    NTF_PROXY = 0x8,
    NTF_EXT_LEARNED = 0x10,
    NTF_ROUTER = 0x80,
}
impl ::std::str::FromStr for NdFlags {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NTF_NONE" => Ok(NdFlags::NTF_NONE),
            "NTF_USE" => Ok(NdFlags::NTF_USE),
            "NTF_SELF" => Ok(NdFlags::NTF_SELF),
            "NTF_MASTER" => Ok(NdFlags::NTF_MASTER),
            "NTF_PROXY" => Ok(NdFlags::NTF_PROXY),
            "NTF_EXT_LEARNED" => Ok(NdFlags::NTF_EXT_LEARNED),
            "NTF_ROUTER" => Ok(NdFlags::NTF_ROUTER),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NdFlags {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NdFlags::NTF_NONE => write!(f, "NTF_NONE"),
            NdFlags::NTF_USE => write!(f, "NTF_USE"),
            NdFlags::NTF_SELF => write!(f, "NTF_SELF"),
            NdFlags::NTF_MASTER => write!(f, "NTF_MASTER"),
            NdFlags::NTF_PROXY => write!(f, "NTF_PROXY"),
            NdFlags::NTF_EXT_LEARNED => write!(f, "NTF_EXT_LEARNED"),
            NdFlags::NTF_ROUTER => write!(f, "NTF_ROUTER"),
        }
    }
}
impl ::num::traits::FromPrimitive for NdFlags {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x0 => Some(NdFlags::NTF_NONE),
            0x1 => Some(NdFlags::NTF_USE),
            0x2 => Some(NdFlags::NTF_SELF),
            0x4 => Some(NdFlags::NTF_MASTER),
            0x8 => Some(NdFlags::NTF_PROXY),
            0x10 => Some(NdFlags::NTF_EXT_LEARNED),
            0x80 => Some(NdFlags::NTF_ROUTER),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0x0 => Some(NdFlags::NTF_NONE),
            0x1 => Some(NdFlags::NTF_USE),
            0x2 => Some(NdFlags::NTF_SELF),
            0x4 => Some(NdFlags::NTF_MASTER),
            0x8 => Some(NdFlags::NTF_PROXY),
            0x10 => Some(NdFlags::NTF_EXT_LEARNED),
            0x80 => Some(NdFlags::NTF_ROUTER),
            _ => None
        }
    }
}
impl Default for NdFlags {
    fn default() -> NdFlags {
        NdFlags::NTF_NONE
    }
}
impl_pretty_flag_fmt!(NdFlags, NdFlags::NTF_ROUTER, NdFlags::from_u32);

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
        try!(write!(f, "{}    ndm_updated: {:#X}\n", indent, self.ndm_updated));
        try!(write!(f, "{}    ndm_flags: {:#X}\n", indent, self.ndm_flags));
        write!(f, "{}}}", indent)
    }
}
impl ::std::fmt::Display for NdaCacheinfo {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.pretty_fmt(f, 0)
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NdAttr {
    NDA_UNSPEC = 0,
    NDA_DST = 1,
    NDA_LLADDR = 2,
    NDA_CACHEINFO = 3,
    NDA_PROBES = 4,
    NDA_VLAN = 5,
    NDA_PORT = 6,
    NDA_VNI = 7,
    NDA_IFINDEX = 8,
    NDA_MASTER = 9,
}
impl ::std::str::FromStr for NdAttr {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NDA_UNSPEC" => Ok(NdAttr::NDA_UNSPEC),
            "NDA_DST" => Ok(NdAttr::NDA_DST),
            "NDA_LLADDR" => Ok(NdAttr::NDA_LLADDR),
            "NDA_CACHEINFO" => Ok(NdAttr::NDA_CACHEINFO),
            "NDA_PROBES" => Ok(NdAttr::NDA_PROBES),
            "NDA_VLAN" => Ok(NdAttr::NDA_VLAN),
            "NDA_PORT" => Ok(NdAttr::NDA_PORT),
            "NDA_VNI" => Ok(NdAttr::NDA_VNI),
            "NDA_IFINDEX" => Ok(NdAttr::NDA_IFINDEX),
            "NDA_MASTER" => Ok(NdAttr::NDA_MASTER),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NdAttr {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NdAttr::NDA_UNSPEC => write!(f, "NDA_UNSPEC"),
            NdAttr::NDA_DST => write!(f, "NDA_DST"),
            NdAttr::NDA_LLADDR => write!(f, "NDA_LLADDR"),
            NdAttr::NDA_CACHEINFO => write!(f, "NDA_CACHEINFO"),
            NdAttr::NDA_PROBES => write!(f, "NDA_PROBES"),
            NdAttr::NDA_VLAN => write!(f, "NDA_VLAN"),
            NdAttr::NDA_PORT => write!(f, "NDA_PORT"),
            NdAttr::NDA_VNI => write!(f, "NDA_VNI"),
            NdAttr::NDA_IFINDEX => write!(f, "NDA_IFINDEX"),
            NdAttr::NDA_MASTER => write!(f, "NDA_MASTER"),
        }
    }
}
impl ::num::traits::FromPrimitive for NdAttr {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(NdAttr::NDA_UNSPEC),
            1 => Some(NdAttr::NDA_DST),
            2 => Some(NdAttr::NDA_LLADDR),
            3 => Some(NdAttr::NDA_CACHEINFO),
            4 => Some(NdAttr::NDA_PROBES),
            5 => Some(NdAttr::NDA_VLAN),
            6 => Some(NdAttr::NDA_PORT),
            7 => Some(NdAttr::NDA_VNI),
            8 => Some(NdAttr::NDA_IFINDEX),
            9 => Some(NdAttr::NDA_MASTER),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(NdAttr::NDA_UNSPEC),
            1 => Some(NdAttr::NDA_DST),
            2 => Some(NdAttr::NDA_LLADDR),
            3 => Some(NdAttr::NDA_CACHEINFO),
            4 => Some(NdAttr::NDA_PROBES),
            5 => Some(NdAttr::NDA_VLAN),
            6 => Some(NdAttr::NDA_PORT),
            7 => Some(NdAttr::NDA_VNI),
            8 => Some(NdAttr::NDA_IFINDEX),
            9 => Some(NdAttr::NDA_MASTER),
            _ => None
        }
    }
}
impl Default for NdAttr {
    fn default() -> NdAttr {
        NdAttr::NDA_UNSPEC
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
        try!(write!(f, ")\n{}    ndm_flags: {:#X} (", i_s, self.ndm_flags));
        try!(NdFlags::pretty_fmt(f, self.ndm_flags as u32));
        try!(write!(f, ")\n{}    ndm_type: {},\n", i_s, self.ndm_type));
        try!(write!(f, "{}    ndm_cacheinfo: ", i_s));
        match self.ndm_cacheinfo {
            None => try!(write!(f, "None")),
            Some(ref cacheinfo) => try!(cacheinfo.pretty_fmt(f, indent+1)),
        }

        // TODO: macro? Or move into Rtattr?
        try!(write!(f, "\n{}    ndm_attr: [ ", i_s));
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

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum TcAttr {
    TCA_UNSPEC = 0,
    TCA_KIND = 1,
    TCA_OPTIONS = 2,
    TCA_STATS = 3,
    TCA_XSTATS = 4,
    TCA_RATE = 5,
    TCA_FCNT = 6,
    TCA_STATS2 = 7,
    TCA_STAB = 8,
}
impl ::std::str::FromStr for TcAttr {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TCA_UNSPEC" => Ok(TcAttr::TCA_UNSPEC),
            "TCA_KIND" => Ok(TcAttr::TCA_KIND),
            "TCA_OPTIONS" => Ok(TcAttr::TCA_OPTIONS),
            "TCA_STATS" => Ok(TcAttr::TCA_STATS),
            "TCA_XSTATS" => Ok(TcAttr::TCA_XSTATS),
            "TCA_RATE" => Ok(TcAttr::TCA_RATE),
            "TCA_FCNT" => Ok(TcAttr::TCA_FCNT),
            "TCA_STATS2" => Ok(TcAttr::TCA_STATS2),
            "TCA_STAB" => Ok(TcAttr::TCA_STAB),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for TcAttr {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TcAttr::TCA_UNSPEC => write!(f, "TCA_UNSPEC"),
            TcAttr::TCA_KIND => write!(f, "TCA_KIND"),
            TcAttr::TCA_OPTIONS => write!(f, "TCA_OPTIONS"),
            TcAttr::TCA_STATS => write!(f, "TCA_STATS"),
            TcAttr::TCA_XSTATS => write!(f, "TCA_XSTATS"),
            TcAttr::TCA_RATE => write!(f, "TCA_RATE"),
            TcAttr::TCA_FCNT => write!(f, "TCA_FCNT"),
            TcAttr::TCA_STATS2 => write!(f, "TCA_STATS2"),
            TcAttr::TCA_STAB => write!(f, "TCA_STAB"),
        }
    }
}
impl ::num::traits::FromPrimitive for TcAttr {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(TcAttr::TCA_UNSPEC),
            1 => Some(TcAttr::TCA_KIND),
            2 => Some(TcAttr::TCA_OPTIONS),
            3 => Some(TcAttr::TCA_STATS),
            4 => Some(TcAttr::TCA_XSTATS),
            5 => Some(TcAttr::TCA_RATE),
            6 => Some(TcAttr::TCA_FCNT),
            7 => Some(TcAttr::TCA_STATS2),
            8 => Some(TcAttr::TCA_STAB),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            0 => Some(TcAttr::TCA_UNSPEC),
            1 => Some(TcAttr::TCA_KIND),
            2 => Some(TcAttr::TCA_OPTIONS),
            3 => Some(TcAttr::TCA_STATS),
            4 => Some(TcAttr::TCA_XSTATS),
            5 => Some(TcAttr::TCA_RATE),
            6 => Some(TcAttr::TCA_FCNT),
            7 => Some(TcAttr::TCA_STATS2),
            8 => Some(TcAttr::TCA_STAB),
            _ => None
        }
    }
}
impl Default for TcAttr {
    fn default() -> TcAttr {
        TcAttr::TCA_UNSPEC
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
        try!(write!(f, "{}    tcm_handle: {:#X}\n", i_s, self.tcm_handle));
        try!(write!(f, "{}    tcm_parent: {:#X}\n", i_s, self.tcm_parent));
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
