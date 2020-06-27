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

    {
        // 4th example - Ownership and functions
        // just as in variable 'move', a variable
        // passed into the function moves ownership
        // as well.

        // ex A
        let s1: String = String::from("Ex 4: A:Hello");
        // s1 is in scope after line above

        takes_ownership(s1);
        // s1's value was moved into the function in call above
        // after this call s1 is no longer valid.

        // uncomment to see the error
        // println!("s1 = {}", s1);

        // ex B
        let x:i32 = 5;  // x comes into scope
        makes_copy(x);   // x would normally move into the function
                        // but since x is a 32-bit int on the stack
                        // still ok to use X after the call to make_copy()
        println!("After calling makes_copy() x is still {}", x);
    }

    { 
        // return values and scope
        let s1 = gives_ownership();   // gives_ownership() moves its return
                                      // from the heap into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back()
                                           // s2 is no longer valid
                                           // which moves the return value into s3

        // uncomment to see compilation error
        //   println!("s2 cannot be used since it was moved {}", s2);

     } // s3 dropped out of scope; s2 was moved already; s1 out of scope and dropped
}


fn takes_ownership(str: String) -> i32 // str coming into scope
{
   println!("{}", str);

   5
} // now str out of scope and 'drop' is called... the
  // heap memory of the callee's variable has been dropped / freed.


fn makes_copy(n: i32) -> i32 // n is coming into scope
{
   println!("{}", n);

   20
} // n goes out of scope, nothing special happens 


fn gives_ownership() -> String
{
  // gives_ownership() will move it return value into
  // the variable in the function that calls it
  let str = String::from("Ex 4: C: Hello");
  str
}


fn takes_and_gives_back(a: String) -> String // 'a' comes into scope
{
    a // 'a' is returned and is moved into the var of the callee.
}
