// train.rs

// Train the NN
fn train(config: Box<dyn Config>) -> Result<(), Error> {
    let config = match config.as_any().downcast_ref::<TrainConfig>() {
        Some(config) => config,
        None => panic!("&config isn't a TrainConfig!"),
    };

    println!("Train config is {:#?}", config);

    // Create the NN

    // Load the training data CSV

    // Save the model

    Ok(())
}
