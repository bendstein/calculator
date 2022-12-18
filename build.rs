fn main() {
    let slint_files: Vec<&str> = vec![
        "helloworld"
    ];

    let mut compile_errs: Vec<String> = Vec::new();

    for file in slint_files {
        let slint_compile_result = slint_build::compile(format!("src\\calculator_interface\\gui\\components\\{file}.slint"));

        if let Err(compile_err) = slint_compile_result {
            compile_errs.push(compile_err.to_string());
        }
    }

    if !compile_errs.is_empty() {
        let concatenated = compile_errs.join("\r\n");
        panic!("Failed to build slint files: {concatenated}");
    }
}