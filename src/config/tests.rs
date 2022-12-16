#![allow(non_snake_case)]
#![cfg(test)]


use super::*;


#[test]
fn Config__getter() {
    let myConfig = Config::new();

    assert_eq!(myConfig.addr(), &SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564), "Wrong address got returned.");
    assert_eq!(myConfig.buffsize(), &100000000, "Wrong buffsize got returned.");
    assert_eq!(myConfig.refresh_rate(), &Duration::new(0, 100000000), "Wrong refresh_rate got returned.");
    assert_eq!(myConfig.runner_mac_addr(), &vec!["44-8A-5B-8A-02-79".to_owned()], "Wrong runner_mac_addr got returned.");
    assert_eq!(myConfig.fancy_write(), &true, "Wrong fancy_write got returned.");
    assert_eq!(myConfig.max_tries(), &3, "Wrong max_tries got returned.");
}