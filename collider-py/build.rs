use std::process::Command;
pub fn main() {
    // Install setuptools
    let output = Command::new("pip")
        .arg("install")
        .arg("setuptools")
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        eprintln!("Command failed with status: {}", output.status);
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Installation of setuptools failed");
    }

    // Run distutils to get the library path
    let python_inline_script =
        "from distutils import sysconfig;print(sysconfig.get_config_var('LIBDIR'))";
    let output = Command::new("python")
        .arg("-c")
        .arg(python_inline_script)
        .output()
        .expect("Failed to get LIBDIR");
    if !output.status.success() {
        eprintln!("Python command failed with status: {}", output.status);
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Python command failed");
    }

    // Get LIBDIR
    let python_libdir = {
        let output = Command::new("python")
            .arg("-c")
            .arg("from distutils import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
            .output()
            .expect("Failed to get LIBDIR");
        if !output.status.success() {
            panic!("Failed to get LIBDIR");
        }
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    // Get LIBRARY (e.g., python3.11)
    let python_lib = {
        let output = Command::new("python")
            .arg("-c")
            .arg("from distutils import sysconfig; print(sysconfig.get_config_var('LDLIBRARY'))")
            .output()
            .expect("Failed to get LDLIBRARY");
        if !output.status.success() {
            panic!("Failed to get LDLIBRARY");
        }
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    // Remove 'lib' prefix and '.dylib' or '.a' suffix to get the correct name for rustc-link-lib
    let libname = python_lib
        .strip_prefix("lib")
        .unwrap_or(&python_lib)
        .trim_end_matches(".dylib")
        .trim_end_matches(".a");

    println!("cargo:rustc-link-search=native={}", python_libdir);
    println!("cargo:rustc-link-lib=dylib={}", libname);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", python_libdir);
}
