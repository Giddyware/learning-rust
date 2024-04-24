fn main() {

    let name = "Hello".to_string();

    // Shadowing to access variable
    // let name =greet(name);


    //clone can be slow prod
    // greet(name.clone());
    // greet(name);



    greet_borrow(&name);
    let mut name_mut = "Hello".to_string();
    greet_borrow_mut(&mut name_mut);

    println!("{name_mut}");
}


//Borrow 
//&-allows borrowing without mutation
fn greet_borrow(s: &String) {
    println!("{s}")
}

// Borrow with mutation
fn greet_borrow_mut(s:&mut String) {
    // println!("Hello {s}")
    *s = format!("Hello {s}");
}
//Moving back and using shadowing
// fn greet(s: String) -> String {
//     print!("Hello {s}");
//     s
// }

// fn double_or_nothing(n:i32) -> i32 {
//     if(n > 0){
//         return n * 2;
//     }else{
//         return 0
//     }
// }

// fn double(n:i32)->i32 {
//     n * 2
// }