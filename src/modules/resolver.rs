use log::{info,debug};
use colored::Colorize;

use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::HashMap;
use std::time::Duration;

/// Function to resolve IP address from the ldap FQDN
/// <https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/index.html>
/// <https://github.com/shadowsocks/shadowsocks-rust/blob/master/crates/shadowsocks-service/src/config.rs>
pub async fn fqdn_resolver(dns_tcp: bool, ldapip: &String, name_server: &String, fqdn_ip: &mut HashMap<String, String>, vec_computer: &Vec<serde_json::value::Value>)
{
   info!("Resolving FQDN to IP address started...");
   let mut c = ResolverConfig::new();
   let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);
   let mut dns_protocol = Protocol::Udp;

   if dns_tcp == true
   {
      dns_protocol = Protocol::Tcp;
   }

   if name_server.contains("127.0.0.1") {
      let address = ldapip.parse::<IpAddr>().unwrap_or(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
      socket.set_ip(address);
   }
   else
   {
      let address = name_server.parse::<IpAddr>().unwrap_or(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
      socket.set_ip(address);
   }
   

   debug!("Protocol DNS: {:?}",&dns_protocol);
   debug!("Name server DNS: {:?}",name_server.parse::<Ipv4Addr>());

   c.add_name_server(NameServerConfig {
      socket_addr: socket,
      protocol: dns_protocol,
      tls_dns_name: None,
      trust_nx_responses: false,
      bind_addr: None,
   });

   let mut o = ResolverOpts::default();
   o.timeout = Duration::new(0, 5);
   
   // Construct a new Resolver with default configuration options
   let resolver = TokioAsyncResolver::tokio(c,o).unwrap();

   for value in fqdn_ip.to_owned()
   {
      for i in 0..vec_computer.len()
      {
          if (vec_computer[i]["Properties"]["name"].as_str().unwrap().to_string() == value.0.to_owned().to_string()) && (vec_computer[i]["Properties"]["enabled"] == true) {
            debug!("Trying to resolve FQDN: {}",value.0.to_string());
            // #todo: ()?; error management if failed message but dont exit program
            let result = resolver.lookup_ip(value.0.to_string());
            match result.await{
               Ok(response) => {
                  let address = response.iter().next().expect("no addresses returned!");
                  if address.is_ipv4() {
                     fqdn_ip.insert(value.0.to_owned().to_string(),address.to_owned().to_string());
                     info!("IP address for {}: {}",&value.0.to_string().yellow().bold(),address.to_owned().to_string().yellow().bold());
                  }
               },
               Err(_err) => continue,
            };
            continue
          }
         continue
      }
   }
   info!("Resolving FQDN to IP address finished!");
}