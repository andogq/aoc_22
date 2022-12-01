pub trait Day {
    type Input: Clone;
    type Output;

    fn part_1(input: Self::Input) -> Self::Output;
    fn part_2(input: Self::Input) -> Self::Output;

    fn parse(raw: &str) -> Self::Input;

    fn run(input: &str) -> (Self::Output, Self::Output) {
        let input = Self::parse(input);

        (Self::part_1(input.clone()), Self::part_2(input))
    }
}
