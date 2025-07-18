use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;


//Custom derive macro

#[derive(HelloMacro)]
struct Pancake;

fn main() {
    Pancake::hello_macro();
}

//Attribute-like macro

#[route(GET, "/")]
fn index() {}


#[proc_macro_attribute]
pub fn route(attr: TokenStream //GET ,"/"
            , item: TokenStream //fn index() {}
) -> TokenStream {

}


// Function-like macros

let sql = sql!(SELECT * FROM users WHERE id = 1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}