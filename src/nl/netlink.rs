// this is where the NetlinkFamily enum was generated into but build.rs
include!(concat!(env!("OUT_DIR"), "/netlink_family.rs"));

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NlMsgType {
    NLMSG_NOOP = 1,
    NLMSG_ERROR = 2,
    NLMSG_DONE = 3,
    NLMSG_OVERRUN = 4,
}
impl ::std::str::FromStr for NlMsgType {
    type Err = ();
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NLMSG_NOOP" => Ok(NlMsgType::NLMSG_NOOP),
            "NLMSG_ERROR" => Ok(NlMsgType::NLMSG_ERROR),
            "NLMSG_DONE" => Ok(NlMsgType::NLMSG_DONE),
            "NLMSG_OVERRUN" => Ok(NlMsgType::NLMSG_OVERRUN),
            _ => Err( () )
        }
    }
}
impl ::std::fmt::Display for NlMsgType {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NlMsgType::NLMSG_NOOP => write!(f, "NLMSG_NOOP"),
            NlMsgType::NLMSG_ERROR => write!(f, "NLMSG_ERROR"),
            NlMsgType::NLMSG_DONE => write!(f, "NLMSG_DONE"),
            NlMsgType::NLMSG_OVERRUN => write!(f, "NLMSG_OVERRUN"),
        }
    }
}
impl ::num::traits::FromPrimitive for NlMsgType {
    #[allow(dead_code)]
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            1 => Some(NlMsgType::NLMSG_NOOP),
            2 => Some(NlMsgType::NLMSG_ERROR),
            3 => Some(NlMsgType::NLMSG_DONE),
            4 => Some(NlMsgType::NLMSG_OVERRUN),
            _ => None
        }
    }
    #[allow(dead_code)]
    fn from_u64(n: u64) -> Option<Self> {
        match n {
            1 => Some(NlMsgType::NLMSG_NOOP),
            2 => Some(NlMsgType::NLMSG_ERROR),
            3 => Some(NlMsgType::NLMSG_DONE),
            4 => Some(NlMsgType::NLMSG_OVERRUN),
            _ => None
        }
    }
}
impl Default for NlMsgType {
    fn default() -> NlMsgType {
        NlMsgType::NLMSG_NOOP
    }
}
