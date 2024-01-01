use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=./templates/*.html.tera");
    println!("cargo:rerun-if-changed=./container/*");
    println!("cargo:rerun-if-changed=./styles/*");

    let minify_arg = match cfg!(not(debug_assertions)) {
        true => vec!["--minify"],
        false => Vec::new(),
    };

    let output = Command::new("bash")
        .arg("./scripts/tailwind.sh")
        .args(["-i", "styles/styles.scss"])
        .args(["-o", "static/styles.css"])
        .args(&minify_arg)
        .output()
        .expect("Failed to execute tailwind");

    if !output.status.success() {
        println!("cargo:error={}", String::from_utf8(output.stdout).unwrap());
    }

    let output = Command::new("docker")
        .args(["build", "-t", "devdock", "./container/"])
        .output()
        .expect("Failed to executed docker");

    if !output.status.success() {
        println!("cargo:error={}", String::from_utf8(output.stdout).unwrap());
    }
}
