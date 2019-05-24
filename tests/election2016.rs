//! You need to have downloaded the CSV data from the AEC before running these tests.
//! Run with `cargo test --release -- --ignored`

extern crate aus_senate;

use aus_senate::election2016;
use aus_senate::util::Int;

#[test]
#[ignore]
fn nsw_2016_election() {
    let num_senators = 12;
    let result = election2016::run(
        "data/candidate_ordering.csv",
        "data/NSW.csv",
        "NSW",
        num_senators,
    )
    .unwrap();
    let expected = vec![
        ("Marise PAYNE", 1583601),
        ("Sam DASTYARI", 1385000),
        ("Arthur SINODINOS", 1233372),
        ("Jenny McALLISTER", 1035654),
        ("Fiona NASH", 892890),
        ("Deborah O'NEILL", 692256),
        ("Concetta FIERRAVANTI-WELLS", 547933),
        ("Doug CAMERON", 351003),
        ("Lee RHIANNON", 347579),
        ("John WILLIAMS", 427771),
        ("Brian BURSTON", 353829),
        ("David LEYONHJELM", 275766),
    ];
    assert_eq!(expected.len(), num_senators);
    assert_eq!(result.senators.len(), num_senators);

    for ((senator, obs_count), (name, exp_count)) in result.senators.iter().zip(expected.iter()) {
        assert_eq!(
            name,
            &format!("{} {}", senator.other_names, senator.surname)
        );
        assert_eq!(obs_count, &Int::from(*exp_count));
    }
}
