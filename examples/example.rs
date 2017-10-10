extern crate ci_info;

fn main() {
    let info = ci_info::get();

    println!("Is CI: {}", info.ci);
    if info.ci {
        println!("Vendor: {:#?}", info.vendor.unwrap());
    }
}
