pub mod ssh_tool {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use std::process::Command;
    pub fn generate_ssh_rsa_key(email: &str) {
        let output = Command::new("ssh-keygen")
            .arg("-t")
            .arg("rsa")
            .arg("-C")
            .arg(email)
            .output()
            .expect("failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    pub fn get_ssh_pub_rsa_key() -> String {
        let mut buffer = String::new();
        let mut pub_rsa_key_file_path = dirs::home_dir().unwrap();
        pub_rsa_key_file_path.push(".ssh\\id_rsa.pub");
        let mut rsa_pub_file =
            File::open(PathBuf::from(pub_rsa_key_file_path)).expect("failed to open file");
        rsa_pub_file
            .read_to_string(&mut buffer)
            .expect("failed to read rsa public key");
        buffer
    }
}
