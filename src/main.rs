use geo::BooleanOps;
use geo_types::{polygon, Coord, MultiPolygon, Polygon};

fn main() {
    let clip = create_clip_multipolygon();
    let subjects = load_subject_multipolygon();

    // Does not fail on every single call, so run continuously until failure is hit
    loop {
        let failing_subjects = MultiPolygon(
            subjects.clone().into_iter().collect::<Vec<Polygon>>(),
        );

        BooleanOps::difference(&failing_subjects, &clip);
    }
}

fn load_subject_multipolygon() -> MultiPolygon {
    serde_any::from_file(std::path::Path::new(
        "./test_data/boolean_difference_subject_multipolygon.json",
    ))
        .unwrap()
}

fn create_clip_multipolygon() -> MultiPolygon {
    let polygon = polygon!(
        Coord {
            x: 3666300.0,
            y: -1181300.0,
        },
        Coord {
            x: 4414100.0,
            y: -1181300.0,
        },
        Coord {
            x: 4436000.0,
            y: -1159400.0,
        },
        Coord {
            x: 4436000.0,
            y: 4401900.0,
        },
        Coord {
            x: 4407000.0,
            y: 4430900.0,
        },
        Coord {
            x: -4420900.0,
            y: 4430900.0,
        },
        Coord {
            x: -4420900.0,
            y: -1136200.0,
        },
        Coord {
            x: -4406000.0,
            y: -1151100.0,
        },
        Coord {
            x: -3643800.0,
            y: -1151100.0,
        },
        Coord {
            x: -3640000.0,
            y: -1147300.0,
        },
        Coord {
            x: -3640000.0,
            y: -319400.0,
        },
        Coord {
            x: -3618900.0,
            y: -298300.0,
        },
        Coord {
            x: -2584900.0,
            y: -298300.0,
        },
        Coord {
            x: -2524600.0,
            y: -358600.0,
        },
        Coord {
            x: 1775200.0,
            y: -358600.0,
        },
        Coord {
            x: 1893600.0,
            y: -477000.0,
        },
        Coord {
            x: 2177300.0,
            y: -477000.0,
        },
        Coord {
            x: 2239800.0,
            y: -414500.0,
        },
        Coord {
            x: 3618500.0,
            y: -414500.0,
        },
        Coord {
            x: 3643200.0,
            y: -439200.0,
        },
        Coord {
            x: 3643200.0,
            y: -1158200.0,
        },
        Coord {
            x: 3666300.0,
            y: -1181300.0,
        },
    );
    MultiPolygon(vec![polygon])
}
