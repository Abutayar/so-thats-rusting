pub fn run(){
    // https://doc.rust-lang.org/stable/rust-by-example/hello/print.html 
    // Just Warm UP
    // println!('Hello World') Aaah!! Single Quotes Bad Idea PS: JS Best Practices ;)
    println!("Just Warm UP");

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    println!("31i64 = {} OooooooooOOooooooo", 31i64);

    //TRYOUT - String Literal - Sequential 
    println!("Exploring Move then {}/{}","JavaScript","JS");

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

     // As can named arguments.
     println!("{subject} {verb} {object}",
     object="the lazy dog",
     subject="the quick brown fox",
     verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2i64); //Just Curiosity 


    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // The Idea
    println!("Keep Between {a} {b:>width$} PS: This Didn't Work", a="Rust",b="JavaScript", width=10); // Bad Idea
    //Woe!!! My Curiosity 

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$} Comment :: Why do we need this", number=1, width=6); // Come back to you let

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond"); // Do i really need to Fix this??
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)] // This is something Interesting 
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3)); // This really need to be fixed
    // FIXME ^ Comment out this line. 
    // If you can fix it then comment it COOL IDEA THANK YOU 
}