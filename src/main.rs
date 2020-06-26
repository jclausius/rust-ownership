fn main() {
    println!("Hello, world!");

    {
        // var s is not valid here (not yet declared)
        let s = "hello";

        // do stuff with s
        let x = s.len();

    } // scope is over... cannot use s as it is no longer valid

    // if uncommented, error regarding use of s
    //   let y = s.len();
    
}
