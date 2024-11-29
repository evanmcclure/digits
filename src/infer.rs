// infer.rs

// Infer a new handwritten digit
fn infer(model_input_filename: &str, png_input_filename: &str, processor: Processor) -> Result<(), Error> {
    // Load the model

    // Create the NN from a pre-existing model
    let model = Model{};
    let nn = NeuralNetwork::fromModel(model);

    // Read an image and infer the digit
    let outputs = nn.query();

    // println!("label is {}", label);

    Ok(())
}
