// This file is autogenerated, do not manually edit.

use crate::zbus::proxy;

/// Proxy object for `org.freedesktop.resolve1.Manager`.
#[proxy(
    interface = "org.freedesktop.resolve1.Manager",
    gen_blocking = false,
    default_service = "org.freedesktop.resolve1",
    default_path = "/org/freedesktop/resolve1"
)]
trait Manager {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResolveHostname()) Call interface method `ResolveHostname`.
    #[zbus(name = "ResolveHostname")]
    fn resolve_hostname(
        &self,
        ifindex: i32,
        name: String,
        family: i32,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(i32, i32, Vec<u8>)>, String, u64)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResolveAddress()) Call interface method `ResolveAddress`.
    #[zbus(name = "ResolveAddress")]
    fn resolve_address(
        &self,
        ifindex: i32,
        family: i32,
        address: Vec<u8>,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(i32, String)>, u64)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResolveRecord()) Call interface method `ResolveRecord`.
    #[zbus(name = "ResolveRecord")]
    fn resolve_record(
        &self,
        ifindex: i32,
        name: String,
        class: u16,
        typelabel: u16,
        flags: u64,
    ) -> crate::zbus::Result<(Vec<(i32, u16, u16, Vec<u8>)>, u64)>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResolveService()) Call interface method `ResolveService`.
    #[zbus(name = "ResolveService")]
    fn resolve_service(
        &self,
        ifindex: i32,
        name: String,
        typelabel: String,
        domain: String,
        family: i32,
        flags: u64,
    ) -> crate::zbus::Result<(
        Vec<(u16, u16, u16, String, Vec<(i32, i32, Vec<u8>)>, String)>,
        Vec<Vec<u8>>,
        String,
        String,
        String,
        u64,
    )>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#GetLink()) Call interface method `GetLink`.
    #[zbus(name = "GetLink")]
    fn get_link(&self, ifindex: i32) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDNS()) Call interface method `SetLinkDNS`.
    #[zbus(name = "SetLinkDNS")]
    fn set_link_dns(&self, ifindex: i32, addresses: Vec<(i32, Vec<u8>)>)
        -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDNSEx()) Call interface method `SetLinkDNSEx`.
    #[zbus(name = "SetLinkDNSEx")]
    fn set_link_dns_ex(
        &self,
        ifindex: i32,
        addresses: Vec<(i32, Vec<u8>, u16, String)>,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDomains()) Call interface method `SetLinkDomains`.
    #[zbus(name = "SetLinkDomains")]
    fn set_link_domains(
        &self,
        ifindex: i32,
        domains: Vec<(String, bool)>,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDefaultRoute()) Call interface method `SetLinkDefaultRoute`.
    #[zbus(name = "SetLinkDefaultRoute")]
    fn set_link_default_route(&self, ifindex: i32, enable: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkLLMNR()) Call interface method `SetLinkLLMNR`.
    #[zbus(name = "SetLinkLLMNR")]
    fn set_link_llmnr(&self, ifindex: i32, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkMulticastDNS()) Call interface method `SetLinkMulticastDNS`.
    #[zbus(name = "SetLinkMulticastDNS")]
    fn set_link_multicast_dns(&self, ifindex: i32, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDNSOverTLS()) Call interface method `SetLinkDNSOverTLS`.
    #[zbus(name = "SetLinkDNSOverTLS")]
    fn set_link_dns_over_tls(&self, ifindex: i32, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDNSSEC()) Call interface method `SetLinkDNSSEC`.
    #[zbus(name = "SetLinkDNSSEC")]
    fn set_link_dnssec(&self, ifindex: i32, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLinkDNSSECNegativeTrustAnchors()) Call interface method `SetLinkDNSSECNegativeTrustAnchors`.
    #[zbus(name = "SetLinkDNSSECNegativeTrustAnchors")]
    fn set_link_dnssec_negative_trust_anchors(
        &self,
        ifindex: i32,
        names: Vec<String>,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RevertLink()) Call interface method `RevertLink`.
    #[zbus(name = "RevertLink")]
    fn revert_link(&self, ifindex: i32) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#RegisterService()) Call interface method `RegisterService`.
    #[zbus(name = "RegisterService")]
    fn register_service(
        &self,
        id: String,
        name_template: String,
        typelabel: String,
        service_port: u16,
        service_priority: u16,
        service_weight: u16,
        txt_datas: Vec<::std::collections::HashMap<String, Vec<u8>>>,
    ) -> crate::zbus::Result<crate::zvariant::OwnedObjectPath>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#UnregisterService()) Call interface method `UnregisterService`.
    #[zbus(name = "UnregisterService")]
    fn unregister_service(
        &self,
        service_path: crate::zvariant::OwnedObjectPath,
    ) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResetStatistics()) Call interface method `ResetStatistics`.
    #[zbus(name = "ResetStatistics")]
    fn reset_statistics(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#FlushCaches()) Call interface method `FlushCaches`.
    #[zbus(name = "FlushCaches")]
    fn flush_caches(&self) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#ResetServerFeatures()) Call interface method `ResetServerFeatures`.
    #[zbus(name = "ResetServerFeatures")]
    fn reset_server_features(&self) -> crate::zbus::Result<()>;

    /// Get property `LLMNRHostname`.
    #[zbus(property(emits_changed_signal = "true"), name = "LLMNRHostname")]
    fn llmnr_hostname(&self) -> crate::zbus::Result<String>;

    /// Get property `LLMNR`.
    #[zbus(property(emits_changed_signal = "false"), name = "LLMNR")]
    fn llmnr(&self) -> crate::zbus::Result<String>;

    /// Get property `MulticastDNS`.
    #[zbus(property(emits_changed_signal = "false"), name = "MulticastDNS")]
    fn multicast_dns(&self) -> crate::zbus::Result<String>;

    /// Get property `DNSOverTLS`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSOverTLS")]
    fn dns_over_tls(&self) -> crate::zbus::Result<String>;

    /// Get property `DNS`.
    #[zbus(property(emits_changed_signal = "true"), name = "DNS")]
    fn dns(&self) -> crate::zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// Get property `DNSEx`.
    #[zbus(property(emits_changed_signal = "true"), name = "DNSEx")]
    fn dns_ex(&self) -> crate::zbus::Result<Vec<(i32, i32, Vec<u8>, u16, String)>>;

    /// Get property `FallbackDNS`.
    #[zbus(property(emits_changed_signal = "const"), name = "FallbackDNS")]
    fn fallback_dns(&self) -> crate::zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    /// Get property `FallbackDNSEx`.
    #[zbus(property(emits_changed_signal = "const"), name = "FallbackDNSEx")]
    fn fallback_dns_ex(&self) -> crate::zbus::Result<Vec<(i32, i32, Vec<u8>, u16, String)>>;

    /// Get property `CurrentDNSServer`.
    #[zbus(property(emits_changed_signal = "true"), name = "CurrentDNSServer")]
    fn current_dns_server(&self) -> crate::zbus::Result<(i32, i32, Vec<u8>)>;

    /// Get property `CurrentDNSServerEx`.
    #[zbus(property(emits_changed_signal = "true"), name = "CurrentDNSServerEx")]
    fn current_dns_server_ex(&self) -> crate::zbus::Result<(i32, i32, Vec<u8>, u16, String)>;

    /// Get property `Domains`.
    #[zbus(property(emits_changed_signal = "false"), name = "Domains")]
    fn domains(&self) -> crate::zbus::Result<Vec<(i32, String, bool)>>;

    /// Get property `TransactionStatistics`.
    #[zbus(
        property(emits_changed_signal = "false"),
        name = "TransactionStatistics"
    )]
    fn transaction_statistics(&self) -> crate::zbus::Result<(u64, u64)>;

    /// Get property `CacheStatistics`.
    #[zbus(property(emits_changed_signal = "false"), name = "CacheStatistics")]
    fn cache_statistics(&self) -> crate::zbus::Result<(u64, u64, u64)>;

    /// Get property `DNSSEC`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSSEC")]
    fn dnssec(&self) -> crate::zbus::Result<String>;

    /// Get property `DNSSECStatistics`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSSECStatistics")]
    fn dnssec_statistics(&self) -> crate::zbus::Result<(u64, u64, u64, u64)>;

    /// Get property `DNSSECSupported`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSSECSupported")]
    fn dnssec_supported(&self) -> crate::zbus::Result<bool>;

    /// Get property `DNSSECNegativeTrustAnchors`.
    #[zbus(
        property(emits_changed_signal = "false"),
        name = "DNSSECNegativeTrustAnchors"
    )]
    fn dnssec_negative_trust_anchors(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `DNSStubListener`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSStubListener")]
    fn dns_stub_listener(&self) -> crate::zbus::Result<String>;

    /// Get property `ResolvConfMode`.
    #[zbus(property(emits_changed_signal = "false"), name = "ResolvConfMode")]
    fn resolv_conf_mode(&self) -> crate::zbus::Result<String>;
}

/// Proxy object for `org.freedesktop.resolve1.Link`.
#[proxy(
    interface = "org.freedesktop.resolve1.Link",
    gen_blocking = false,
    default_service = "org.freedesktop.resolve1",
    assume_defaults = false
)]
trait Link {
    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDNS()) Call interface method `SetDNS`.
    #[zbus(name = "SetDNS")]
    fn set_dns(&self, addresses: Vec<(i32, Vec<u8>)>) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDNSEx()) Call interface method `SetDNSEx`.
    #[zbus(name = "SetDNSEx")]
    fn set_dns_ex(&self, addresses: Vec<(i32, Vec<u8>, u16, String)>) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDomains()) Call interface method `SetDomains`.
    #[zbus(name = "SetDomains")]
    fn set_domains(&self, domains: Vec<(String, bool)>) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDefaultRoute()) Call interface method `SetDefaultRoute`.
    #[zbus(name = "SetDefaultRoute")]
    fn set_default_route(&self, enable: bool) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetLLMNR()) Call interface method `SetLLMNR`.
    #[zbus(name = "SetLLMNR")]
    fn set_llmnr(&self, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetMulticastDNS()) Call interface method `SetMulticastDNS`.
    #[zbus(name = "SetMulticastDNS")]
    fn set_multicast_dns(&self, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDNSOverTLS()) Call interface method `SetDNSOverTLS`.
    #[zbus(name = "SetDNSOverTLS")]
    fn set_dns_over_tls(&self, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDNSSEC()) Call interface method `SetDNSSEC`.
    #[zbus(name = "SetDNSSEC")]
    fn set_dnssec(&self, mode: String) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#SetDNSSECNegativeTrustAnchors()) Call interface method `SetDNSSECNegativeTrustAnchors`.
    #[zbus(name = "SetDNSSECNegativeTrustAnchors")]
    fn set_dnssec_negative_trust_anchors(&self, names: Vec<String>) -> crate::zbus::Result<()>;

    /// [📖](https://www.freedesktop.org/software/systemd/man/systemd.directives.html#Revert()) Call interface method `Revert`.
    #[zbus(name = "Revert")]
    fn revert(&self) -> crate::zbus::Result<()>;

    /// Get property `ScopesMask`.
    #[zbus(property(emits_changed_signal = "false"), name = "ScopesMask")]
    fn scopes_mask(&self) -> crate::zbus::Result<u64>;

    /// Get property `DNS`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNS")]
    fn dns(&self) -> crate::zbus::Result<Vec<(i32, Vec<u8>)>>;

    /// Get property `DNSEx`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSEx")]
    fn dns_ex(&self) -> crate::zbus::Result<Vec<(i32, Vec<u8>, u16, String)>>;

    /// Get property `CurrentDNSServer`.
    #[zbus(property(emits_changed_signal = "false"), name = "CurrentDNSServer")]
    fn current_dns_server(&self) -> crate::zbus::Result<(i32, Vec<u8>)>;

    /// Get property `CurrentDNSServerEx`.
    #[zbus(property(emits_changed_signal = "false"), name = "CurrentDNSServerEx")]
    fn current_dns_server_ex(&self) -> crate::zbus::Result<(i32, Vec<u8>, u16, String)>;

    /// Get property `Domains`.
    #[zbus(property(emits_changed_signal = "false"), name = "Domains")]
    fn domains(&self) -> crate::zbus::Result<Vec<(String, bool)>>;

    /// Get property `DefaultRoute`.
    #[zbus(property(emits_changed_signal = "false"), name = "DefaultRoute")]
    fn default_route(&self) -> crate::zbus::Result<bool>;

    /// Get property `LLMNR`.
    #[zbus(property(emits_changed_signal = "false"), name = "LLMNR")]
    fn llmnr(&self) -> crate::zbus::Result<String>;

    /// Get property `MulticastDNS`.
    #[zbus(property(emits_changed_signal = "false"), name = "MulticastDNS")]
    fn multicast_dns(&self) -> crate::zbus::Result<String>;

    /// Get property `DNSOverTLS`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSOverTLS")]
    fn dns_over_tls(&self) -> crate::zbus::Result<String>;

    /// Get property `DNSSEC`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSSEC")]
    fn dnssec(&self) -> crate::zbus::Result<String>;

    /// Get property `DNSSECNegativeTrustAnchors`.
    #[zbus(
        property(emits_changed_signal = "false"),
        name = "DNSSECNegativeTrustAnchors"
    )]
    fn dnssec_negative_trust_anchors(&self) -> crate::zbus::Result<Vec<String>>;

    /// Get property `DNSSECSupported`.
    #[zbus(property(emits_changed_signal = "false"), name = "DNSSECSupported")]
    fn dnssec_supported(&self) -> crate::zbus::Result<bool>;
}
