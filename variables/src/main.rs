fn main() {

    // Variables are immutable by default
    let  n=5;
   
    println!("The value of n is: {n}");

    //Scope
    {
        let n=6;
        println!("The value of n in scope: {n}");
    }
    // The value of n is: 6

    // Shadowing and return value
    // let n= {
    //     6
    // };

    // Return void

    let n= {
        let n=3;
    };
    println!("The value of n in scope: {n:?}");
}
