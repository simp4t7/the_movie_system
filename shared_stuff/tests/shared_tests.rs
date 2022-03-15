use shared_stuff::imdb_structs::ImdbQuery;

#[test]
fn check_query() {
    let test_1: &str = "is this a query?";
    let query_1: ImdbQuery = test_1.into();
    println!("{:?}", &query_1);

    let test_2: String = String::from("another query");
    let query_2: ImdbQuery = test_2.into();
    println!("{:?}", &query_2);

    let test_3: usize = 47;
    let query_3: ImdbQuery = test_3.into();
    println!("{:?}", &query_3);
}
