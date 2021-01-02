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
    pub fn descend(&self, slope_height: i64) -> TobogganTrajectoryDescender {
        TobogganTrajectoryDescender {
            current_x_position: self.starting_position.0,
            current_y_position: self.starting_position.1,
            max_y_position: slope_height - 1,
            velocity: self.velocity,
        }
    }
}

/// Iterates through every position along a `TobogganTrajectory`.
pub struct TobogganTrajectoryDescender {
    /// The current x-position of the ongoing descent.
    current_x_position: i64,
    /// The current y-position of the ongoing descent.
    current_y_position: i64,
    /// Largest valid `current_y_position`.
    max_y_position: i64,
    /// `(x, y)` pair describing the direction and magnitude of descent down the slope.
    velocity: (i64, i64),
}

impl Iterator for TobogganTrajectoryDescender {
    type Item = (i64, i64);

    // Advances to the next position in the descent.
    fn next(&mut self) -> Option<(i64, i64)> {
        let (x_velocity, y_velocity) = self.velocity;

        self.current_x_position += x_velocity;
        self.current_y_position += y_velocity;

        if self.current_y_position <= self.max_y_position {
            Some((self.current_x_position, self.current_y_position))
        } else {
            None
        }
    }
}
