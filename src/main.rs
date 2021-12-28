#[warn(non_snake_case)]

fn main() {
    println!("Hello, world!");
    println!("Hello, Mani!");
    for_loop();
    match_coder();
    match_coder();
    match_coder();

}

fn for_loop(){
    for x in 1..12{
        println!("x is equal to {}", x);

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

    println!("The country name with country code {} is {}", country_code,country)
}