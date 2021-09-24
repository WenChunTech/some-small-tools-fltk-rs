pub mod ssh_tool;
use self::ssh_tool::ssh_tool::get_ssh_pub_rsa_key;

pub fn use_test() -> String {
    get_ssh_pub_rsa_key()
}