use crate::tic_tac_toe::TicTacToe;

pub struct MainScreen {
    games: [(bool, Box<dyn AppWindow>); 1],
}

impl MainScreen {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            games: [(true, Box::new(TicTacToe::default()))],
        }
    }
}

impl eframe::App for MainScreen {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.games.iter_mut().for_each(|(open, game)| {
                    ui.toggle_value(open, game.name());
                })
            });
        });

        self.games
            .iter_mut()
            .for_each(|(open, game)| game.show(ctx, open));
    }
}

pub(crate) trait AppWindow {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
