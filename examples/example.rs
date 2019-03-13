extern crate ci_info;

fn main() {
    // Just check if a CI environment is detected.
    let ci = ci_info::is_ci();
    println!("Is CI: {}", ci);

    // Get CI environment information
    let info = ci_info::get();
    println!("Is CI: {}", info.ci);
    if info.vendor.is_some() {
        println!("Vendor: {:#?}", info.vendor.unwrap());
        println!("Name: {:#?}", info.name.unwrap());
    }
    if info.pr.is_some() {
        println!("Is PR: {:#?}", info.pr.unwrap());
    }
}
