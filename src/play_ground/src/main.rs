fn main() {
    
    println!("Hello, world!");
    
    //let x = 5;

    let mut x = 5;
    println!("[before] simple example variable mutable: {}", x);
    
    x=6;
    println!("[after] simple example variable mutable: {}", x);
    
    //i8, u8, i16, i32, u32, i64, u64
    //f32, f64

    let sum = 1 + 20;
    println!("sum: {}", sum);

    let subtraction = 30 - 20;
    println!("subtraction: {}", subtraction);
    
    let multiplication = 5 * 10;
    println!("multiplication: {}", multiplication);

    let division = 100 / 20;
    println!("division: {}", division);
    
    let remainder = 49 % 2;
    println!("Mod: {}", remainder);

    let _c = 'c';
    let _z:char = 'z';

    let tuple:(i32, f64, char) = (42, 6.12, 'Z');

    println!("Tuple {:?}", tuple);

    let (z1, y, x) = tuple;

    println!("z1, y, x {:?} {} {}", z1, y, x);

    let (_, _, x1) = tuple;

    println!("{}", x1);

    let array = [1,2,3,4,5,6,7,8,9,10];

    println!("array: {:?}", array);

    let a1 = array[0];

    println!("a1: {}", a1);
 }