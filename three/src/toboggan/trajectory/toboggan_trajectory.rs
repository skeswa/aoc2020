/// Indicates a direction of descent down a slope.
#[derive(Debug)]
pub struct TobogganTrajectory {
    /// `(x, y)` pair describing the starting position of the toboggan before descent begins.
    starting_position: (i64, i64),
    /// `(x, y)` pair describing the direction and magnitude of descent down the slope.
    velocity: (i64, i64),
}

impl TobogganTrajectory {
    /// Creates a new `TobogganTrajectory`.
    ///
    /// `starting_position`:    `(x, y)` pair describing the starting position
    ///                         of the toboggan before descent begins.
    ///
    /// `velocity`:             `(x, y)` pair describing the direction and
    ///                         magnitude of descent down the slope.
    pub fn new(starting_position: (i64, i64), velocity: (i64, i64)) -> TobogganTrajectory {
        TobogganTrajectory {
            starting_position,
            velocity,
        }
    }

    /// Begins a descent down a slope.
    pub fn descend(&self) -> impl Iterator<Item = (i64, i64)> {
        TobogganTrajectoryDescender {
            current_x_position: self.starting_position.0,
            current_y_position: self.starting_position.1,
            has_descent_begun: false,
            velocity: self.velocity,
        }
    }
}

/// Iterates through every position along a `TobogganTrajectory`.
struct TobogganTrajectoryDescender {
    /// The current x-position of the ongoing descent.
    current_x_position: i64,
    /// The current y-position of the ongoing descent.
    current_y_position: i64,
    /// `true` if we are no longer at the starting position of descent.
    has_descent_begun: bool,
    /// `(x, y)` pair describing the direction and magnitude of descent down the slope.
    velocity: (i64, i64),
}

impl Iterator for TobogganTrajectoryDescender {
    type Item = (i64, i64);

    // Advances to the next position in the descent.
    fn next(&mut self) -> Option<(i64, i64)> {
        if !self.has_descent_begun {
            let (x_velocity, y_velocity) = self.velocity;

            self.current_x_position += x_velocity;
            self.current_y_position += y_velocity;
        } else {
            self.has_descent_begun = true;
        }

        Some((self.current_x_position, self.current_y_position))
    }
}
