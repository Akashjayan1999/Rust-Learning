
pub fn main(){

    let c=1;

    match c{
        1=>println!("One"),
        2=>println!("Two"),
        _=>println!("Other")
    }

    let x= Some(5);
    let y= 10;

    match x{
        Some(50)=>println!("x is Some(5)"),
        Some(y)=>println!("x is Some({})",y),
        _=>println!("x is None")
    }

    let x =1;

    match x{
        1|2=>println!("x is 1 or 2"),
        _=>println!("x is not 1")
    }

    match x{
        1..=5=>println!("x is between 1 and 5"),
        _=>println!("x is not 1")
    }

    let x = 'c';
    match x{
        'a'..='z'=>println!("x is between a and z"),
        _=>println!("x is not a")
    }


    struct Point{
       x:i32,
       y:i32,
    }
    
     let p=Point{x:1,y:2};

     match p{
        Point{x:1,y:2}=>println!("x is 1 and y is 2"),
        Point{x:a,y:b}=>println!("x is {} and y is {}",a,b),
        _=>println!("x is not 1 and y is not 2")
     }

     let p=Point{x:1,y:2};

     let Point{x:a,y:b}=p;
     println!("x is {} and y is {}",a,b);

     let Point{x,y}=p;
     println!("x is {} and y is {}",x,y);


     let  p = Point{x:1,y:2};
     match p{
        Point{x:2,y}=>println!("x is 1 and y is 2"),
        Point{x,y:1}=>println!("x is {} and y is {}",x,y),
        _=>println!("anything")
    }



    // Enum
//     enum Message{
//         Quit,
//         Move {x:i32,y:i32},
//         Write(String),
//         ChangeColor(i32,i32,i32),
//     }


//     let msg = Message::ChangeColor(0, 160, 255);
//     match msg {
//        Message::Quit => println!("Quit"),
//        Message::Move { x, y } => println!("Move to {}, {}", x, y),
//        Message::Write(text) => println!("Text: {}", text),
//        Message::ChangeColor(r, g, b) => println!("Change color to {}, {}, {}", r, g, b),
//    }
   enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
   }

    enum Message{
        Quit,
        Move {x:i32,y:i32},
        Write(String),
        ChangeColor(Color),
    }
  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
       Message::Quit => println!("Quit"),
       Message::Move { x, y } => println!("Move to {}, {}", x, y),
       Message::Write(text) => println!("Text: {}", text),
       Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change color to {}, {}, {}", r, g, b),
       Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change color to {}, {}, {}", h, s, v),
   }



   // Advanced 

   let((feet, inches),Point{x,y})=((5,4),Point{x:1,y:2});
   println!("Feet: {}, Inches: {}, X: {}, Y: {}",feet,inches,x,y);

   foo(3,5);
   fn foo(_:i32,y:i32){
    println!("y: {}",y);
   }



   let mut setting_value = Some(5);
   let new_setting_value =  Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => println!("cant overwrite an exist customized value"),
    _ => setting_value = new_setting_value,
}
  println!("{:?}", setting_value);


  let number =(2,3,4,5);
  match number{
      (first,_,third,_)=>println!("first: {}, third: {}",first,third),
  }

  let _x=10;
  let y=10;


  let s=Some(String::from("Hello"));
   if let Some(_)=s{
       println!("found a string");
   }
   if let Some(_s)=s{
       println!("found a string");
   }

   struct Points{
    x:i32,
    y:i32,
    z:i32,
   }
   
   let origin=Points{x:0,y:0,z:0};

   match origin{
    Points {x,..}=>println!("x: {}",x),
   }


   let numbers =(2,3,4,5,6);

   match numbers{
    (first,..,last)=>{print!("first: {}, last: {}",first,last);},
   }

   let num = Some(4);

   match num{
    Some(x) if x<5=>println!("x is less than 5"),
    Some(x) =>println!("x is greater than 5"),
    None=>(),
   }

   let x= Some(5);

   let y=10;
   match x{
    Some(n) if n==y=>println!("n is equal to y"),
    _=>println!("Default Case x={:?}",x),
   }

   let x=4;
   let y =false;

   match x{
    4|5|6 if y=>println!("y is true"),
    _=>println!("y is false"),
   }


   enum Message2{
    Hello {id:i32},
   }

   let msg = Message2::Hello{id:5};

    match msg{
        Message2::Hello { id:id @3..7,} => println!("id is {}",id),
        Message2::Hello { id:10..=12,} => println!("id in anorter range"),

        Message2::Hello{id}=>println!("id is {}",id),
    }

//    println!("{:?}",s); //error
}