use day_04::part1::process;

#[tracing::instrument]
fn main() {
    let file = include_str!("../../input1.txt");

    let result = process(file);
    println!("{}", result);
}
