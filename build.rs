use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=./templates/*.html.tera");
    println!("cargo:rerun-if-changed=./styles/*");

    let output = Command::new("bash")
        .arg("./scripts/tailwind.sh")
        .args(["-i", "styles/styles.scss"])
        .args(["-o", "static/styles.css"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        println!("cargo:error={}", String::from_utf8(output.stdout).unwrap());
    }
}
