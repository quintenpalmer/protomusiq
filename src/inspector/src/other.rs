use claxon::FlacReader;

fn main() {
    println!("Hello, world!");

    let mut reader =
        FlacReader::open("/home/quinten/test.flac").expect("failed to open FLAC stream");

    // TODO: Write fallback for other sample widths and channel numbers.
    assert!(reader.streaminfo().bits_per_sample == 16);
    assert!(reader.streaminfo().channels == 2);

    for (tag_key, tag_value) in reader.tags() {
        println!("{}:\t{}", tag_key, tag_value);
    }

    let mut sqr_sum = 0.0;
    let mut count = 0;
    for sample in reader.samples() {
        let s = sample.unwrap() as f64;
        sqr_sum += s * s;
        count += 1;
    }
    println!("RMS is {}", (sqr_sum / count as f64).sqrt());
}
