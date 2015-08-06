use std::process::Command;
use std::io::Write;
use std::dynamic_lib::DynamicLibrary;
use std::result::Result;

/// An encapsulater object of a function loaded from a dynamic library.
/// The object is required because the if the `DynamicLibrary` object is dropped the function pointer will no longer be valid
pub struct DLFunction<P,R>{
    pub lib: DynamicLibrary,
    pub func: fn(P) -> R
}

impl<P,R> DLFunction<P,R> {
    /// Evaluates the function with the given parameters
    // #[inline(always)]
    pub fn eval(&self, args: P) -> R {
        (self.func)(args)
    }
}

/// The function performs several actions in sequence:
///
/// * Creates a file `location/file_name.rs`
/// * Writes the source code to the file
/// * Compiles the source code using rustc to a dynamic library `location/file_name.so`
/// * Opens the compiled dynamic library
/// * Finds and links from the symbol table `func_name`
/// * Returns a `DLFunction` object
pub fn dynamical_linking_rust<P,R>(source: &str, file_name: &str, location: &mut ::std::path::PathBuf, func_name: &str)
    -> Result<DLFunction<P,R>,DynamicLinkingError>{
    // Write source code to file
    let mut source_loc = location.clone();
    source_loc.push(&file_name);
    source_loc.set_extension("rs");
    let file = try!(::std::fs::File::create(source_loc.as_path()));
    {
        let mut writer = ::std::io::BufWriter::new(&file);
        let _ = try!(writer.write_fmt(format_args!("{}\n",source)));
    }
    // Compile the source code
    let mut target_loc = location.clone();
    target_loc.push(&file_name);
    target_loc.set_extension("so");
    let cmd = format!("rustc --crate-type dylib {} -o {}",
        source_loc.into_os_string().to_str().unwrap(),
        target_loc.clone().into_os_string().to_str().unwrap());
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    // Check if compilation is ok
    if !output.status.success() {
        return Err(DynamicLinkingError::Compilation(String::from_utf8(output.stderr).unwrap()))
    }

    // Link the library
    let lib = match DynamicLibrary::open(Some(target_loc.as_path())){
        Ok(lib) => lib,
        Err(err) => return Err(DynamicLinkingError::OpenLibrary(err))
    };
    let func: fn(P) -> R = unsafe {
        match lib.symbol(func_name) {
            Ok(func) => ::std::mem::transmute::<*mut u8,fn(P) -> R>(func),
            Err(err) => return Err(DynamicLinkingError::SymbolTable(err))
        }
    };
    Ok(DLFunction{lib: lib, func: func})
}

#[derive(Debug)]
pub enum DynamicLinkingError{
    Io(::std::io::Error),
    Compilation(String),
    OpenLibrary(String),
    SymbolTable(String)
}

impl ::std::convert::From<::std::io::Error> for DynamicLinkingError {
    fn from(err: ::std::io::Error) -> DynamicLinkingError {
        DynamicLinkingError::Io(err)
    }
}

impl ::std::fmt::Display for DynamicLinkingError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
			DynamicLinkingError::Io(ref err) => write!(f, "{}",  err),
            DynamicLinkingError::Compilation(ref err) => write!(f, "{}",  err),
            DynamicLinkingError::OpenLibrary(ref err) => write!(f, "{}",  err),
            DynamicLinkingError::SymbolTable(ref err) => write!(f, "{}",  err),
        }
    }
}

impl ::std::error::Error for DynamicLinkingError {
    fn description(&self) -> &str {
        match *self {
            DynamicLinkingError::Io(ref err) => err.description(),
            DynamicLinkingError::Compilation(_) => "Compilation of the source code failed",
            DynamicLinkingError::OpenLibrary(_) => "Opening the compiled library failed",
            DynamicLinkingError::SymbolTable(_) => "Finding the function in the symbol table failed",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            DynamicLinkingError::Io(ref err) => Some(err),
			_ => None
        }
    }
}
