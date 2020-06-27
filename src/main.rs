fn main() {

    {
        // var s is not valid here (not yet declared)
        let s = "hello";

        // do stuff with s
        let x = s.len();
        println!("s is {} characters long", x);

    } // scope is over... cannot use s as it is no longer valid

    // if uncommented, error regarding use of s
    //   let y = s.len();


    // 2nd EXAMPLE
    {
        // s is a String (not a literal string) from the heap
        // also declared mutable since the string inside will
        // be modified
        let mut s: String = String::from("2nd ex: hello");

        // do some more stuff - append the literal onto the String var
        s.push_str(", world!");

        // print the string
        println!("{}", s);
    }

    // 3rd EXAMPLE ( Memory and Allocation )
    {
        // a variable of a heap allocated object
        // goes out of scope if that variable is assigned
        // to another variable.

        // for example, this works (because s1 and s2 are stack vars)
        let s1 = "3rd ex: A:hello";
        let s2 = s1;

        println!("s1 = {} ; s2 = {}", s1, s2);
   
        let s3: String = String::from("3rd ex: B:hello");
        let s4: String = s3;
        // uncomment and compile to see error
        // rust calls the assignment of s4 to s3 a "move"
        //  println!("{}", s3);

        // to allow assignments, you can create a deep copy using "clone()"
        let s5: String = String::from("3rd ex: C:hello");
        let s6: String = s5.clone();

        // can use both s5 and s6 here
        println!("s5={} ; s6={}", s5, s6);
    }
}
