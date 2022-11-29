#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(specialization)]
#![feature(concat_idents)]

pub mod base;
pub mod octave;
pub mod pitch;
pub mod named_pitch;
pub mod note;
pub mod interval;
pub mod modifier;
pub mod known_chord;
pub mod chord;


// Notes



// Intervals


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
