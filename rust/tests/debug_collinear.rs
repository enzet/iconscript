use iconscript_rust::generator::{
    deduplicate_path_elements, simplify_path_collinear,
};
use kurbo::BezPath;

#[test]
fn debug_simple_collinear() {
    let path_str = "M 1,1 L 2,2 3,3";
    println!("\nInput: {}", path_str);

    let parsed = BezPath::from_svg(path_str).unwrap();
    println!("Parsed elements:");
    for el in parsed.iter() {
        println!("  {:?}", el);
    }

    let deduplicated = deduplicate_path_elements(&parsed);
    println!("\nAfter deduplication:");
    for el in deduplicated.iter() {
        println!("  {:?}", el);
    }

    let simplified = simplify_path_collinear(&deduplicated);
    println!("\nAfter collinear simplification:");
    for el in simplified.iter() {
        println!("  {:?}", el);
    }

    let result = simplified.to_svg();
    println!("\nFinal SVG: {}", result);
    println!("Expected:  M 1,1 L 3,3");

    // TODO(enzet): fix the assertion.
    // assert_eq!(result.trim(), "M1,1 L3,3");
}
