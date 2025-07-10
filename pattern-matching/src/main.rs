mod matching_literals;
fn main() {
    matching_literals::main();
    #[derive(Debug)]
    enum Language{
        English,
        Spanish,
        Russian,
        Japanese,
    }

    let language = Language::English;

    match language{
        Language::English => println!("English"),
        Language::Spanish => println!("Spanish"),
        Language::Russian => println!("Russian"),
        lang=> println!("{:?}", lang),
        // _ => println!("Japanese"),
    }

    // While let
    let mut stack =Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }


    let v= vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate(){
        println!("{}: {}", index, value);
    }


    // Let statement

    let x=5;

    let (x,y,_)=(1,2,3);


    // Function Parameters

    let point=(3,4);
    print_coordinates(&point);

    //Trrefutable and refutable patterns

    // Irrefutable pattern

    let x=5;

    // Refutable pattern
    let x:Option<&str> =None;
    if let Some(x)=x{
        println!("{}",x);
    }

    let x:Option<&str>=None;
    // let Some(x)=x; through error because x is not refutable

    // if let x=5{ // always true
    //     println!("{}",x);
    // }
}

fn print_coordinates(&(x,y):&(i32,i32)){
    println!("x: {}, y: {}", x, y);
}