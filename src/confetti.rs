//! A confetti burst drawn over the play area when a kid gets it right.

use gtk::prelude::*;
use rand::Rng;
use std::{cell::RefCell, rc::Rc};

struct Bit {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    spin: f64,
    angle: f64,
    color: (f64, f64, f64),
}

const COLORS: [(f64, f64, f64); 6] = [(1.0, 0.72, 0.01), (0.98, 0.52, 0.0), (0.27, 0.69, 0.28), (0.25, 0.5, 0.88), (0.91, 0.3, 0.55), (0.6, 0.4, 0.9)];

#[derive(Clone)]
pub struct Confetti {
    pub area: gtk::DrawingArea,
    bits: Rc<RefCell<Vec<Bit>>>,
    last: Rc<RefCell<Option<i64>>>,
}

impl Confetti {
    pub fn new() -> Self {
        let area = gtk::DrawingArea::builder().can_target(false).hexpand(true).vexpand(true).build();
        let bits: Rc<RefCell<Vec<Bit>>> = Rc::default();
        let b = bits.clone();
        area.set_draw_func(move |_, cr, _, _| {
            for bit in b.borrow().iter() {
                cr.save().ok();
                cr.translate(bit.x, bit.y);
                cr.rotate(bit.angle);
                cr.set_source_rgb(bit.color.0, bit.color.1, bit.color.2);
                cr.rectangle(-5.0, -3.0, 10.0, 6.0);
                cr.fill().ok();
                cr.restore().ok();
            }
        });
        Confetti { area, bits, last: Rc::default() }
    }

    pub fn burst(&self, count: usize) {
        let (w, h) = (self.area.width() as f64, self.area.height() as f64);
        let mut rng = rand::rng();
        let was_idle = self.bits.borrow().is_empty();
        {
            let mut bits = self.bits.borrow_mut();
            for _ in 0..count {
                bits.push(Bit {
                    x: w * rng.random_range(0.3..0.7),
                    y: h * 0.45,
                    vx: rng.random_range(-420.0..420.0),
                    vy: rng.random_range(-900.0..-300.0),
                    spin: rng.random_range(-8.0..8.0),
                    angle: rng.random_range(0.0..6.3),
                    color: COLORS[rng.random_range(0..COLORS.len())],
                });
            }
        }
        if !was_idle {
            return;
        }
        *self.last.borrow_mut() = None;
        let bits = self.bits.clone();
        let last = self.last.clone();
        self.area.add_tick_callback(move |area, clock| {
            let now = clock.frame_time();
            let dt = last.borrow().map_or(0.016, |t| ((now - t) as f64 / 1e6).min(0.05));
            *last.borrow_mut() = Some(now);
            let h = area.height() as f64;
            let mut bits = bits.borrow_mut();
            for b in bits.iter_mut() {
                b.vy += 1400.0 * dt;
                b.vx *= 0.99;
                b.x += b.vx * dt;
                b.y += b.vy * dt;
                b.angle += b.spin * dt;
            }
            bits.retain(|b| b.y < h + 20.0);
            area.queue_draw();
            if bits.is_empty() { gtk::glib::ControlFlow::Break } else { gtk::glib::ControlFlow::Continue }
        });
    }
}
