# Rust ipify Library
A rust client library for [ipify](https://www.ipify.org/)

<img src="https://img.shields.io/badge/rust%20-%2300599C.svg?&style=for-the-badge&logo=rust%2B%2B&ogoColor=white"/> <img src="https://img.shields.io/badge/github%20-%23121011.svg?&style=for-the-badge&logo=github&logoColor=white"/>

[![GitHub license](https://img.shields.io/github/license/ZigRazor/rust_ipify.svg)](https://github.com/ZigRazor/rust_ipify/blob/master/LICENSE)
[![GitHub release](https://img.shields.io/github/release/ZigRazor/rust_ipify.svg)](https://GitHub.com/ZigRazor/rust_ipify/releases/)

[![Generic badge](https://img.shields.io/badge/Build-Passed-Green.svg)](https://shields.io/) [![Generic badge](https://img.shields.io/badge/UnitTest-Passed-Green.svg)](https://shields.io/)

## Introduction

**Rust ipify** is a client library in Rust language for ipify.   

## Why ipify?
Ever needed to get your public IP address programmatically?
Maybe you're provisioning new cloud servers and need to know your IP -- maybe you're behind a corporate firewall and need to tunnel information -- whatever the reason: sometimes having a public IP address API is useful!

You should use ipify because:

 1. You can use it without limit (even if you're doing millions of requests per minute.

 2. It works flawlessly with both IPv4 and IPv6 addresses, so no matter what sort of technology you're using, there won't be issues.

 3. It's always online and available, and its infrastructure is powered by Heroku, which means that regardless of whether the server running the API dies, or if there's an enormous tornado which destroys half of the east coast, ipify will still be running!

 4. ipify is completely open source (check out the GitHub repository).

 5. No visitor information is ever logged. Period.

 6. Lastly, ipify is funded by Randall Degges, so there's no need to worry about the domain name disappearing in three years or anything like that: ipify is here to stay!

## Requirements

The library requires the Rust Edition >= 2018

## How to Compile

The repository contains 2 folder. The first one *Rust_ipify_lib* contains the sources of the library, the second one *Rust_ipify_lib_test* contains the sources of the test.
- To compile the library:
  - `cd  Rust_ipify_lib`
  - `cargo build`
- To compile the test:
  - `cd  Rust_ipify_lib_test`
  - `cargo build`

## How to Run

- To run the test:
  - `cd  Rust_ipify_lib_test`
  - `cargo run`

## Example

The example is provided under the `Rust_ipify_lib_test/src/` folder.
The file contains an example of usage of the API of the library to retrive the global IP:
    
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

## How to contribute 
[![GitHub contributors](https://img.shields.io/github/contributors/ZigRazor/rust_ipify.svg)](https://GitHub.com/ZigRazor/rust_ipify/graphs/contributors/)

If you want give your support you can create a ***pull request***   [![GitHub pull-requests](https://img.shields.io/github/issues-pr/ZigRazor/rust_ipify.svg)](https://GitHub.com/ZigRazor/rust_ipify/pull/) 

or report an ***issue***  [![GitHub issues](https://img.shields.io/github/issues/ZigRazor/rust_ipify.svg)](https://GitHub.com/ZigRazor/rust_ipify/issues/).

If you want to change the code, or fix issue, or implement a new feature please read our [CONTRIBUTING Guide](https://github.com/ZigRazor/rust_ipify/blob/master/CONTRIBUTING.md).

## Site

Checkout last released version from Crates.io Site [here](https://crates.io/crates/rust_ipify)

## Contact
E-Mail : zigrazor@gmail.com

[GitHub Profile](https://github.com/ZigRazor) ![Profile views](https://gpvc.arturio.dev/ZigRazor)

![ZigRazor's github stats](https://github-readme-stats.vercel.app/api?username=ZigRazor&show_icons=true&title_color=fff&icon_color=79ff97&text_color=9f9f9f&bg_color=151515)

## Support
To support me just add ***Star*** the project  [![GitHub stars](https://img.shields.io/github/stars/ZigRazor/rust_ipify.svg?style=social&label=Star&maxAge=2592000)](https://GitHub.com/ZigRazor/rust_ipify/stargazers/) or ***follow me***  [![GitHub followers](https://img.shields.io/github/followers/ZigRazor.svg?style=social&label=Follow&maxAge=2592000)](https://github.com/ZigRazor?tab=followers)
