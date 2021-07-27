use std::io;

fn main() {
    
    println!("Faren to Cels OR Cels to Faren");
    println!("Please select the switcher");
    println!("--------------------------");
    println!("1. Faren to Cels");
    println!("2. Cels to Faren");
    
    let mut selector = String::new();

    // std input 
    io::stdin().read_line(&mut selector)
        .expect("Failed to read line");

    let selector: u32 = selector.trim().parse()
        .expect("Please type a number");

    println!("Selector {}", selector);

    let _condition = if selector == 1 {
        println!("The faren to cels {}", faren_to_cels());
    } else if selector == 2 {
        println!("The cels to faren {}", cels_to_faren());
    } else {
        println!("wrong number !!");
    };
}

fn faren_to_cels() -> f64 {
    println!("변환할 화씨온도를 입력해주세요");
    let mut faren = String::new();

    // std input
    io::stdin().read_line(&mut faren)
        .expect("Failed to read line");

    let faren: f64 = faren.trim().parse()
        .expect("Please type a number");

    // faren -> cels
    // println!("The faren is {}", faren);
    return (faren - 32.0) * (5.0/9.0);
}

fn cels_to_faren() -> f64 {
    println!("변환할 섭씨온도를 입력해주세요");
    let mut cels = String::new();

    // std input
    io::stdin().read_line(&mut cels)
        .expect("Failed to read line");

    let cels: f64 = cels.trim().parse()
        .expect("Please type a number");

    // cels -> faren
    // println!("The cels is {}", cels);
    return (cels * (9.0/5.0)) + 32.0;
}
