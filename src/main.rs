fn main() {

    {
        // var s is not valid here (not yet declared)
        let s = "hello";

        // do stuff with s
        let x = s.len();

    } // scope is over... cannot use s as it is no longer valid

    // if uncommented, error regarding use of s
    //   let y = s.len();


    // 2nd EXAMPLE
    {
        // s is a String (not a literal string) from the heap
        // also declared mutable since the string inside will
        // be modified
        let mut s = String::from("hello");

        // do some more stuff - append the literal onto the String var
        s.push_str(", world!");

        // print the string
        println!("{}", s);
    }

    // 3rd EXAMPLE ( Memory and Allocation )
}
