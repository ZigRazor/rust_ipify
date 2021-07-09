extern crate rust_ipify;
use rust_ipify::ipify::get_ip_json;
use rust_ipify::ipify::get_ip_string;
use rust_ipify::ipify::get_ip4_json;
use rust_ipify::ipify::get_ip4_string;
use rust_ipify::ipify::get_ip4_i32;
use rust_ipify::ipify::get_ip_i128;
fn main() {
    println!("inside main of test ");

    let result_ip64_json = get_ip_json();

    match result_ip64_json {
        Ok(r) => println!("Result IP ( JSON ): \n{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }

    let result_ip64_string = get_ip_string();
    match result_ip64_string {
        Ok(r) => println!("Result IP ( String ): \n\t{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }

    let result_ip4_json = get_ip4_json();

    match result_ip4_json {
        Ok(r) => println!("Result IP v4 ( JSON ): \n{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }

    let result_ip4_string = get_ip4_string();
    match result_ip4_string {
        Ok(r) => println!("Result IP v4 ( String ): \n\t{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }

    let result_ip4_i32 = get_ip4_i32();
    match result_ip4_i32 {
        Ok(r) => println!("Result IP v4 ( i32 ): \n\t{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }

    let result_ip_i128 = get_ip_i128();
    match result_ip_i128 {
        Ok(r) => println!("Result IP ( i128 ): \n\t{:#?}\n", r),
        Err(e) => println!("error in request {:?}", e),
    }


}
