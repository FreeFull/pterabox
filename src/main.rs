mod kvm;
mod device;

#[cfg(target_arch = "x86_64")]
fn main() {
    println!("Hello, world!");
}

#[cfg(not(target_arch = "x86_64"))]
fn main() {
    compile_error!("This software only works on x86_64.");
}
