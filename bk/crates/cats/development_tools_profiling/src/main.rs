mod memory_usage_analysis {
    pub mod dhat;
    pub mod measure_time;
}

fn main() {
    memory_usage_analysis::dhat::run();
    memory_usage_analysis::measure_time::run();
}
