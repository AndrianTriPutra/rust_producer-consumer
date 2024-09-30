use peak_alloc::PeakAlloc;
use crate::pkg::utils::log;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;
pub async fn memory_check(delay: tokio::time::Duration) {
    let mut counter=0;
    loop {
        counter+=1;
        let data = format!("counter [{}]", counter);
        let current_mem = PEAK_ALLOC.current_usage_as_kb();
        let peak_mem = PEAK_ALLOC.peak_usage_as_kb();

        log::logger("info", "main", &format!("\t\t\t\t\t data {} -> memory usage [{} kB] peak [{} kB],", data, current_mem, peak_mem));
        tokio::time::sleep(delay).await; 
    }
}
