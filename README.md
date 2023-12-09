# IP Extractor

A Simple crate that extracts IP addresses for a unix based system.
It is basically a wrapper around the `ifconfig` command.

> This is my first crate, so please be gentle.

Keep reading for a mini doc, or you can just go to the [docs](https://docs.rs/crate/ip_extractor/latest).

## Usage

```rust
use ip_extractor::find_network;

fn main() {
    let networks = get_networks().unwrap();

    networks.iter().for_each(|network| {
        println!("{}", network);
    });
}
```

## How it works and Warning

This crate is basically a wrapper around the `ifconfig` command. It runs the command and then parses the output to get the IP addresses.

The parsing is done using a simple split algorithm, so don’t expect it to be perfect. However, it should work for most cases.

Moreover, this crate only works on systems that have the ifconfig command. So it won’t work on windows. I mean, you can probably use it with WSL or Git Bash, but I haven’t tested it. So if you do, please let me know.

There is an internal function that actually executes the command and parses the output. However, it is not exposed to the user. This is the function that failed if the ifconfig command is not found.

## Mini Doc

This is a simple crate that only has like 5 main functions. So if you have some programming knowledge, in rust, going through the [docs](https://docs.rs/crate/ip_extractor/latest) would be a breeze. However, if you want a quick overview, here is the mini doc.

### `Network` Struct

The `Network` struct is a simple struct that is meant to be the representation of a network interface. It has 5 fields as of version `0.1.0`.

```rust
pub struct Network {
    pub name: String,
    pub inet: String,
    pub mac: String,
    pub netmask: String,
    pub broadcast: String,
}
```

The ip of the network interface is stored in the `inet` field if you are wondering.

This struct only representational, hence, it does not have any methods. By the way, it implements the `Display` trait, so you can print it out directly.

### `get_networks` Function

This is the main function of the crate. It returns a `Vec<Network>` which is a vector of all the network interfaces on the system.

Remember it returns the networks directly, so you can use it like this:

> Signature: `get_networks() -> Vec<Network>`

```rust
get_networks().iter().for_each(|network| {
    println!("{}", network);
});
```

### `find_network` Function

A function that fuzzy searches for a network interface. It takes a `&str` as an argument and returns an `Option<Network>`.

> Signature: `find_network(&str) -> Option<Network>`

```rust
use ip_extractor::{find_network, Network};
 
let network = find_network("wlan");
 
match network {
   Some(network) => println!("{:?}", network),
  None => println!("No network found."),
}
```

### `parse_network` Function

This is an internal function that is used to parse the output of the `ifconfig` command. It takes a `&str` as an argument and returns a `Network`.

The parsing is done using a simple split algorithm, so don’t expect it to be perfect. However, it should work for most cases.

> Signature: `parse_network(&str) -> Network`

```rust
use ip_extractor::parse_network;
 
let network = parse_network("wlp2s0: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500
       inet
      inet6
     ether
    ");
 
println!("{:?}", network);
```

### `get_wlan` Function

A method to get all wireless networks on the system. Most unix based systems use wlan or wlp as the prefix for wireless network interfaces.

Hence this method is basically a iterator filter on the get_networks method for finding them. You can also pass in an optional identifier to fuzzy match a wireless network interface’s name.

> Signature: `get_wlan(Optional<&str>) -> Vec<Network>`

```rust
use ip_extractor::{get_wlan, Network};
 
let networks = get_wlan(None);
 
for network in networks {
   println!("{:?}", network);
}
 
let networks = get_wlan(Some("wlp"));
 
for network in networks {
  println!("{:?}", network);
}
```

### `get_ethernet` Function

This is similar to the `get_wlan` function, but it is for ethernet network interfaces. The prefix for ethernet network interfaces is usually eth or enp.

> Signature: `get_ethernet(Optional<&str>) -> Vec<Network>`

```rust
use ip_extractor::{get_ethernet, Network};
 
let networks = get_ethernet(None);
 
for network in networks {
  println!("{:?}", network);
}
 
let networks = get_ethernet(Some("enp"));
 
for network in networks {
 println!("{:?}", network);
}
```

> Note: The `get_wlan` and `get_ethernet` functions are just iterators over the `get_networks` function. So you can use the `get_networks` function directly if you want.

## Contributing

If you want to contribute to this crate, you can do so by opening an issue or a pull request. I would really appreciate it.

## License

This crate is licensed under the MIT license. You can read the license [here](LICENSE). So basically, as long as the thing you are using it for is legal, you can use it (At least, that’s what I think it means).
