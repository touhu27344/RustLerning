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

    /*
    //配列型の宣言と型注釈
    let a = [1,2,3,4,5];
    let _months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    let a:[i32;5] = [1,2,3,4,5];
    //配列の要素の呼び出し
    let first = a[0];
    let second = a[1];

    println!("The value of first element in a is: {}",first);
    println!("The value of second element in a is: {}",second);
    */

    //
    //関数
    //
    /*
    //関数の定義 (関数_i)
    println!("Hello, world!");

    another_function(5);

    //関数の引数（関数_ii)
    print_labeled_mesurement(5,'h');

    //戻り値のある関数(関数_iii)
    let x = plus_one(5);
    println!("The value of x is:{}", x);
    */

    //
    //if文
    //

    //ifとelse if
    let number = 7;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }

    //let文内でif式を扱う
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is:{}",number);
}

//関数_i
fn another_function(x: i32) {
    println!("The value of x is: {}",x);
}

//関数_ii
fn print_labeled_mesurement(value: i32, unit_label: char) {
    println!("The mesurement is: {}{}",value,unit_label);
}

//関数_iii
fn plus_one(x: i32) -> i32 {
    x+1 //戻り値には;を付けない。
}
