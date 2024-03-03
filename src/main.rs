use rand::prelude::*;


#[derive(Clone, Debug)]
struct StoryBeat{
    start_hint: &'static str,
    end_condition_location_hint: Place,
    sub_story: Option<AnyStoryWithEffect>,
    end_condition: Action,
}

enum Asset{
    NPC(NPC),
    Structure(Place),
}

#[derive(Clone, Debug)]
struct Story{
    beats: Vec<StoryBeat>,
}

impl Story{
    fn print(&self, margin: String, stories: &Vec<Story> ){
        for beat in &self.beats{
            println!("{}con la pista: {}", margin, beat.start_hint);
            if let Some(sub_story) = &beat.sub_story{
                sub_story.get(stories).print(format!("  {}",margin), stories);
            }
            println!("{}situacion necesaria para continuar: [{:?}]", margin, beat.end_condition);

        }
        
    }

    fn hasEffect(&self, effect: &StoryEffect) -> bool{
        self.beats.iter().any(|beat| beat.sub_story.as_ref().map_or(false, |sub_story| sub_story.0 == *effect))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Item{
    Tinte,
    Espada,
    Zumito,
    Naranja,
}
use Item::*;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Place{
    Tienda,
    Huerto,
    Forja,
    Guarida,
    GranjaObejas,
    TiendaRopa,
    LocationOfChest(Item),
    Casa,
    EscuelaEspadachin,
}
use Place::*;

#[derive(Eq, PartialEq, Clone, Debug)]
enum NPC{
    VendedorRopa,
    CuidadorObejas,
    Herrero,
    MaestroEspadachin,
    Malo,
}
use NPC::*;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Action{
    Tener(Item),
    Hablar(NPC, &'static str, StoryEffect),
    Matar(NPC, StoryEffect),
    Darle(NPC, Item, StoryEffect),
    MoverObejas,
}
use Action::*;

#[derive(Eq, PartialEq, Clone, Debug)]
enum StoryEffect{
    NoEffect,
    EndGame,
    NextPlaceKnoliedge,
    NewItem(Item),
    CambiarStat(&'static str, usize),
}
use StoryEffect::*;

#[derive(Clone, Debug)]
struct AnyStoryWithEffect(StoryEffect);


impl AnyStoryWithEffect {
    fn get(&self, stories: &Vec<Story>) -> Story{
        let options = stories.iter().filter(|e| e.hasEffect(&self.0)).collect::<Vec<_>>();
        options.choose(&mut rand::thread_rng()).unwrap().to_owned().to_owned()
    }
}

fn main(){

    let stories = vec![
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "pero yo no se donde está, creo que el que cuida obejas lo sabe",
                    end_condition_location_hint: GranjaObejas,
                    sub_story: None,
                    end_condition: Hablar(CuidadorObejas, "Buenos dias, blabla, ", NoEffect),
                },
                StoryBeat{
                    start_hint: "se me han escapado las obejas, ayudame a encontrarlas",
                    end_condition_location_hint: GranjaObejas,
                    sub_story: None,
                    end_condition: MoverObejas,
                }
            ],
        },
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "el de la tienda lo vende",
                    end_condition_location_hint: Tienda,
                    sub_story: None,
                    end_condition: Tener(Tinte),
                }
            ],
        },
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "hay en una caja al otro lado de la murralla",
                    end_condition_location_hint: LocationOfChest(Tinte),
                    sub_story: None,
                    end_condition: Tener(Tinte),
                }
            ],
        },
      
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "pero yo no se donde está, creo que el que vende ropa ha contado historias sobre aquel lugar",
                    end_condition_location_hint: TiendaRopa,
                    sub_story: None,
                    end_condition: Hablar(VendedorRopa, "Buenos dias, para ayudarte a encontrar el lugar del que hablas ", NoEffect),
                },
                StoryBeat{
                    start_hint: "debes conseguirme tinte",
                    end_condition_location_hint: TiendaRopa,
                    sub_story: Some(AnyStoryWithEffect(StoryEffect::NewItem(Item::Tinte))),
                    end_condition: Darle(VendedorRopa, Tinte, NextPlaceKnoliedge),
                },
            ],
        },

        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "debeis ir al huerto, donde encontrarás los ingredientes",
                    sub_story: Some(AnyStoryWithEffect(StoryEffect::NextPlaceKnoliedge)),
                    end_condition_location_hint: Huerto,
                    end_condition: Tener(Naranja),
                },
                StoryBeat{
                    start_hint: "ve a la cocina a hacer zumo",
                    sub_story: None,
                    end_condition_location_hint: Casa,
                    end_condition: Tener(Zumito),
                }
            ],
        },
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "para conseguir una espada debes hablar con el herrero",
                    sub_story: None,
                    end_condition_location_hint: Forja,
                    end_condition: Hablar(Herrero, "Hey pequeño! Asi que quieres una espada! ", NoEffect),
                },
                StoryBeat{
                    start_hint: "si quieres una espada necesitaré que me traigas un zumito",
                    sub_story: Some(AnyStoryWithEffect(StoryEffect::NewItem(Item::Zumito))),
                    end_condition_location_hint: Forja,
                    end_condition: Darle(Herrero, Zumito, NewItem(Espada)),
                }
            ],
        },
        Story{
            beats: vec![
                StoryBeat{
                    start_hint: "han secuestrado a dulcinea, necesitas una espada para batirte en duelo",
                    sub_story: Some(AnyStoryWithEffect(StoryEffect::NewItem(Item::Espada))),
                    end_condition_location_hint: EscuelaEspadachin,
                    end_condition: Hablar(MaestroEspadachin, "Hola! asi que quieres entrenar conmigo?", CambiarStat("espada",1)),
                },
                StoryBeat{
                    start_hint: "antes de ir a la guarida del malo deberás practicar, ve a la sala de entrenamiento",
                    sub_story: None,
                    end_condition_location_hint: EscuelaEspadachin,
                    end_condition: Matar(Malo, EndGame),
                },
                StoryBeat{
                    start_hint: "ahora debes irle a partirle la cara al malo alla donde esté",
                    sub_story: Some(AnyStoryWithEffect(StoryEffect::NextPlaceKnoliedge)),
                    end_condition_location_hint: Guarida,
                    end_condition: Matar(Malo, EndGame),
                },
                
            ],
        }

    ];


    AnyStoryWithEffect(StoryEffect::EndGame).get(&stories).print(String::new(),&stories);
   
    
}
