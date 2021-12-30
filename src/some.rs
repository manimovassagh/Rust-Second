

pub(crate) fn run (){
    let name ="mani";
    println!("This is my name : {}", name);
    let face = '\u{1F600}';
    println!("this is a smile from Rust {}",face);
    let mut string_sample =String::from("Mani Movassagh from String Module ");
    println!("{}",string_sample);
    string_sample.push_str("Some More");
    println!("{}",string_sample);
       println!("Length of The String is {}",string_sample.len());
   

}