fn main() {
    let sum = (1..1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .fold(0, |sum, n| sum + n);
    println!("{}", sum);
}
