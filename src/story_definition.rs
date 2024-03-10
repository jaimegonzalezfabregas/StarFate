
#[derive(Eq, PartialEq, Clone, Debug, Copy)]
pub enum Item {
    Tinte,
    Espada,
    Zumito,
    Naranja,
    Paquete,
    Dinero(usize),
}
use Item::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Place {
    Tienda,
    Huerto,
    Forja,
    Guarida,
    GranjaObejas,
    TiendaRopa,
    LocationOfChest(Item),
    Casa,
    EscuelaEspadachin,
    OficinaDeCorreos,
}
use Place::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum NPC {
    Padre,
    Agricultor,
    VendedorRopa,
    Vendedor,
    CuidadorObejas,
    Herrero,
    MaestroEspadachin,
    Cartero,
    Malo,
    MiniBoss,
}
use NPC::*;

impl NPC {
    pub const fn name(&self) -> &'static str {
        match self {
            Padre => "Padre",
            VendedorRopa => "Vendedor de ropa",
            CuidadorObejas => "Cuidador de obejas",
            Herrero => "Herrero",
            MaestroEspadachin => "Maestro Espadachin",
            Cartero => "Cartero",
            Malo => "Malo",
            MiniBoss => "Mini boss",
            Vendedor => "Vendedor",
            Agricultor => "Agricultor",
        }
    }

    pub const fn location(&self) -> Place {
        match self {
            Padre => Casa,
            VendedorRopa => TiendaRopa,
            CuidadorObejas => GranjaObejas,
            Herrero => Forja,
            MaestroEspadachin => EscuelaEspadachin,
            Cartero => OficinaDeCorreos,
            Malo => Guarida,
            MiniBoss => EscuelaEspadachin,
            Vendedor => Tienda,
            Agricultor => Huerto,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum InterestPoint {
    Batidora,
    OrangeTree,
}
use InterestPoint::*;
use Triger::*;
use DialogAction::*;
use StoryEffect::*;


use crate::story_builder::*;

pub fn get_story_heap() -> StoryHeap{
    StoryHeap::new(vec![
        Story{
            start_hint: "pero yo no se donde está, creo que el que cuida obejas lo sabe".to_string(),
            inmediate_substory: None,
            beats: vec![
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: Dialog(Hablar(CuidadorObejas, "Se me han escapado las obejas, ayudame a encontrarlas!".to_string())),
                    end_effect: NoEffect
                },
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: MoverObejas,
                    end_effect: NoEffect
                }
            ],
            ending_substory: None,
            end_dialog: Hablar(CuidadorObejas, "Genial! Me sacas de un apuro enorme, te has ganado saber como ir hasta donde decías".to_string()),
            end_effect: NextPlaceKnoliedge
        },
        Story{
            start_hint: "creo que el de la tienda vendía tinte".to_string(),
            inmediate_substory: None,
            beats: vec![],
            ending_substory: None,
            end_dialog: Darle(Vendedor, Dinero(20), "Aqui tienes lo que querías, muchas gracias por comprar con nosotros".to_string()),
            end_effect: NewItem(Tinte)
        },
        Story{
            start_hint: "creo que el de la tienda vendía tinte".to_string(),
            inmediate_substory: None,
            beats: vec![],
            end_dialog: Hablar(Vendedor, "Aqui tienes lo que querías, muchas gracias por comprar con nosotros".to_string()),
            ending_substory: None,
            end_effect: NewItem(Tinte)
        },
      
        Story{
            start_hint: "pero yo no se donde está, creo que el que vende ropa ha contado historias sobre aquel lugar".to_string(),
            inmediate_substory: None,
            beats: vec![],
            ending_substory: Some((Hablar(VendedorRopa, "Buenos dias, para ayudarte a encontrar el lugar del que hablas quiero que encuentres tinte por mi ".to_string()),
                SubStoryTransition{
                    sub_story_end_effect: NewItem(Tinte),
                    end_comment: "Perfecto! Te marco en el mapa el lugar que querías saber".to_string()
                }
            )),
            end_dialog: Hablar(VendedorRopa, "Perfecto! Te marco en el mapa el lugar que querías saber".to_string()),
            end_effect: NextPlaceKnoliedge
        },

        Story{
            start_hint: "podrás encontrar los ingredientes en el huerto".to_string(),
            inmediate_substory: Some(SubStoryTransition { 
                    sub_story_end_effect: NextPlaceKnoliedge, 
                    end_comment: "Ya puedes ir al huerto para conseguir los ingredientes del zumo".to_string(),
                }
            ),
            beats: vec![
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: UseIP(OrangeTree),
                    end_effect: NewItem(Naranja),
                }
            ],
            ending_substory: None,
            end_dialog: FairyComment(Box::new(SpendITOnIP(Naranja,Batidora)), "Bien! Ya hemos conseguido zumito!".to_string()),
            end_effect: NewItem(Zumito),

        },
        Story{
            start_hint: "para conseguir una espada debes hablar con el herrero".to_string(),
            inmediate_substory: None,
            beats: vec![], 
            ending_substory: Some((
                Hablar(Herrero, "Hey pequeño! Asi que quieres una espada! si quieres una espada necesitaré que me traigas un zumito".to_string()),
                SubStoryTransition{
                    sub_story_end_effect: NewItem(Zumito), 
                    end_comment: "ahora vuelve y daselo al herrero, que seguro que te está esperando".to_string()
                }
            )),
            end_dialog: Hablar(Herrero, "Muchas gracias campeón, aqui tienes tu espada!".to_string()),
            end_effect: NewItem(Espada),
        },
        Story{
            start_hint: "han secuestrado a Dulcinea, ve a por el malo".to_string(),
            inmediate_substory: Some(SubStoryTransition{
                sub_story_end_effect:StoryEffect::NewItem(Item::Espada), 
                end_comment: "ahora que tienes la espada, ve a la escuela de espadachines para aprender a usarla".to_string() 
            }),
            beats: vec![
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: Dialog(Enseñar(MaestroEspadachin, Espada, "Hola! asi que quieres entrenar conmigo? Pasa a la sala de entrenamiento por aqui".to_string())),
                    end_effect: CambiarStat("espada".to_string(),1)
                },
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: Matar(MiniBoss),
                    end_effect: NoEffect,
                },
                StoryBeatDescription{
                    
                    sub_story: Some((
                        Hablar(MaestroEspadachin,"Muy bien, ya estás preparado para enfrentarte al malo".to_string()),
                    SubStoryTransition{
                        sub_story_end_effect: NextPlaceKnoliedge, 
                        end_comment: "ahora que sabes donde esta revientalé la cara".to_string(),
                    })),
                    end_triger: Matar(Malo),
                    end_effect: NoEffect,
                },
                
            ],
            ending_substory: None,
            end_dialog: Hablar(Padre, "Muy bien hijo".to_string()),
            end_effect: EndGame
        }
    ].into_iter().chain(
        [Herrero, CuidadorObejas, VendedorRopa].into_iter().map(|recipient| Story{
            start_hint: "pero yo no se donde está, seguro que el cartero sabe como llegar".to_string(),
            inmediate_substory: None,
            beats: vec![
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: Dialog(Hablar(Cartero, format!("Hola, si quieres esa info me vas a tener que ayudar en las entregas de hoy. Este paquete debes darselo al {}", recipient.name()))),
                    end_effect: NewItem(Paquete)
                },
                StoryBeatDescription{
                    sub_story: None,
                    end_triger: Dialog(Darle(recipient,Paquete,  "Muchas gracias por el paquete! Buena suerte en tu aventura!".to_string())),
                    end_effect: NoEffect
                }
            ],
            ending_substory: None,
            end_dialog: Hablar(Cartero, "Perfecto! Te marco en el mapa el lugar que querías saber".to_string()),
            end_effect: NextPlaceKnoliedge
        })
    ).collect::<Vec<_>>())
}