pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}
