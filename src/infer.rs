// infer.rs

// Infer a new handwritten digit
fn infer(config: Box<dyn Config>) -> Result<(), Error> {
    let config = match config.as_any().downcast_ref::<InferConfig>() {
        Some(config) => config,
        None => panic!("&config isn't a InferConfig!"),
    };

    println!("Inter config is {:#?}", config);

    // Load the model

    // Create the NN

    // Read an image and infer the digit

    Ok(())
}
