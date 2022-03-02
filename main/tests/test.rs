use main::run;
use futures::executor::block_on;

#[test]
fn test_all() {
    let answers = block_on(run());
    for (num, (a, b)) in main::register_expected_solutions().iter().enumerate() {
        assert_eq!(format!("|Day {}| A: {:?}, B: {:?}", num + 1,  a, b), answers[num])
        // Anther way to test the functions if desired
        // assert_eq!(*a, answers[num].split_whitespace().collect::<Vec<&str>>()[0].replace("\"", ""));
        // assert_eq!(*b, answers[num].split_whitespace().collect::<Vec<&str>>()[1].replace("\"", ""));
    }
}

