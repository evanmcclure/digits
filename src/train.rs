// train.rs

// Train the NN
fn train(input_nodes: u16,
    hidden_nodes: u16,
    output_nodes: u16,
    learning_rate: f32,
    mnist_training_data_csv_filename: &str,
    num_training_epochs: u16,
    model_output_filename: &str,
    processor: Processor) -> Result<(), Error> {

    // Create the NN
    let nn = NeuralNetwork::new(input_nodes, hidden_nodes, output_nodes, learning_rate);

    // Load the training data CSV

    // Train
    for e in 0..num_training_epochs {
        nn.train();
    }

    // Save the model

    Ok(())
}
