use busywork::{busywork, busywork_with, BusyWork, Categories, Intensity};
use std::time::Instant;

fn measure<F: FnOnce()>(f: F) -> f64 {
    let start = Instant::now();
    f();
    start.elapsed().as_secs_f64() * 1000.0
}

fn avg_ms<F: Fn()>(runs: usize, f: F) -> (f64, f64, f64) {
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        times.push(measure(|| f()));
    }
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = times[0];
    let max = times[times.len() - 1];
    let avg = times.iter().sum::<f64>() / times.len() as f64;
    (min, avg, max)
}

#[test]
fn bench_intensities() {
    let runs = 5;
    println!("\n{:=<70}", "");
    println!("  INTENSITY BENCHMARKS ({} runs each, all categories)", runs);
    println!("{:=<70}", "");
    println!(
        "  {:10} {:>12} {:>12} {:>12}",
        "Level", "Min (ms)", "Avg (ms)", "Max (ms)"
    );
    println!("{:-<70}", "");

    for (name, intensity) in [
        ("Low", Intensity::Low),
        ("Medium", Intensity::Medium),
        ("High", Intensity::High),
        ("Ultra", Intensity::Ultra),
    ] {
        let (min, avg, max) = avg_ms(runs, || busywork(intensity));
        println!(
            "  {:10} {:>12.2} {:>12.2} {:>12.2}",
            name, min, avg, max
        );
    }
    println!("{:=<70}\n", "");
}

#[test]
fn bench_categories_medium() {
    let runs = 5;
    println!("\n{:=<70}", "");
    println!(
        "  CATEGORY BENCHMARKS @ Medium ({} runs each, single category)",
        runs
    );
    println!("{:=<70}", "");
    println!(
        "  {:14} {:>12} {:>12} {:>12}",
        "Category", "Min (ms)", "Avg (ms)", "Max (ms)"
    );
    println!("{:-<70}", "");

    let cats: &[(&str, Categories)] = &[
        ("COMPUTE", Categories::COMPUTE),
        ("MEMORY", Categories::MEMORY),
        ("FILESYSTEM", Categories::FILESYSTEM),
        ("REGISTRY", Categories::REGISTRY),
        ("WINAPI", Categories::WINAPI),
        ("NETWORK", Categories::NETWORK),
        ("CRYPTO", Categories::CRYPTO),
    ];

    for &(name, cat) in cats {
        let (min, avg, max) = avg_ms(runs, || busywork_with(Intensity::Medium, cat));
        println!(
            "  {:14} {:>12.2} {:>12.2} {:>12.2}",
            name, min, avg, max
        );
    }
    println!("{:=<70}\n", "");
}

#[test]
fn bench_categories_high() {
    let runs = 3;
    println!("\n{:=<70}", "");
    println!(
        "  CATEGORY BENCHMARKS @ High ({} runs each, single category)",
        runs
    );
    println!("{:=<70}", "");
    println!(
        "  {:14} {:>12} {:>12} {:>12}",
        "Category", "Min (ms)", "Avg (ms)", "Max (ms)"
    );
    println!("{:-<70}", "");

    let cats: &[(&str, Categories)] = &[
        ("COMPUTE", Categories::COMPUTE),
        ("MEMORY", Categories::MEMORY),
        ("FILESYSTEM", Categories::FILESYSTEM),
        ("REGISTRY", Categories::REGISTRY),
        ("WINAPI", Categories::WINAPI),
        ("NETWORK", Categories::NETWORK),
        ("CRYPTO", Categories::CRYPTO),
    ];

    for &(name, cat) in cats {
        let (min, avg, max) = avg_ms(runs, || busywork_with(Intensity::High, cat));
        println!(
            "  {:14} {:>12.2} {:>12.2} {:>12.2}",
            name, min, avg, max
        );
    }
    println!("{:=<70}\n", "");
}

#[test]
fn bench_jitter_variance() {
    let runs = 10;
    println!("\n{:=<70}", "");
    println!("  JITTER VARIANCE (Medium, COMPUTE only, {} runs)", runs);
    println!("{:=<70}", "");

    let mut with_jitter = Vec::new();
    let mut without_jitter = Vec::new();

    for _ in 0..runs {
        with_jitter.push(measure(|| {
            BusyWork::new(Intensity::Medium)
                .allow(Categories::COMPUTE)
                .jitter(true)
                .run();
        }));
        without_jitter.push(measure(|| {
            BusyWork::new(Intensity::Medium)
                .allow(Categories::COMPUTE)
                .jitter(false)
                .run();
        }));
    }

    let variance = |times: &[f64]| -> f64 {
        let avg = times.iter().sum::<f64>() / times.len() as f64;
        let var = times.iter().map(|t| (t - avg).powi(2)).sum::<f64>() / times.len() as f64;
        var.sqrt()
    };

    let jitter_avg = with_jitter.iter().sum::<f64>() / runs as f64;
    let no_jitter_avg = without_jitter.iter().sum::<f64>() / runs as f64;
    let jitter_std = variance(&with_jitter);
    let no_jitter_std = variance(&without_jitter);

    println!(
        "  {:20} {:>10} {:>10}",
        "", "Avg (ms)", "StdDev"
    );
    println!("{:-<70}", "");
    println!(
        "  {:20} {:>10.2} {:>10.2}",
        "Jitter ON", jitter_avg, jitter_std
    );
    println!(
        "  {:20} {:>10.2} {:>10.2}",
        "Jitter OFF", no_jitter_avg, no_jitter_std
    );
    println!("{:=<70}\n", "");
}
