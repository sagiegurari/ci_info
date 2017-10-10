extern crate ci_info;

fn main() {
    // Just check if a CI environment is detected.
    let is_ci = ci_info::is_ci();
    println!("Is CI: {}", is_ci);

    // Get CI environment information
    let info = ci_info::get();
    println!("Is CI: {}", info.ci);
    if info.ci {
        println!("Vendor: {:#?}", info.vendor.unwrap());
    }
}
