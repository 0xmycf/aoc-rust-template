#!/bin/sh

# A simple shell script to create the files for each day

days=(
    "DayOne"
    "DayTwo"
    "DayThree"
    "DayFour"
    "DayFive"
    "DaySix"
    "DaySeven"
    "DayEight"
    "DayNine"
    "DayTen"
    "DayEleven"
    "DayTwelve"
    "DayThirteen"
    "DayFourteen"
    "DayFifteen"
    "DaySixteen"
    "DaySeventeen"
    "DayEighteen"
    "DayNineteen"
    "DayTwenty"
    "DayTwentyOne"
    "DayTwentyTwo"
    "DayTwentyThree"
    "DayTwentyFour"
    "DayTwentyFive"
    )

for day in "${days[@]}"; do
    lower=$(echo "$day" | tr '[:upper:]' '[:lower:]')
    USAGE=$(cat <<-//END
use crate::Solvable;

pub struct ${day};

impl Solvable<String, String> for ${day} {
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
//END
    )
    touch "${lower}.rs"
    echo "$USAGE" > "${day}.rs"
done

