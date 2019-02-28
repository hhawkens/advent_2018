use crate::day_9::types::*;
use crate::day_9::utils;

impl Circle {
    /// Places the next marble in the circle, returns the score of the marbles removed this round (if any)
    pub fn play_next_marble(&mut self) -> Score {
        if self.next_marble % 23 == 0 {
            self.play_next_marble_with_score()
        } else {
            self.play_next_marble_no_score();
            0
        }
    }

    fn play_next_marble_no_score(&mut self) {
        let one_clockwise_of_current = self.circle_index(self.current_marble_index as isize + 1);
        self.marbles.insert(one_clockwise_of_current + 1, self.next_marble);
        self.current_marble_index = one_clockwise_of_current + 1;
        self.next_marble += 1;
    }

    fn play_next_marble_with_score(&mut self) -> Score {
        let removed_marble_index = self.circle_index(self.current_marble_index as isize - 7);
        let removed_marble = self.marbles.remove(removed_marble_index).unwrap();
        let score =  self.next_marble + removed_marble;
        self.current_marble_index = self.circle_index(removed_marble_index as isize);
        self.next_marble += 1;

       score
    }

    fn circle_index(&self, index: isize) -> usize {
        utils::circular_index(index, &self.marbles)
    }
}
