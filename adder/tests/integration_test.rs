use adder;

/*
  In order to execute only the integration tests, we can run
  this command: `cargo test --test integration_test`.
 */
#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}