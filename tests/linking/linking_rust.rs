extern crate meta_diff;
extern crate tempdir;

pub fn dynamic_linking_ok(args: Vec<f64>, out: Vec<f64>,source: &str) {
    let file_name = "test_dl";
    let func_name = "test";
    let mut location = tempdir::TempDir::new("rust_test").unwrap().into_path();
    let dl_func: meta_diff::linking::DLFunction<f64,f64> = meta_diff::linking::dynamical_linking_rust(
        source, file_name, &mut location, func_name).unwrap();

    for (i,arg) in args.iter().enumerate(){
        let diff = (dl_func.eval(*arg) - out[i]).abs();
        assert!(diff < 1e-10, format!("Incorrect result for argument {}",arg))
    }
}

parametarise_test!(dynamic_linking_ok,
    [vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0],
    vec![0.26894142136999510395,0.32082130082460702525,0.37754066879814540680,0.43782349911420193056,0.50000000000000000000,0.56217650088579806944,0.62245933120185459320,0.67917869917539297475,0.73105857863000489605],
    "   // test_dl.rs
        #[no_mangle]
        pub fn test(x: f64) -> f64 {
            1.0 / (1.0 + (-x).exp())
        } "]);
