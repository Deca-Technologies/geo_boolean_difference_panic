use geo::BooleanOps;
use geo_types::MultiPolygon;
use rstest::rstest;

#[rstest]
#[case::difference(
    "difference",
    "./test_data/boolean_difference_polygons.json",
    "./test_data/boolean_difference_polygons.json"
)]
fn difference_operation_should_panic(
    #[case] case_name: &str,
    #[case] input_left: &str,
    #[case] input_right: &str,
) {
    let left_polygons: MultiPolygon = serde_any::from_file(input_left).unwrap();
    let right_polygons: MultiPolygon = serde_any::from_file(input_right).unwrap();

    for i in 0..left_polygons.0.len() {
        let left_iteration = MultiPolygon(
            left_polygons
                .clone()
                .into_iter()
                .take(i + 1)
                .collect::<Vec<geo_types::Polygon>>(),
        );

        let right_iteration = MultiPolygon(
            right_polygons
                .clone()
                .into_iter()
                .take(i + 1)
                .collect::<Vec<geo_types::Polygon>>(),
        );

        left_iteration.difference(&right_iteration);

        println!(
            "Case: `{}`. The test panicked after iteration: `{}`",
            case_name, i
        );
    }
}
