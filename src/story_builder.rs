use crate::story_builder::StoryEffect::*;
use crate::story_builder::Triger::*;
use crate::story_definition::*;
use rand::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum DialogAction {
    Hablar(NPC, String),
    Darle(NPC, Item, String),
    Enseñar(NPC, Item, String),
    FairyComment(Box<Triger>, String),
}

impl DialogAction {
    fn add_comment(&self, comment: String) -> DialogAction {
        match self {
            DialogAction::Hablar(npc, text) => {
                DialogAction::Hablar(*npc, format!("{} {}", text, comment))
            }
            DialogAction::Darle(npc, item, text) => {
                DialogAction::Darle(*npc, *item, format!("{} {}", text, comment))
            }
            DialogAction::Enseñar(npc, item, text) => {
                DialogAction::Enseñar(*npc, *item, format!("{} {}", text, comment))
            }
            DialogAction::FairyComment(triger, text) => {
                DialogAction::FairyComment(triger.to_owned(), format!("{} {}", text, comment))
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Triger {
    Dialog(DialogAction),
    Matar(NPC),
    MoverObejas,
    UseIP(InterestPoint),
    SpendITOnIP(Item, InterestPoint),
    UseITOnIP(Item, InterestPoint),
    GetCloseTo(InterestPoint, String),
    Enter(Place),
    Inmediate,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StoryBeatDescription {
    pub sub_story: Option<(DialogAction, SubStoryTransition)>,
    pub end_triger: Triger,
    pub end_effect: StoryEffect,
}

impl StoryBeatDescription {
    fn render(&self, sh: &mut StoryHeap) -> Result<Vec<StoryBeat>, String> {
        match &self.sub_story {
            None => Ok(vec![StoryBeat {
                end_triger: self.end_triger.to_owned(),
                end_effect: self.end_effect.to_owned(),
            }]),
            Some((starting_dialog, sub_story_transition)) => sh.render(
                &sub_story_transition.sub_story_end_effect,
                starting_dialog,
                &sub_story_transition.end_comment,
            ),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StoryBeat {
    end_triger: Triger,
    end_effect: StoryEffect,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SubStoryTransition {
    pub sub_story_end_effect: StoryEffect,
    pub end_comment: String,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Story {
    pub start_hint: String,
    pub inmediate_substory: Option<SubStoryTransition>,
    pub beats: Vec<StoryBeatDescription>,
    pub ending_substory: Option<(DialogAction, SubStoryTransition)>,
    pub end_dialog: DialogAction,
    pub end_effect: StoryEffect,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum StoryEffect {
    NoEffect,
    EndGame,
    NextPlaceKnoliedge,
    NewItem(Item),
    CambiarStat(String, usize),
}

pub struct StoryHeap {
    stories: Vec<(bool, Story)>,
}

impl StoryHeap {
    pub fn new(stories: Vec<Story>) -> Self {
        StoryHeap {
            stories: stories.into_iter().map(|e| (false, e)).collect(),
        }
    }

    pub fn reset_used(&mut self) {
        self.stories.iter_mut().for_each(|(used, _)| *used = false);
    }

    pub fn render(
        &mut self,
        effect: &StoryEffect,
        starting_dialog: &DialogAction,
        end_comment: &String,
    ) -> Result<Vec<StoryBeat>, String> {
        let mut story_line = vec![];

        let choosen_story = self.get(effect)?;

        let introduction = starting_dialog.add_comment(choosen_story.start_hint);

        if let Some(sub_story) = choosen_story.inmediate_substory {
            story_line.extend(
                self.render(
                    &sub_story.sub_story_end_effect,
                    &introduction,
                    &sub_story.end_comment,
                )?
                .into_iter(),
            );
        } else {
            story_line.push(StoryBeat {
                end_triger: Dialog(introduction),
                end_effect: NoEffect,
            });
        }

        story_line.extend(
            choosen_story
                .beats
                .iter()
                .map(|e| e.render(self))
                .collect::<Result<Vec<_>, String>>()?
                .into_iter()
                .flatten(),
        );

        if let Some((start_dialog, sub_story)) = choosen_story.ending_substory {
            story_line.extend(
                self.render(
                    &sub_story.sub_story_end_effect,
                    &start_dialog,
                    &sub_story.end_comment,
                )?
                .into_iter(),
            );
        }

        story_line.push(StoryBeat {
            end_triger: Dialog(
                choosen_story
                    .end_dialog
                    .add_comment(end_comment.to_string()),
            ),
            end_effect: choosen_story.end_effect.to_owned(),
        });

        Ok(story_line)
    }

    pub fn get(&mut self, effect: &StoryEffect) -> Result<Story, String> {
        let mut options = self
            .stories
            .iter_mut()
            .filter(|(used, story)| !*used && story.end_effect == *effect)
            .collect::<Vec<_>>();
        match options.choose_mut(&mut rand::thread_rng()) {
            Some(story) => {
                story.0 = true;
                Ok(story.1.to_owned().to_owned())
            }
            None => Err("No story found".to_string()),
        }
    }
}
