use core::fmt;
use std::fmt::Display;
use ndarray::Array2;

struct Terminal<T>
where
    T: Into<char> + Clone,
{
    contents: Array2<T>,
    render_buffer: Array2<char>,
}

impl<T: Into<char> + Clone> Terminal<T> {
    // Converts everything in self.contents into chars and puts them into self.render_buffer
    fn render(&mut self) {
        self.contents.indexed_iter().for_each(|((y, x), elem)| {
            self.render_buffer[[y, x]] = elem.clone().into(); 
        })
    }
}

impl<T: Into<char> + Clone> Display for Terminal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let s = self.render_buffer.rows()
        // write!(f,s)
        todo!();
    }
}

fn main() {
    println!("Hello, world!");
}
