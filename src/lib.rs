mod delay_line;
mod lfo;

pub use delay_line::*;
pub use lfo::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
