// config.rs

enum ConfigKind {
    TRAIN,
    TEST,
    INFER,
}

trait Config {
    fn kind(&self) -> ConfigKind;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct TrainConfig {
    input_nodes: u16,
    hidden_nodes: u16,
    output_nodes: u16,
    learning_rate: f32,
    mnist_training_data_csv_filename: String,
    num_training_epochs: u16,
    model_output_filename: String,
    processor: Processor,
}

impl Config for TrainConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::TRAIN
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TrainConfig {
    pub fn new(
        input_nodes: u16,
        hidden_nodes: u16,
        output_nodes: u16,
        learning_rate: f32,
        mnist_training_data_csv_filename: &str,
        num_training_epochs: u16,
        model_output_filename: &str,
        processor: Processor,
    ) -> Self {
        TrainConfig {
            input_nodes: input_nodes,
            hidden_nodes: hidden_nodes,
            output_nodes: output_nodes,
            learning_rate: learning_rate,
            mnist_training_data_csv_filename: String::from(mnist_training_data_csv_filename),
            num_training_epochs: num_training_epochs,
            model_output_filename: String::from(model_output_filename),
            processor: processor,
        }
    }
}

#[derive(Debug)]
struct TestConfig {
    mnist_test_data_csv_filename: String,
    model_input_filename: String,
    processor: Processor,
}

impl Config for TestConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::TEST
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TestConfig {
    pub fn new(mnist_test_data_csv_filename: &str, model_input_filename: &str, processor: Processor) -> Self {
        TestConfig {
            mnist_test_data_csv_filename: String::from(mnist_test_data_csv_filename),
            model_input_filename: String::from(model_input_filename),
            processor: processor,
        }
    }
}

#[derive(Debug)]
struct InferConfig {
    model_input_filename: String,
    png_input_filename: String,
    processor: Processor,
}

impl Config for InferConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::INFER
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl InferConfig {
    pub fn new(model_input_filename: &str, png_input_filename: &str, processor: Processor) -> Self {
        InferConfig {
            model_input_filename: String::from(model_input_filename),
            png_input_filename: String::from(png_input_filename),
            processor: processor
        }
    }
}
