use state_pattern_trad::blog::Post;

fn main() {

    // creates a new empty post with the draft state
    let mut post = Post::new();
    
    // the content for a draft is inaccessible, so the .content() always returns the empty &str
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // calling .request_review() makes the state into a PendingReview
    // again, .content() returns the empty &str
    post.request_review();
    assert_eq!("", post.content());

    // only after the post has been approved will the .content() method return anything
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

}
