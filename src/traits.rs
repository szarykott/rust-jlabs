use rand::{rngs::ThreadRng, Rng};

pub fn entrypoint() {
    let mut rand_generator = rand::thread_rng();
    println!("random from my random: {0}", get_random_succinct(&mut rand_generator));
    println!("random from my random: {0}", get_random(&mut rand_generator))
}

fn get_random<T>(randomizer: &mut T) -> String
where
    T: BussinessRandomizer,
{
    randomizer.random_number_as_string()
}

fn get_random_succinct(randomizer: &mut impl BussinessRandomizer) -> String {
    randomizer.random_number_as_string()
}

trait BussinessRandomizer {
    fn random_number_as_string(&mut self) -> String;
}

impl BussinessRandomizer for ThreadRng {
    fn random_number_as_string(&mut self) -> String {
        self.gen::<u64>().to_string()
    }
}

// impl<'a> BussinessRandomizer for &'a mut ThreadRng {
//     fn random_number_as_string(&mut self) -> String {
//         self.gen::<u64>().to_string()
//     }
// }
