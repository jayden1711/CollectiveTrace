use collectivetrace::parser::parse_file;
use std::path::Path;
use std::time::Instant;

fn main() {
    let path = Path::new("fixtures/sample_trace.json");
    let runs = 20;
    let mut best = f64::MAX;

    for _ in 0..runs {
        let start = Instant::now();
        parse_file(path).unwrap();
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed < best {
            best = elapsed;
        }
    }

    println!("rust parse time (best of {} runs): {:.4}s", runs, best);
}
