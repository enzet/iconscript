fn main() {
    // Before running this file, run `./generate_parser.sh`.

    let grammar_file = "../grammar/IconScript.g4";
    println!("cargo:rerun-if-changed={}", grammar_file);
}
