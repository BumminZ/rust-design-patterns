// use crate::{
//     component::{Attack, Element},
//     product::Trainer,
// };

// use super::Builder;

// pub struct TrainerBuilder {
//     teach: Option<Element>,
//     set_attack: Option<Attack>,
//     set_defend: Option<i32>,
// }

// impl Builder for TrainerBuilder {
//     type OutputType = Trainer;

//     fn set_attack(&mut self, attack: Attack) {
//         self.set_attack = Some(attack)
//     }

//     fn set_defence(&mut self, defence: i32) {
//         self.set_defend = Some(defence)
//     }

//     fn build(self) -> Trainer {
//         Trainer::new(
//             self.teach
//                 .expect("Please set an element to teach the pokemon."),
//             self.set_attack
//                 .expect("Please set an ammount of attack and attack element."),
//             self.set_defend.expect("Please set an ammount of defence."),
//         )
//     }
// }
