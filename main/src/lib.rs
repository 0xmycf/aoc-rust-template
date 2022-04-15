pub mod days;

use days::{
    dayone::*,
    daytwo::*,
    daythree::*,
    dayfour::*,
    dayfive::*,
    daysix::*,
    dayseven::*,
    dayeight::*,
    daynine::*,
    dayten::*,
    dayeleven::*,
    daytwelve::*,
    daythirteen::*,
    dayfourteen::*,
    dayfifteen::*,
    daysixteen::*,
    dayseventeen::*,
    dayeighteen::*,
    daynineteen::*,
    daytwenty::*,
    daytwentyone::*,
    daytwentytwo::*,
    daytwentythree::*,
    daytwentyfour::*,
    daytwentyfive::*
    };

use futures::executor::block_on;
use async_trait::*;
use futures::future::BoxFuture;
use std::fs;

/// abstracts over the input and return type of a day
#[async_trait]
pub trait Solvable<A, B: std::fmt::Debug> {
    /// For testing
    /// These functions should only return the
    /// expected values for part a and b respectively
    /// Should be converted to a String so testing it is easy
    fn expected_a() -> String;
    fn expected_b() -> String;

    /// For parsing the input
    fn parse(file: &str) -> A;
    /// solving part a
    fn part_a(input: &A) -> B;
    /// solving part b
    fn part_b(input: &A) -> B;

    /// calls both solutions and returns them in a string,
    /// can be overridden if wanted
    /// Overriding this method will cause the test to fail though
    async fn solve(day: usize, file: String) -> String {
        let parsed = &Self::parse(&file);
        format!("|Day {day}| A: {:?}, B: {:?}", Self::part_a(parsed), Self::part_b(parsed))
    }

}

pub fn get_file_content(a: &str) -> std::io::Result<String> {
    fs::read_to_string(a)
}

/// Makes the next function a bit clearer
type SolutionFunc<'a> = Box<dyn Fn(usize, String) -> BoxFuture<'a, String>>;

/// Returns a Vector which holds all solve functions
fn register_solutions<'a>() -> Vec<SolutionFunc<'a>> {
    let mut solutions: Vec<SolutionFunc> = Vec::new();
    solutions.push(Box::new(DayOne::solve));
    solutions.push(Box::new(DayTwo::solve));
    solutions.push(Box::new(DayThree::solve));
    solutions.push(Box::new(DayFour::solve));
    solutions.push(Box::new(DayFive::solve));
    solutions.push(Box::new(DaySix::solve));
    solutions.push(Box::new(DaySeven::solve));
    solutions.push(Box::new(DayEight::solve));
    solutions.push(Box::new(DayNine::solve));
    solutions.push(Box::new(DayTen::solve));
    solutions.push(Box::new(DayEleven::solve));
    solutions.push(Box::new(DayTwelve::solve));
    solutions.push(Box::new(DayThirteen::solve));
    solutions.push(Box::new(DayFourteen::solve));
    solutions.push(Box::new(DayFifteen::solve));
    solutions.push(Box::new(DaySixteen::solve));
    solutions.push(Box::new(DaySeventeen::solve));
    solutions.push(Box::new(DayEighteen::solve));
    solutions.push(Box::new(DayNineteen::solve));
    solutions.push(Box::new(DayTwenty::solve));
    solutions.push(Box::new(DayTwentyOne::solve));
    solutions.push(Box::new(DayTwentyTwo::solve));
    solutions.push(Box::new(DayTwentyThree::solve));
    solutions.push(Box::new(DayTwentyFour::solve));
    solutions.push(Box::new(DayTwentyFive::solve));
    solutions
}

/// Returns a Vector which holds tuples of the expected inputs (Part a, Part b)
/// for each day
pub fn register_expected_solutions() -> Vec<(String , String)> {
    let mut solutions: Vec<(String              , String)> = Vec::new();
    solutions.push((DayOne::expected_a()        , DayOne::expected_b()));
    solutions.push((DayTwo::expected_a()        , DayTwo::expected_b()));
    solutions.push((DayThree::expected_a()      , DayThree::expected_b()));
    solutions.push((DayFour::expected_a()       , DayFour::expected_b()));
    solutions.push((DayFive::expected_a()       , DayFive::expected_b()));
    solutions.push((DaySix::expected_a()        , DaySix::expected_b()));
    solutions.push((DaySeven::expected_a()      , DaySeven::expected_b()));
    solutions.push((DayEight::expected_a()      , DayEight::expected_b()));
    solutions.push((DayNine::expected_a()       , DayNine::expected_b()));
    solutions.push((DayTen::expected_a()        , DayTen::expected_b()));
    solutions.push((DayEleven::expected_a()     , DayEleven::expected_b()));
    solutions.push((DayTwelve::expected_a()     , DayTwelve::expected_b()));
    solutions.push((DayThirteen::expected_a()   , DayThirteen::expected_b()));
    solutions.push((DayFourteen::expected_a()   , DayFourteen::expected_b()));
    solutions.push((DayFifteen::expected_a()    , DayFifteen::expected_b()));
    solutions.push((DaySixteen::expected_a()    , DaySixteen::expected_b()));
    solutions.push((DaySeventeen::expected_a()  , DaySeventeen::expected_b()));
    solutions.push((DayEighteen::expected_a()   , DayEighteen::expected_b()));
    solutions.push((DayNineteen::expected_a()   , DayNineteen::expected_b()));
    solutions.push((DayTwenty::expected_a()     , DayTwenty::expected_b()));
    solutions.push((DayTwentyOne::expected_a()  , DayTwentyOne::expected_b()));
    solutions.push((DayTwentyTwo::expected_a()  , DayTwentyTwo::expected_b()));
    solutions.push((DayTwentyThree::expected_a(), DayTwentyThree::expected_b()));
    solutions.push((DayTwentyFour::expected_a() , DayTwentyFour::expected_b()));
    solutions.push((DayTwentyFive::expected_a() , DayTwentyFive::expected_b()));
    solutions
}

/// Asynchronously solves all days and returns them if nothing went wrong
/// for day-level tests/answers write functions in the corresponding files
pub async fn run() -> Vec<String> {
    let mut handles = Vec::new();
    let mut counter = 1;

    for (num, i) in register_solutions().iter().enumerate() {
        handles.push(i(counter, get_file_content(&format!("../input/day{}.txt", num + 1)).unwrap_or(String::from("0"))));
        counter += 1;
    }

    futures::future::join_all(handles).await
}

