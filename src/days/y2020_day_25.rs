use crate::days::internal_common::*;

struct LinesWithCount<'a> {
    lines: std::str::Lines<'a>,
    count: usize
}

impl<'a> LinesWithCount<'a> {
    fn new(lines: std::str::Lines<'a>) -> Self {
        Self {
            lines,
            count: 1
        }
    }

    fn get_an_i32(&mut self) -> Result<i32> {
        let content = self.lines.next().ok_or_else(|| Error::new_parsing_no_content(self.count))?;
        content.parse::<i32>().map_err(|_| Error::new_parsing(content, self.count))
                              .and_then(|ret| {self.count += 1; Ok(ret)})
    }
}

struct KeyComputation {
    target_pubkey: u64,
    pubkey: u64,
    privkey: u64
}

impl KeyComputation {
    const BASE: u64 = 7;
    const MOD: u64 = 20201227;
    const MAX_TRIES: u64 = 20201227;

    fn new(target_pubkey: u64) -> Self {
        Self {
            target_pubkey,
            pubkey: 1,
            privkey: 0
        }
    }

    fn calc_next(&mut self) {
        let pubkey_next = (self.pubkey * Self::BASE) % Self::MOD;
        let privkey_next = self.privkey + 1;
        self.pubkey = pubkey_next;
        self.privkey = privkey_next;
    }

    fn max_tries_exceeded(&self) -> bool {
        self.privkey > Self::MAX_TRIES
    }

    fn target_pubkey_obtained(&self) -> bool {
        self.target_pubkey == self.pubkey
    }

    fn get_privkey(&mut self) -> Option<u64> {
        while !self.target_pubkey_obtained() {
            if self.max_tries_exceeded() {
                println!("Max tries exceeded");
                return None;
            }
            self.calc_next();
        }
        Some(self.privkey)
    }
}

fn calc_encryption_key(pubkey_a: u64, privkey_b: u64, mod_val: u64) -> u64 {
    let mut k = 1;
    for _i in 0..privkey_b {
        k = (k * pubkey_a) % mod_val;
    }
    k
}

pub fn y2020_day_25_part_1<Input>(input: &mut Input) -> Result<()>
where Input: Read
{
    let mut content = String::new();
    input.read_to_string(&mut content).map_err(|_| Error::NotUtf8)?;

    let mut lines = LinesWithCount::new(content.lines());
    let card_pubkey = lines.get_an_i32()?;
    let door_pubkey = lines.get_an_i32()?;

    println!("Searching for card privkey");
    let mut card_privkey_computation = KeyComputation::new(card_pubkey as u64);
    let card_privkey = card_privkey_computation.get_privkey().ok_or_else(|| Error::NoSolution)?;
    println!("Card privkey is {}", card_privkey);

    println!("Searching for door privkey");
    let mut door_privkey_computation = KeyComputation::new(door_pubkey as u64);
    let door_privkey = door_privkey_computation.get_privkey().ok_or_else(|| Error::NoSolution)?;
    println!("Door privkey is {}", door_privkey);

    let encrypt_key = calc_encryption_key(card_pubkey as u64, door_privkey, KeyComputation::MOD);
    println!("encrypt_key privkey is {}", encrypt_key);

    Ok(())
}