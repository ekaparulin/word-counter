use std::collections::BTreeMap;

pub struct Histogram {
    data: BTreeMap<usize, usize>,
    bin_size: usize,
    include_zeroes: bool
}

impl Histogram {

    pub fn new(bin_size: usize, include_zeroes: bool) -> Histogram {
        Histogram{
            data: BTreeMap::new(),
            bin_size,
            include_zeroes
        }
    }

    pub fn add_word_count(&mut self, count: usize) {

        // Calculate current bin index
        let bin_index: usize = count / self.bin_size;

        // Increase counter for the current bin
        {
            // get mutable ref to counter or init it w/ 0 if doesn't exist
            let frequency = self.data.entry(bin_index).or_insert(0);

            // increase counter
            *frequency += 1;

            println!("bin={} {} {}", bin_index, count, frequency);
        }

        // Fill gaps for unset bins until current bin
        if self.include_zeroes {
            // fill gaps w/ zeroes
            for k in 0..(bin_index) {
                self.data.entry(k).or_insert(0);
            }
        }
    }

    pub fn acsii_histogram(&self) {

        println!("{0}{0:->11}{0:->11}","+");
        println!("|{:>10}|{:>10}|", "Word count", "Frequency");
        println!("{0}{0:->11}{0:->11}","+");
        for (count, freq) in &self.data {

            // Set bin range for label
            let mut count_label = format!("{}-{}",
                                          count * self.bin_size,
                                          (count+1) * self.bin_size);

            // Override label to bin size if bin size is 1
            if self.bin_size == 1 {
                count_label = format!("{}",count);
            }

            // Print data
            println!("|{count:>10}|{freq:10}|", count=count_label, freq=freq);
        }
        println!("{0}{0:->11}{0:->11}","+");

    }
}