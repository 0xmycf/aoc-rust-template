use crate::Solvable;

pub struct DayTen;

impl Solvable<String, String> for DayTen {
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
