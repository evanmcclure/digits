// test.rs

// Test the NN
fn test(mnist_test_data_csv_filename: &str, model_input_filename: &str, processor: Processor) -> Result<(), Error> {
    // Load the model

    // Create the NN from a pre-existing model
    let model = Model{};
    let nn = NeuralNetwork::from_model(model);

    // Load the test data CSV
    let file = File::open(mnist_test_data_csv_filename)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut data_list = Vec::new();
    for result in reader.deserialize() {
        let record: Vec<u8> = result?;
        data_list.push(record);
    }

    println!("{:?}", data_list[0]);

    // Test

    // Calculate the performance score

    Ok(())
}
