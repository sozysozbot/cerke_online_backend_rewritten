
use cetkaik_full_state_transition::{Rate, Season, Config, state};
use cetkaik_core::absolute::Field;
use rand::{Rng, prelude::ThreadRng};
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr,Deserialize_repr};

use super::MoveToBePolled;

pub type AbsoluteCoord = cetkaik_core::absolute::Coord;

pub struct GameState {
    pub state: state::A,
    pub config: Config,
    pub waiting_for_after_half_acceptance: Option<SrcStep>,
    pub moves_to_be_polled: [Vec<MovePiece>; 4],
}

impl GameState { 
    fn is_ia_owner_s_turn(&self) -> bool {
        self.state.whose_turn == cetkaik_core::absolute::Side::IASide
    }
}


pub struct SrcStep {
    pub src: AbsoluteCoord,
    pub step: AbsoluteCoord,
}

pub struct MovePiece {
    pub mov: MoveToBePolled,
    pub status: Option<HandCompletionStatus>,
    pub by_ia_owner: bool,
}
pub enum HandCompletionStatus {
    TyMok,
    TaXot,
    NotYetDetermined,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Kok1 = 0,
    Huok2 = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Profession {
    Nuak1 = 0,
    Kauk2 = 1,
    Gua2 = 2,
    Kaun1 = 3,
    Dau2 = 4,
    Maun1 = 5,
    Kua2 = 6,
    Tuk2 = 7,
    Uai1 = 8,
    Io = 9,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
// Using boolean is natural, and this is also necessary to allow easy interop with the frontend
#[allow(clippy::struct_excessive_bools)]
pub struct Ciurl(bool, bool, bool, bool, bool);

impl Ciurl {
    pub fn new(rng: &mut ThreadRng) -> Ciurl {
        Ciurl(rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
    pub fn count(self) -> usize {
        self.0 as usize + self.1 as usize + self.2 as usize + self.3 as usize + self.4 as usize
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NormalMove {
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[serde(tag = "type")]

pub enum NonTamMoveDotData {
    FromHand {
        color: Color,
        profession: Profession,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
    },
    SrcDst {
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        src: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
        #[serde(skip_serializing_if = "Option::is_none")]
        water_entry_ciurl: Option<Ciurl>,
    },
    SrcStepDstFinite {
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        src: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        step: AbsoluteCoord,
        #[serde(serialize_with="serialize_coord",deserialize_with="deserialize_coord")]
        dest: AbsoluteCoord,
        #[serde(skip_serializing_if = "Option::is_none")]
        water_entry_ciurl: Option<Ciurl>,
    },
}
fn serialize_coord<S>(value: &AbsoluteCoord, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&value.0)?;
        seq.serialize_element(&value.1)?;
        seq.end()
    }

struct CoordVisitor;

impl<'de> serde::de::Visitor<'de> for CoordVisitor {
    type Value = AbsoluteCoord;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a coordinate")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<AbsoluteCoord, V::Error> where
    V: serde::de::SeqAccess<'de>{
        use cetkaik_core::absolute::{Column,Row,Coord};
        let mut column = None;
        let mut row = None;
        
        for i in 0..2 {
            if let Some(item) = visitor.next_element()? {
                match item {
                    "C" => { column = Some(Column::C)}
                    "K" => { column = Some(Column::K)}
                    "L" => { column = Some(Column::L)}
                    "M" => { column = Some(Column::M)}
                    "N" => { column = Some(Column::N)}
                    "P" => { column = Some(Column::P)}
                    "T" => { column = Some(Column::T)}
                    "X" => { column = Some(Column::X)}
                    "Z" => { column = Some(Column::Z)}

                    "A" => { row = Some(Row::A)} 
                    "AI" => { row = Some(Row::AI)} 
                    "AU" => { row = Some(Row::AU)} 
                    "E" => { row = Some(Row::E)} 
                    "I" => { row = Some(Row::I)} 
                    "O" => { row = Some(Row::O)} 
                    "U" => { row = Some(Row::U)} 
                    "Y" => { row = Some(Row::Y)} 
                    "IA" => { row = Some(Row::IA)} 

                    _ => {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(item),
                            &self,
                        ))
                    }
                }
            } else { 
                return Err(
                    serde::de::Error::invalid_length(i, &"2")
                );
            }
        }

        if let Some(column) = column { 
            if let Some(row) = row {
                Ok( Coord(row, column) )
            } else {
                Err(
                    serde::de::Error::missing_field("row")
                )
            }
        } else {
            Err(
                serde::de::Error::missing_field("column")
            )
        }

    }
}

fn deserialize_coord<'de, D>(deserializer: D) -> Result<AbsoluteCoord, D::Error>
where
D: serde::Deserializer<'de> {
    let visitor = CoordVisitor;
    deserializer.deserialize_tuple(2, visitor)
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[serde(tag = "stepStyle")]
pub enum TamMoveInternal {
    NoStep {
        src: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringFormer {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringLatter {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },
}