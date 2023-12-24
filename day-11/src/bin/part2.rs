use day_11::part2::process;

#[tracing::instrument]
fn main() {
    let file = include_str!("../../input2.txt");

    let result = process(file, 1000000);
    println!("{}", result);
}
