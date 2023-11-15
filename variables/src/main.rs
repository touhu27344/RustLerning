fn main() {
    /*
    let guess: i32 = "-43".parse().expect("Not a number!");
    println!("{}",guess);
    */

    //
    //タプル型
    //
    /*
    // タプル型の型注釈と代入、
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x,y,_z) = tup;

    println!("The value of y is: {}", y);

    //タプル型の要素の呼び出し。
    let five_hunderd = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of first element in tup is: {}", five_hunderd);
    println!("The value of second element in tup is: {}", six_point_four);
    println!("The value of third element in tup is: {}", one);
    */

    //
    //配列型
    //

    //配列型の宣言と型注釈
    let a = [1,2,3,4,5];
    let _months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    let a:[i32;5] = [1,2,3,4,5];
    //配列の要素の呼び出し
    let first = a[0];
    let second = a[1];

    println!("The value of first element in a is: {}",first);
    println!("The value of second element in a is: {}",second);



}
