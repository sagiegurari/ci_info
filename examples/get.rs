use ci_info;

fn main() {
    // Just check if a CI environment is detected.
    let ci = ci_info::is_ci();
    println!("Is CI: {}", ci);

    // Get CI environment information
    let info = ci_info::get();
    println!("Is CI: {}", info.ci);
    if let Some(vendor) = info.vendor {
        println!("Vendor: {:#?}", vendor);
        println!("Name: {:#?}", info.name.unwrap());
    }
    if let Some(pr) = info.pr {
        println!("Is PR: {:#?}", pr);
    }
    if let Some(branch_name) = info.branch_name {
        println!("Branch Name: {:#?}", branch_name);
    }
}
