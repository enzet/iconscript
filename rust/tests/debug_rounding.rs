use iconscript_rust::generator::round_path_coordinates;
use kurbo::BezPath;

#[test]
fn test_rounding() {
    let path_str = "M5.00000001,4.999993 L10.5687736,8";
    println!("\nInput: {}", path_str);

    let parsed = BezPath::from_svg(path_str).unwrap();
    println!("Parsed:");
    for el in parsed.iter() {
        println!("  {:?}", el);
    }

    let rounded = round_path_coordinates(&parsed);
    println!("\nRounded:");
    for el in rounded.iter() {
        println!("  {:?}", el);
    }

    let result = rounded.to_svg();
    println!("\nFinal SVG: {}", result);
    println!("Expected:  M5,5 L10.5688,8");
}
