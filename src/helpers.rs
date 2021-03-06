use rand::{thread_rng, Rng, Rand};
use rand::distributions::range::SampleRange;

pub struct Helpers;

impl Helpers {

    pub fn new() -> Helpers{
        Helpers
    }

    pub fn shuffle<T: Clone>(&self, arr: &[T]) -> Vec<T>{
        let mut newvec = arr.to_vec();
        thread_rng().shuffle(&mut *newvec);
        newvec
    }

    pub fn replace_sym_with_number(&self, string: String) -> String {
        string.chars().map(|x| match x {
            'X' => self.number_in_range(0i32, 9i32).to_string(),
            'Z' => self.number_in_range(1i32, 9i32).to_string(),
            'N' => self.number_in_range(2i32, 9i32).to_string(),
            '#' => self.number_in_range(0i32, 9i32).to_string(),
            other => other.to_string()
        }).collect::<Vec<String>>().join("")
    }

    pub fn number<T: Rand>(&self) -> T {
        thread_rng().gen()
    }

    pub fn number_in_range<T: SampleRange + PartialOrd>(&self, min: T, max: T) -> T {
        if (min < max) || (min > max) {
            thread_rng().gen_range(min, max)
        } else {
            min
        }
    }

    pub fn array_element<'a, T>(&'a self, array: &'a [T]) -> &T {
        let index = self.number_in_range(0, array.len() - 1);
        &array[index]
    }

    pub fn sentence_case(&self, str: String) -> String {
        str.chars().enumerate().map(|(i, c)| {
            if i == 0 { c.to_uppercase().next().unwrap() } else { c }
        }).collect()
    }

    pub fn lowercase(&self, str: String) -> String {
        str.chars().enumerate().map(|(i, c)| {
            if i == 0 { c.to_lowercase().next().unwrap() } else { c }
        }).collect()
    }

}
