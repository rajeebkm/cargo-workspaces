use blog_shared::Post;

fn main() {
    let post = Post::new(
        "Hello, world!".to_owned(),
        "This is a test post.".to_owned(),
    );
    println!("{:?}", post);
}
