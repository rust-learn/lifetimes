use std::fmt::Display;

// lifetime annotation for function
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    }
    str2
}

// lifetime annotation for struct
// This annotation means an instance of 'ImportantExcerpt' can’t outlive the reference it holds in its 'part' field.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// <'a> is required after `impl` because it's part of the struct's type of ImportantExcerpt<'a>
impl<'a> ImportantExcerpt<'a> {
    // due to Lifetime Elision rule 1 & 2, we don't need to annotate the lifetimes of input param `self` and return type
    fn level(&self) -> i32 {
        3
    }

    // due to Lifetime Elision rule 1 & 3, we don't need to annotate the lifetimes
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could no find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // static lifetime: `'static`, meaning this reference can live for the entire duration of the program
    // the lifetime of all string literals is `'static`
    let s: &'static str = "I have a static lifetime.";
}
