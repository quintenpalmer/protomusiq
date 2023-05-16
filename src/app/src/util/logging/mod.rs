use std::time;

pub enum LogType {
    Timing,
}

impl LogType {
    fn get_name(&self) -> String {
        match self {
            LogType::Timing => "TIMING",
        }
        .to_string()
    }
}

pub struct Logger {
    type_: LogType,
    additional_pre_text: String,
    last_log: time::Instant,
}

impl Logger {
    pub fn new(type_: LogType, pre_text: &'static str) -> Self {
        Logger {
            type_: type_,
            additional_pre_text: pre_text.to_string(),
            last_log: time::Instant::now(),
        }
    }

    pub fn print_elapsed<S: ToString>(&mut self, function_name: S) {
        let top_level_name = self.type_.get_name();

        let duration = self.last_log.elapsed();

        println!(
            "{}\t{}\t{}\ttook\t{}.{:03}",
            top_level_name,
            self.additional_pre_text,
            function_name.to_string(),
            duration.as_secs(),
            duration.subsec_millis()
        );
        self.last_log = time::Instant::now();
    }
}
