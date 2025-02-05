pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greet(name: &str)-> String{
    format!("Hello {}", name)
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
  

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

   
}

pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess{
       if value<1{
           panic!("Guess value should be between 1 and 100");
           
       }else if value>100{
           panic!("Guess value should be less than 100");
       }
       Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_ne!(result, 5);
    }
    #[test]
    fn test_can_hold_smaller(){
        let larger = Rectangle{width: 10, height: 20};
        let smaller = Rectangle{width: 5, height: 10};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_can_hold_bigger(){
        let smaller = Rectangle{width: 10, height: 20};
        let larger = Rectangle{width: 15, height: 25};
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greet_test(){
        let name = "Akash";
        let result = greet(name);
        assert!(result.contains(name), "Greet result did not contain {}", name);
    }

    #[test]
    #[should_panic]
    fn test_guess_panic(){
        Guess::new(0);
    }

    #[test]
     #[should_panic(expected = "Guess value should be less than 100")]
    fn test_guess_panic2(){
        Guess::new(101);
    }

    #[test]
    fn it_works()-> Result<(),String>{
        if 2+2==4{
            Ok(())
        }else{
            Err(String::from("2+2 is not equal to 4"))
        }
    }
}
