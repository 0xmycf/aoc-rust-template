use crate::Solvable;

pub struct DayFive;

impl Solvable<String, String> for DayFive {
    fn parse(file: &str) -> String {
        String::from("parsed")
    }


    fn part_a(input: &String) -> String {
        String::from("a")
    }


    fn part_b(input: &String) -> String {
        String::from("b")
    }

    fn expected_a() -> String {
        String::from("a")
    }

    fn expected_b() -> String {
        String::from("b")
    }

}
