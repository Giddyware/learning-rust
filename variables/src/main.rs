fn main() {

println!("{}", double(5));

let i=5;
let n = if i==5
{
    6
}else{
    2
};
}


fn double_or_nothing(n:i32)->i32 {
    if(n>0){
        return n * 2;
    }else{
        return 0
    }
}

fn double(n:i32)->i32 {
    n * 2
}