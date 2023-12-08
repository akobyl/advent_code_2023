use day_06::part2::process;


#[tracing::instrument]
fn main() {
    let file = include_str!("../../input2.txt");


    let result = process(file);
    println!("{}", result);

}
