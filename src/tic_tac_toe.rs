use crate::app::AppWindow;

mod marker {
    use std::ops::Not;

    const CIRCLE: &'static str = "⭕";
    const CROSS: &'static str = "❌";

    pub(super) const WIDGET_SIZE: egui::Vec2 = egui::Vec2::splat(30.0);

    #[derive(Debug, Clone, Copy)]
    pub(super) enum Marker {
        First,
        Second,
    }

    impl Default for Marker {
        fn default() -> Self {
            Self::First
        }
    }

    impl Not for Marker {
        type Output = Self;
        fn not(self) -> Self::Output {
            match self {
                Self::First => Self::Second,
                Self::Second => Self::First,
            }
        }
    }

    impl From<Marker> for &str {
        fn from(value: Marker) -> Self {
            match value {
                Marker::First => CIRCLE,
                Marker::Second => CROSS,
            }
        }
    }

    impl egui::Widget for Marker {
        fn ui(self, ui: &mut egui::Ui) -> egui::Response {
            ui.allocate_ui(WIDGET_SIZE, |ui| ui.label(Into::<&'static str>::into(self)))
                .response
        }
    }
}

mod board {
    use std::ops::{Index, IndexMut};

    use super::marker::{Marker, WIDGET_SIZE};

    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;

    #[derive(Debug, Default, Clone, Copy)]
    pub(super) struct Board {
        states: [Option<Marker>; WIDTH * HEIGHT],
    }

    impl Board {
        pub(super) fn ui(&mut self, ui: &mut egui::Ui, next: &mut Marker) {
            ui.vertical(|ui| {
                for y in 0..HEIGHT {
                    self.ui_line(ui, next, y);
                }
            });
        }
        fn ui_line(&mut self, ui: &mut egui::Ui, next: &mut Marker, y: usize) {
            ui.horizontal(|ui| {
                for x in 0..WIDTH {
                    self.ui_state(ui, next, (x, y));
                }
            });
        }
        fn ui_state(&mut self, ui: &mut egui::Ui, next: &mut Marker, (x, y): (usize, usize)) {
            let response = ui.allocate_response(WIDGET_SIZE, egui::Sense::click());
            let painter = ui.painter_at(response.rect);

            if let Some(state) = self[(x, y)] {
                painter.text(
                    response.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    Into::<&'static str>::into(state),
                    egui::FontId::new(30.0, egui::FontFamily::default()),
                    egui::Color32::WHITE,
                );
            } else if response.clicked() {
                let _ = self[(x, y)].insert(*next);
                *next = !*next;
            }

            painter.rect_stroke(
                response.rect,
                0.0,
                (
                    1.0,
                    if response.hovered() {
                        egui::Color32::RED
                    } else {
                        egui::Color32::WHITE
                    },
                ),
            );
        }
    }

    impl Index<(usize, usize)> for Board {
        type Output = Option<Marker>;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            self.states.get(to_index(index)).unwrap()
        }
    }

    impl IndexMut<(usize, usize)> for Board {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            self.states.get_mut(to_index(index)).unwrap()
        }
    }

    const fn to_index((x, y): (usize, usize)) -> usize {
        x + y * WIDTH
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct TicTacToe {
    board: board::Board,
    next: marker::Marker,
}

impl AppWindow for TicTacToe {
    fn name(&self) -> &'static str {
        "tic tac toe"
    }
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl TicTacToe {
    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::dark_canvas(ui.style())
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::splat(3.0);
                self.board.ui(ui, &mut self.next);
            })
            .response
    }
}
