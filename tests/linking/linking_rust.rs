extern crate meta_diff;
extern crate tempdir;

use self::meta_diff::linking::*;

pub fn dynamic_linking_ok(args: Vec<f64>, out: Vec<f64>,source: &str) {
    let file_name = "test_dl";
    let func_name = "test";
    let mut location = tempdir::TempDir::new("rust_test").unwrap().into_path();
    let dl_func: DLFunction<f64,f64> = dynamical_linking_rust(
        source, file_name, &mut location, func_name).unwrap();

    for (i,arg) in args.iter().enumerate(){
        let res = dl_func.eval(*arg);
        if out[i].is_nan(){
            assert!(res.is_nan(),
                format!("Incorrect result for argument {}, expected {}, was {} ",arg, out[i], res))
        } else {
            let diff = (res - out[i]).abs();
            assert!(diff < 1e-15,
            format!("Incorrect result for argument {}, expected {}, was {} ",arg, out[i], res))
        }
    }
}

pub fn dynamic_linking_fail(error: &DynamicLinkingError, source: &str) {
    let file_name = "test_dl";
    let func_name = "test";
    let mut location = tempdir::TempDir::new("rust_test").unwrap().into_path();
    let res : Result<DLFunction<f64,f64>, DynamicLinkingError> = dynamical_linking_rust(
        source, file_name, &mut location, func_name);
    match res {
        Ok(_) => assert!(false, "Dynamic linking should have failed, but didn't"),
        Err(err) => {
            match err {
                DynamicLinkingError::Io(_) => match *error {
                    DynamicLinkingError::Io(_) => assert!(true),
                    _=> assert!(false, "Incorrect error, expected: {}, was {}|", *error, err)
                },
                DynamicLinkingError::Compilation(ref s1) => match *error {
                    DynamicLinkingError::Compilation(ref s2) => assert!(s1.contains(s2), "Incorrect error, expected: {}, was {}|", error, err),
                    _=> assert!(false, "Incorrect error, expected: {}, was {}|", *error, err)
                },
                DynamicLinkingError::OpenLibrary(ref s1) => match *error {
                    DynamicLinkingError::OpenLibrary(ref s2) => assert!(s1.contains(s2), "Incorrect error, expected: {}, was {}|", error, err),
                    _=> assert!(false, "Incorrect error, expected: {}, was {}|", *error, err)
                },
                DynamicLinkingError::SymbolTable(ref s1) => match *error {
                    DynamicLinkingError::SymbolTable(ref s2) => assert!(s1.contains(s2), "Incorrect error, expected: {}, was {}|", error, err),
                    _=> assert!(false, "Incorrect error, expected: {}, was {}|", *error, err)
                },
            }

        }
    }
}

parametarise_test!(dynamic_linking_ok,{
    vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0],
    vec![0.26894142136999510395, 0.32082130082460702525, 0.37754066879814540680, 0.43782349911420193056, 0.50000000000000000000, 0.56217650088579806944, 0.62245933120185459320, 0.67917869917539297475, 0.73105857863000489605],
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    } "
},{
    vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0],
    vec![-1.10908117742402323458, -0.86005775604188627881, -0.58780478976890193632, -0.29844720425183085544, 0.00000000000000000000, 0.29844720425183085544, 0.58780478976890193632, 0.86005775604188627881, 1.10908117742402323458],
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        2.4 * (x/2.0).tanh()
    } "
},{
    vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0],
    vec![4.73515900796232269698, 4.87731706945529364106, 5.04531083492266496648, 5.24142471325712833163, 5.46731322773325167930, 5.72381678975859031766, 6.01085793007176949487, 6.32744388086401787774, 6.67177643695004984892],
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        let y = (x+2.0).exp() + 8.0;
        (y*y + x).ln()
    } "
},{
    vec![-1.0,-0.75,-0.5,-0.25,0.0,0.25,0.5,0.75,1.0],
    vec![::std::f64::NAN, 0.15674646902621844347, 0.55585313969623040276, 0.81252007721352959013, 1.00000000000000000000, 1.12063988508324130500, 1.17255289492536807217, 1.15542379613608758859, 1.07267867039314279687],
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        let y = (x+2.0).sin() + 1.0;
        let z = y.abs().ln();
        (x*y*z + 1.0).sqrt()
    } "
},{
    vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0],
    vec![9.0, 25.0, 64.0, 169.0, 441.0, 1156.0, 3025.0, 7921.0, 20736.0, 54289.0],
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        let mut x_0 = 1.0;
        let mut x_1 = 1.0;
        let n = x.round() as i64;
        for _ in 0..n {
            let x_n = x_0 + x_1;
            x_0 = x_1;
            x_1 = x_n;
        }
        x_1*x_1
    } "
});

parametarise_test!(dynamic_linking_fail,{
    &meta_diff::linking::DynamicLinkingError::Compilation("`core::ops::Div<f64>` is not implemented for the type `_`".to_string()),
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        1 / (1.0 + (-x).exp())
    } "
},{
    &meta_diff::linking::DynamicLinkingError::Compilation("re-assignment of immutable variable `a`".to_string()),
    "// test_dl.rs
    #[no_mangle]
    pub fn test(x: f64) -> f64 {
        let a = 1.0 / (1.0 + (-x).exp());
        a = x + a*a;
        a
    } "
},{
    &meta_diff::linking::DynamicLinkingError::Compilation("re-assignment of immutable variable `a`".to_string()),
    "// test_dl.rs
    #[no_mangle]
    pub fn test12(x: f64) -> f64 {
        let a = 1.0 / (1.0 + (-x).exp());
        a = x + a*a;
        a
    } "
},{
    &meta_diff::linking::DynamicLinkingError::SymbolTable("undefined symbol: test".to_string()),
    "// test_dl.rs
    #[no_mangle]
    pub fn test12(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    } "
},{
    &meta_diff::linking::DynamicLinkingError::SymbolTable("undefined symbol: test".to_string()),
    "// test_dl.rs
    #[no_mangle]
    pub fn test23(x: f64) -> f64 {
        let mut  a = 1.0 / (1.0 + (-x).exp());
        a = x + a*a;
        a
    } "
});
