## nl-dump
In order to build `nl-dump` you must have `rust-enum-derive` (the excecutable) v0.3.3 or later in your path. eg:

```
$ git clone https://github.com/TaborKelly/rust-enum-derive
$ cd rust-enum-derive
$ cargo build
$ export PATH=$PATH:`pwd`/target/debug
```

Better yet, add a symlink to rust-enum-derive somewhere that is in your path.
`nl-dump` is a dissector for netlink (mostly NETLINK_ROUTE) packets. It runs from the command-line to give you mostly human readable output for NETLINK_ROUTE which have been captured to a pcap file.

```
Usage: nl-dump [options]

Options:
    -i, --input NAME    pcap input file
        --netlink_family FAMILY
                        filter for one netlink_family (NETLINK_ROUTE,
                        NETLINK_GENERIC, etc)
    -h, --help          print this help menu
```

For example:
```
$ nl-dump -i netlink.pcapng
packet[1] = [ {
        netlink_family: NETLINK_ROUTE,
        nlmsghdr: {
            nlmsg_len: 32,
            nlmsg_type: NrMsgType(RTM_NEWLINK),
            nlmsg_flags: 0x5 (NLM_F_REQUEST|NLM_F_ACK),
            nlmsg_seq: 1452822917,
            nlmsg_pid: 3128951544,
        },
        nlmsg: Ifinfomsg( {
                ifi_family: AF_UNSPEC,
                ifi_type: 0,
                ifi_index: 2,
                ifi_flags: 0x1003 (IFF_UP|IFF_BROADCAST|IFF_MULTICAST),
                ifi_change: 0,
                ifi_attr: [  ],
            } )
    }
]
packet[2] = [ {
        netlink_family: NETLINK_GENERIC,
        nlmsghdr: {
...
```
