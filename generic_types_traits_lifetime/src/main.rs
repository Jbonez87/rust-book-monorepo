mod function_abstraction;
mod generics_example;
mod generic_structs;
mod generic_methods;
mod generic_enums;
mod aggregator;
mod lifetimes_examples;

use function_abstraction::{code_duplication, non_duplicate_code};
use generics_example::{generics_practice};
use generic_structs::{Point};
use generic_methods::{DoubleTypePoint, SingleTypePoint};
use generic_enums::generic_to_concrete;
use aggregator::{Summary, Tweet, NewsArticle, trait_parameter_notify, trait_bound_notify, return_summarizable};
use lifetimes_examples::{correct_reference, longest};


fn main() {
    code_duplication();
    non_duplicate_code();
    generics_practice();
    let integer = Point { x: 5, y: 4 };
    println!("Integer is: {:?}", integer);

    let float = Point { x: 7.5, y: 3.8 };
    println!("Float is: {:?}", float);

    /* 
      This will not work due to the mismatched integer and float types.
      In the struct definition, <T> lets the compiler know that both
      `x` and `y` should be the same type.
    */
    /* let wont_work = Point { x: 3, y: 7.5 }; */

    let both_integer = Point { x: 3, y: 7 };
    println!("both_integer is: {:?}", both_integer);

    let both_float = Point { x: 7.8, y: 3.3 };
    println!("both_float is: {:?}", both_float);

    let integer_and_float = Point { x: 4, y: 3.4 };
    println!("integer_and_float is: {:?}", integer_and_float);

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let floating_point: SingleTypePoint<f32> = SingleTypePoint {
        x: 1.5,
        y: 1.5,
    };

    println!("The distance from origin (0.0, 0.0) is: {}", floating_point.distance_from_origin());

    println!("floating_point.x is: {}", floating_point.x());

    let p1 = DoubleTypePoint {
        x: 5,
        y: 2.4
    };

    let p2 = DoubleTypePoint {
        x: "Hello",
        y: 'c',
    };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    generic_to_concrete();

    /*
      At compile time, Rust alters the definitions of the 
      `Option` enum similarly to how the `Option_f64` and 
      `Option_i32` enums are defined and used in the 
      generic_enums module
     */
    let integer = Some(5);
    let float = Some(6.0);

    println!("float = {:?}, integer = {:?}", float, integer);

    let tweet = Tweet {
        username: String::from("John_Castrillon"),
        content: String::from("Learning Rust is fun."),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("UCONN wins NCAA Championship!"),
        location: String::from("Storrs, CT, USA"),
        author: String::from("John Castrillon"),
        content: String::from("The UCONN Huskies won their 5th national championship!")
    };

    println!("New article available! {}", article.summarize());

    trait_parameter_notify(&article);

    trait_bound_notify(&article);

    return_summarizable();

    correct_reference();

    let string1 = String::from("abcd");
    let string2 = "def";

    let result = longest(&string1.as_str(), &string2);
    println!("The longest string is: {}", result);

    let string1 = String::from("This is the absolute longest string!");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1.as_str(), &string2.as_str());

        println!("The longest string is: {}", result);
    }

    /*
      Here is a scenario where our code will fail at compile
      time due to the lifetime of our variables and the scope.
      Note that the code executed within the curly braces does
      not live as long.
     */
    // let string1 = String::from("This is the absolute longest string!");

    // {
    //     let string2 = String::from("xyz");
    //     let new_result = longest(&string1.as_str(), &string2.as_str());
    // }
    // println!("The longest string is: {}", new_result);
}
