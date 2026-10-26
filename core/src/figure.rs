//! Diagrams to draw beside a problem: a right triangle, two sides with the
//! angle between them, and the unit circle. Apps draw them; this is the data.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Figure {
    /// A right triangle: angle A at the bottom left, the right angle at the
    /// bottom right. `adj` and `opp` are lengths (for the shape); labels may be "".
    Right { adj: f64, opp: f64, angle: String, adj_label: String, opp_label: String, hyp_label: String },
    /// Two sides and the angle between them (in degrees); the third side is dashed.
    Sas { a: f64, b: f64, deg: f64, a_label: String, b_label: String, angle: String, third: String },
    /// The unit circle with an angle swept from 0°, and its height (sin) dashed.
    Unit { deg: f64, label: String },
}

fn s(t: &str) -> String {
    t.to_string()
}

/// The diagram beside a level's "How it works" text, if it has one.
pub fn for_lesson(level_id: &str) -> Option<Figure> {
    Some(match level_id {
        "pythagoras" => Figure::Right { adj: 4.0, opp: 3.0, angle: s(""), adj_label: s("a"), opp_label: s("b"), hyp_label: s("c") },
        "trigratios" => Figure::Right { adj: 4.0, opp: 3.0, angle: s("A"), adj_label: s("adjacent"), opp_label: s("opposite"), hyp_label: s("hypotenuse") },
        "specialangles" => Figure::Right { adj: 3f64.sqrt(), opp: 1.0, angle: s("30°"), adj_label: s("√3"), opp_label: s("1"), hyp_label: s("2") },
        "lawsincos" => Figure::Sas { a: 5.0, b: 7.0, deg: 55.0, a_label: s("a"), b_label: s("b"), angle: s("C"), third: s("c") },
        _ => return None,
    })
}
