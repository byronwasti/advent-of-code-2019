fn main() {
    let range_min = 264793u32;
    let range_max = 803935u32;

    let count = (range_min..range_max)
        .map(|x| x.to_string())
        .filter(|x| {
            let mut prev = '/';
            let mut repetition = 0;
            let mut double = false;
            for i in x.chars() {
                if prev > i {
                    return false;
                }

                if prev == i {
                    repetition += 1;
                } else {
                    if repetition == 1 {
                        double = true
                    }
                    repetition = 0;
                }
                prev = i;
            }
            if repetition == 1 {
                double = true;
            }
            double
        })
        .count();

    println!("{}", count);
}
