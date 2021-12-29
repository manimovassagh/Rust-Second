mod some;
mod strings;
mod touples;
mod arrays;

// use std::net::TcpListener;

#[warn(non_snake_case)]
fn main() {
    println!("Hello, world!");
    println!("Hello, Mani!");
    for_loop();
    match_coder();
    match_coder();
    match_coder();
    some::run();
    // strings::run();
    touples::run();
    arrays::run();

    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //
    //     println!("Connection established!");
    // }
}

fn for_loop() {
    for x in 1..12 {
        println!("x is equal to {}", x);
    }
}

fn match_coder() {
    let country_code = 98;
    let country = match country_code {
        98 => "Iran",
        44 => "UK",
        01 => "USA",
        1..=1000 => "unknown",
        _ => "invalid",
    };

    println!("The country name with country code {} is {}", country_code, country)
}