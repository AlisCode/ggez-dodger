use constants::*;
use baddies::{BaddieColor, BaddieFace};
use actions::PlayerAction;

use ggez::{Context, GameResult};
use graphics::{Point2, Rect, Vector2};


pub struct Player {
    pub position: Point2,
    pub speed: Vector2,
    pub captured: Option<(BaddieColor, BaddieFace)>,
    pub score: u32,

    fast_attenuation: bool,
}

impl Player {
    pub fn new(position: Point2) -> Player {
        Player {
            position,
            speed: Vector2::new(0.0, 0.0),
            captured: None,
            score: 0,

            fast_attenuation: false,
        }
    }

    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.position.x = (self.position.x + self.speed.x)
            .min(WIDTH - RADIUS)
            .max(RADIUS);
        self.position.y = (self.position.y + self.speed.y).min(MAX_Y).max(0.0);

        if self.on_the_ground() {
            self.speed.y = 0.0;
        } else {
            self.speed.y += JUMP_ATTENUATION * if self.fast_attenuation {
                FAST_ATTENUATION
            } else {
                1.0
            };
        }

        Ok(())
    }

    pub fn on_the_ground(&self) -> bool {
        self.position.y >= MAX_Y
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.position.x - RADIUS,
            self.position.y - RADIUS,
            RADIUS * 2.0,
            RADIUS * 2.0,
        )
    }

    pub fn process_action(&mut self, action: PlayerAction) -> GameResult<()> {
        use self::PlayerAction::*;

        match action {
            MoveLeft => self.speed.x = -PLAYER_SPEED,
            MoveRight => self.speed.x = PLAYER_SPEED,
            StopMove => self.speed.x = 0.0,
            Jump => {
                self.speed.y = -JUMP_HEIGHT;
                self.fast_attenuation = false;
            }
            Dump => self.fast_attenuation = true,
            StopDump => self.fast_attenuation = false,
        }

        Ok(())
    }
}
