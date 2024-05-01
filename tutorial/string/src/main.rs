fn main() {
    // println!("Hello, world!");
    // let x = "rajesh shedolkar";
    let s = String::from("Rajesh Shedolkar");
    let y = s.chars().nth(7);

    // This works if not out of index
    // print!("by unwrapping char: {}\n", y.unwrap());

    match y {
        Some(z) => {
            print!("char: {}\n", z);
        },
        None => {
            print!("No value \n");
        }
    }
    // print!("{}", y);

    let is_even = false;
    if is_even{
        print!("even number");
    } else if !is_even{
        print!("Odd number");
    }
    

    for i in 0..10{
        print!("\n{}", i);
    }
    

    // iterate over arrays, map, strings 
    let str = String::from("Hello World Rust");
    print!("first word: {}", first_word(str));


    let num = String::from("Rajesh Shedolkar");
    onwner_ship(num);
    print!("num after function call: {}", num);
    print!("\n");
}


fn onwner_ship(num: String){
    print!( "num in ownership function: {}", num);
}



fn first_word(str: String) -> String{
    let mut ans = String::from("");
    for c in str.chars() {
        print!("\n{}", c);
        ans.push(c);
        if c == ' '{
            break;
        }
    }

    for (i, c) in str.chars().enumerate(){
        print!("({}, {}), ", i, c);
    }

    return ans;
}
