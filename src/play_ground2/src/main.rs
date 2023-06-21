fn main() {
    let t = (1, 'a', true);

    let f = (2, t);

    println!("{:?}" , f);

    println!("{:#?}" , f);
}