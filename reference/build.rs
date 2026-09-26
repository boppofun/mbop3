fn main() {
    println!("cargo:rerun-if-changed=src/shim.c");
    println!("cargo:rerun-if-changed=minimp3/minimp3.h");
    for (variant, float) in [("i16", false), ("f32", true)] {
        let mut build = cc::Build::new();
        build
            .file("src/shim.c")
            .include("minimp3")
            .define("REF_VARIANT", variant)
            // Bit-exact comparison with Rust requires plain IEEE single-precision math.
            .flag_if_supported("-ffp-contract=off")
            .flag_if_supported("-fno-fast-math")
            // minimp3 reads its uninitialized stack scratch buffer on some corrupt
            // streams (found by MSan in L3_huffman). Zeroing locals makes the
            // reference deterministic and matches the zero-initialized Rust translation.
            .flag("-ftrivial-auto-var-init=zero")
            .opt_level(2);
        if float {
            build.define("MINIMP3_FLOAT_OUTPUT", None);
        }
        build.compile(&format!("minimp3_ref_{variant}"));
    }
}
