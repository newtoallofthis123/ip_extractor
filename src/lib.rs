use std::fmt::Display;

/// # Network
/// Represents a network interface with it's associated information.
/// The associated information is optional, as it may not be available.
/// Here is what is available:
/// * name: The name of the network interface.
/// * inet: The IP address of the network interface.
/// * broadcast: The broadcast address of the network interface.
/// * netmask: The netmask of the network interface.
/// * mac: The MAC address of the network interface.
/// 
/// This struct is only representational in function and does not
/// contain any methods.
#[derive(Clone)]
pub struct Network {
    pub name: String,
    pub inet: Option<String>,
    pub broadcast: Option<String>,
    pub netmask: Option<String>,
    pub mac: Option<String>,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("name: {}", self.name);

        if let Some(inet) = &self.inet {
            output = format!("{}\ninet: {}", output, inet);
        }

        if let Some(broadcast) = &self.broadcast {
            output = format!("{}\nbroadcast: {}", output, broadcast);
        }

        if let Some(netmask) = &self.netmask {
            output = format!("{}\nnetmask: {}", output, netmask);
        }

        if let Some(mac) = &self.mac {
            output = format!("{}\nmac: {}", output, mac);
        }

        write!(f, "{}", output)
    }
}

/// Internal method to get the output of `ifconfig` split into
/// a vector of strings for each network interface.
/// 
/// # Panics
/// 
/// This method panics if `ifconfig` fails to execute.
fn get_ifconfig_text() -> Vec<String> {
    let output = std::process::Command::new("ifconfig")
        .output()
        .expect("Failed to execute ifconfig.");
    let ifconfig_text = String::from_utf8_lossy(&output.stdout).to_string();

    ifconfig_text
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

/// # Parse Network
/// Parses a string of text from `ifconfig` into a Network struct.
/// This method is mostly always used internally, but can also be
/// used externally if you have a string of text from `ifconfig`.
/// This is not recommended, as it is easier to use the `get_networks`,
/// however, it is possible.
/// 
/// The parsing is done using a simple split algorithm, so don't expect
/// it to be perfect.
/// However, it should work for most cases.
/// 
/// # Arguments
/// 
/// * `line`: The string of text from `ifconfig` to parse.
/// 
/// # Returns
/// 
/// `Network`: A Network struct.
/// 
/// # Example
/// ```
/// use ip_extractor::parse_network;
/// 
/// let network = parse_network("wlp2s0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
///        inet
///       inet6
///      ether
///     ");
/// 
/// println!("{}", network);
/// ```
pub fn parse_network(line: &str) -> Network {
    let mut network = Network {
        name: "".to_string(),
        inet: None,
        broadcast: None,
        netmask: None,
        mac: None,
    };

    network.name = line.split(':').collect::<Vec<&str>>()[0].to_string();

    if let Some(inet) = line.split("inet ").collect::<Vec<&str>>().get(1) {
        let mut inet = inet.to_string();
        inet = inet.split(' ').collect::<Vec<&str>>()[0]
            .to_string()
            .replace('\n', "");
        network.inet = Some(inet);
    }

    if let Some(broadcast) = line.split("broadcast ").collect::<Vec<&str>>().get(1) {
        let mut broadcast = broadcast.to_string();
        broadcast = broadcast.split(' ').collect::<Vec<&str>>()[0]
            .to_string()
            .replace('\n', "");
        network.broadcast = Some(broadcast);
    }

    if let Some(netmask) = line.split("netmask ").collect::<Vec<&str>>().get(1) {
        let mut netmask = netmask.to_string();
        netmask = netmask.split(' ').collect::<Vec<&str>>()[0]
            .to_string()
            .replace('\n', "");
        network.netmask = Some(netmask);
    }

    if let Some(mac) = line.split("ether ").collect::<Vec<&str>>().get(1) {
        let mut mac = mac.to_string();
        mac = mac.split(' ').collect::<Vec<&str>>()[0]
            .to_string()
            .replace('\n', "");
        network.mac = Some(mac);
    }

    network
}

/// # Get Networks
/// 
/// A general method to get all networks on the system.
/// This is the main method that should be used to get all
/// networks on the system.
/// All network interfaces listed are returned, use find_network
/// to find a specific network interface or just use a iterator
/// filter on the returned vector.
///  
/// This is the base for all other methods in this crate.
/// The parsing is done by the `parse_network` method.
/// 
/// # Returns
/// 
/// `Vec<Network>`: A vector of Network structs.
/// 
/// # Example
/// 
/// ```
/// use ip_extractor::{get_networks, Network};
/// 
/// let networks = get_networks();
/// 
/// for network in networks {
///  println!("{}", network);
/// }
/// ```
pub fn get_networks() -> Vec<Network> {
    let mut networks = Vec::new();

    get_ifconfig_text().iter().filter(|x| !x.is_empty()).for_each(|x| {
        networks.push(parse_network(x));
    });

    networks
}

/// # Find Network
/// 
/// A method to find a specific network interface.
/// This method is basically a iterator filter on the
/// `get_networks` method.
/// 
/// # Arguments
/// 
/// * `name`: The name of the network interface to find.
/// 
/// # Returns
/// 
/// `Option<Network>`: An optional Network struct.
/// 
/// # Example
/// 
/// ```
/// use ip_extractor::{find_network, Network};
/// 
/// let network = find_network("wlan");
/// 
/// match network {
///    Some(network) => println!("{}", network),
///   None => println!("No network found."),
/// }
/// ```
pub fn find_network(name: &str) -> Option<Network> {
    get_networks().iter().find(|x| x.name.contains(name)).cloned()
}

/// # Get WLAN
/// 
/// A method to get all wireless networks on the system.
/// Most unix based systems use `wlan` or `wlp` as the
/// prefix for wireless network interfaces.
/// This method is basically a iterator filter on the
/// `get_networks` method for finding them.
/// You can also pass in an optional identifier to fuzzy
/// match a wireless network interface's name.
/// 
/// # Arguments
/// 
/// * `identifier`: An optional identifier to fuzzy match
/// 
/// # Returns
/// 
/// `Vec<Network>`: A vector of Network structs.
/// 
/// # Example
/// 
/// ```
/// use ip_extractor::{get_wlan, Network};
/// 
/// let networks = get_wlan(None);
/// 
/// for network in networks {
///    println!("{}", network);
/// }
/// 
/// let networks = get_wlan(Some("wlp"));
/// 
/// for network in networks {
///   println!("{}", network);
/// }
/// ```
pub fn get_wlan(identifier: Option<&str>) -> Vec<Network> {
    get_networks()
        .iter()
        .filter(|x| {
            (x.name.contains("wlan") || x.name.contains("wlp"))
             && x.inet.is_some() &&
            match identifier {
                Some(ref identifier) => x.name.contains(identifier),
                None => true,
            }
        })
        .cloned()
        .collect::<Vec<Network>>()
}

/// # Get Ethernet
/// 
/// Similar to `get_wlan`, this method gets all ethernet
/// network interfaces on the system.
/// 
/// # Arguments
/// 
/// * `identifier`: An optional identifier to fuzzy match
/// 
/// # Returns
/// 
/// `Vec<Network>`: A vector of Network structs.
/// 
/// # Example
/// 
/// ```
/// use ip_extractor::{get_ethernet, Network};
/// 
/// let networks = get_ethernet(None);
/// 
/// for network in networks {
///   println!("{}", network);
/// }
/// 
/// let networks = get_ethernet(Some("enp"));
/// 
/// for network in networks {
///  println!("{}", network);
/// }
/// ```
/// 
/// # Note
/// 
/// This method is not tested on a system with multiple
/// ethernet network interfaces, so it may not work as
/// expected.
pub fn get_ethernet(identifier: Option<&str>) -> Vec<Network> {
    get_networks()
        .iter()
        .filter(|x| {
            (x.name.contains("eth") || x.name.contains("enp"))
             && x.inet.is_some() &&
            match identifier {
                Some(ref identifier) => x.name.contains(identifier),
                None => true,
            }
        })
        .cloned()
        .collect::<Vec<Network>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_get_networks_work() {
        let networks = get_networks();
        assert!(!networks.is_empty());
    }

    #[test]
    fn does_wlan_work(){
        let wlan = get_wlan(None);
        assert!(!wlan.is_empty())
    }
}
