use state_pattern::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a Salad for lucj today");
   

   let post = post.request_review();
    

    let post = post.approve();
    assert_eq!("I ate a Salad for lucj today", post.content());
}
