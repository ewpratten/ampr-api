use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

/// Describes an encapsulation entry
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EncapEntry {
    /// Public IPv4 address of the gateway
    #[serde(rename = "gatewayIP")]
    pub gateway: Ipv4Addr,

    /// Type of encapsulation (currently only 'IPIP' will appear here)
    #[serde(rename = "encapType")]
    pub ty: String,

    /// Network address of the route being announced by this gateway.
    pub network: Ipv4Addr,

    /// Mask length of the network being announced
    #[serde(rename = "maskLength")]
    pub mask_len: u8,

    /// Description of the gateway as supplied by the owner.
    pub title: String,

    /// Callsign of the owner / person responsible for this gateway.
    pub owner: String,

    /// Date/time this gateway's data was last altered.
    pub updated: String,
}
