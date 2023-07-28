use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct MT19937 {
    mt: [u32; 624],
    index: u32,
    lower_mask: u32,
    upper_mask: u32,
    word_size: u32,
    degree_of_recurrence: u32,
    middle_word: u32,
    separation_p: u32,
    a: u32,
    b_bmask: u32,
    c_bmask: u32,
    s_bshift: u32,
    t_bshift: u32,
    u: u32,
    d: u32,
    l: u32,
    f: u32,
}

impl Default for MT19937 {
    fn default() -> Self {
        let word_size: u32 = 32;
        let separation_p = 31;
        let degree_of_recurrence = 624;

        let mt: [u32; 624] = [0; 624];
        let index = degree_of_recurrence + 1;
        let lower_mask = (1 << separation_p) - 1;
        let upper_mask = !lower_mask;

        Self {
            mt,
            index,
            lower_mask,
            upper_mask,
            word_size,
            degree_of_recurrence,
            middle_word: 397,
            separation_p,
            a: 0x9908B0DF,
            b_bmask: 0x9D2C5680,
            c_bmask: 0xEFC60000,
            s_bshift: 7,
            t_bshift: 15,
            u: 11,
            d: 0xFFFFFFFF,
            l: 18,
            f: 1812433253,
        }
    }
}

impl MT19937 {
    fn new(seed: u32) -> Self {
        let mut mersenne_twitster = Self::default();
        mersenne_twitster.index = mersenne_twitster.degree_of_recurrence;

        mersenne_twitster.mt[0] = seed;

        for i in 1..mersenne_twitster.degree_of_recurrence as usize - 1 {
            let lhs = mersenne_twitster.f;
            let rhs = 
                (mersenne_twitster.mt[i - 1]
                    ^ (mersenne_twitster.mt[i - 1] >> (mersenne_twitster.word_size - 2)))
            ;
            let (value, did_overflow) = lhs.overflowing_mul(rhs);
            mersenne_twitster.mt[i] = value + i as u32;
        }
        mersenne_twitster
    }
    pub fn extract_number(&mut self) -> Option<u32> {
        if self.index >= self.degree_of_recurrence {
            if self.index > self.degree_of_recurrence {
                println!("ERRROR");
                return None;
            }
            self.twist();
        }
        let mut y = self.mt[self.index as usize];
        y = y ^ ((y >> self.u) & self.d);
        y = y ^ ((y << self.s_bshift) & self.b_bmask);
        y = y ^ ((y << self.t_bshift) & self.c_bmask);
        y = y ^ (y >> self.l);

        self.index += 1;
        Some(y)
    }
    pub fn twist(&mut self) {
        for i in 0..self.degree_of_recurrence - 1 {
            let x = (self.mt[i as usize] & self.upper_mask)
                | (self.mt[(i as usize + 1) % self.degree_of_recurrence as usize]
                    & self.lower_mask);
            let mut xA = x >> 1;

            if (x % 2) != 0 {
                // lowest bit of x is 1
                xA = xA ^ self.a;
            }
            self.mt[i as usize] = self.mt
                [(i as usize + self.middle_word as usize) % self.degree_of_recurrence as usize]
                ^ xA;
        }
        self.index = 0;
    }
}

// running 1 test
// test rng::testttt has been running for over 60 seconds
// seed is 1690563913
// number: 1410803203

pub fn crack_mt19937() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random duration between 40 and 100 seconds
    let random_duration = rng.gen_range(40..=1000);

    // Sleep the thread for the random duration
    thread::sleep(Duration::from_secs(random_duration));
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Convert the Duration to a numeric representation of the Unix timestamp (seconds since the Epoch)
    let timestamp = current_time.as_secs();
    let seed = timestamp as u32;

    println!("seed is {}", seed);
    let mut mt = MT19937::new(seed);
    let r = mt.extract_number().unwrap();
    for i in 0..10 {
        
        println!("number: {}", r)
    };
}


#[test]
fn testttt() {
    crack_mt19937()
}
