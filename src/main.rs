use crate::story_builder::DialogAction::*;
use crate::story_builder::StoryEffect::*; // Import the missing EndGame value
use crate::story_definition::get_story_heap;
use crate::story_definition::NPC::*; // Import the missing EndGame value

pub mod story_builder;
pub mod story_definition;

fn main() {
    let mut story_heap = get_story_heap();

    println!(
        "{:#?}",
        story_heap.render(
            &EndGame, // Use the imported EndGame value
            &Hablar(Padre, "".to_string()),
            &" -- terminaste el juego --".to_string()
        )
    );
}
