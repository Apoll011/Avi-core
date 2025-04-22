use rhai::def_package;
use rhai::plugin::*;

pub(crate) mod addr;
pub(crate) mod tcp;

// Re-export types.

def_package! {
    /// Package for networking operations.
    pub NetworkingPackage(lib) {
        combine_with_exported_module!(lib, "rhai_net_addr", addr::addr_functions);
        combine_with_exported_module!(lib, "rhai_net_tcp", tcp::tcp_functions);
    }
}
