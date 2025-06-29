use traits_objects_gui::{Draw, Screen,Button};

struct SelectBox{
    width: u32,
    height: u32,
    option:Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("Drawing select box");
    }
}
fn main() {
    let screen = Screen{
        components: vec![
            Box::new(Button{
                width: 100,
                height: 100,
                label: "Button".to_string(),
            }),
            Box::new(SelectBox{
                width: 100,
                height: 100,
                option: vec!["Option 1".to_string(),"Option 2".to_string()],
            }),
        ],
    };

    screen.run();
}
