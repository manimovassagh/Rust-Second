use core::panicking::AssertKind::Match;

#[warn(non_snake_case)]

fn main() {
    println!("Hello, world!");
    println!("Hello, Mani!");
    for_loop();
}

fn for_loop(){
    for x in 1..12{
        println!("x is equal to {}", x);
        print!("some\n other")
    }
}

fn match_coder(){
    let country_code=98;
    let country=match country_code {
        98=>"Iran",
        44=>"UK",
        01=>"USA",
        1..=1000 => "unknown",
        _ => "invalid",
    };
}