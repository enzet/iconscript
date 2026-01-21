use kurbo::BezPath;
use std::fs;
use std::path::Path;

// Import the functions we want to test.
use iconscript::generator::{
    deduplicate_path_elements, round_path_coordinates, simplify_path_collinear,
};

#[test]
fn test_path_simplification() {
    let test_dir = Path::new("test");

    // Read all .txt test files.
    let test_files = fs::read_dir(test_dir)
        .expect("Failed to read test directory.")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "txt" {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert!(
        !test_files.is_empty(),
        "No test files found in test/ directory."
    );

    for test_file in test_files {
        println!("\nTesting: {}", test_file.display());

        let content = fs::read_to_string(&test_file).expect(&format!(
            "Failed to read test file: {}",
            test_file.display()
        ));

        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < 2 {
            panic!(
                concat!(
                    "Test file {} must have at least 2 lines (input and expected ",
                    "output)."
                ),
                test_file.display()
            );
        }

        let input_path = lines[0].trim();
        let expected_output = lines[1].trim();

        // Skip empty lines.
        if input_path.is_empty() || expected_output.is_empty() {
            continue;
        }

        println!("  Input:    {}", input_path);
        println!("  Expected: {}", expected_output);

        // Parse the input path
        let parsed = BezPath::from_svg(input_path)
            .expect(&format!("Failed to parse input path: {}.", input_path));

        // Apply optimizations (same order as in combine_paths).
        let rounded = round_path_coordinates(&parsed);
        let deduplicated = deduplicate_path_elements(&rounded);
        let simplified = simplify_path_collinear(&deduplicated);

        let actual_output = simplified.to_svg();

        println!("  Actual:   {}", actual_output);

        // Compare outputs.
        assert_eq!(
            normalize_path(&actual_output),
            normalize_path(expected_output),
            "Path simplification failed for test file: {}.",
            test_file.display()
        );

        println!("  PASS");
    }
}

/// Normalize a path string for comparison by
///   - removing extra whitespace,
///   - normalizing number precision.
fn normalize_path(path: &str) -> String {
    path.trim().split_whitespace().collect::<Vec<_>>().join(" ")
}
