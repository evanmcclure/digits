// train.rs

use std::fs::File;

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
    let file = File::open(mnist_training_data_csv_filename)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut data_list = Vec::new();
    for result in reader.deserialize() {
        let record: Vec<u8> = result?;
        data_list.push(record);
    }

    println!("{:?}", data_list[0]);
    
    // Train
    for e in 0..num_training_epochs {
        nn.train();
    }

    // Save the model

    Ok(())
}
