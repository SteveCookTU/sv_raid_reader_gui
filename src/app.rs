use crate::{FilterWrapper, Game, Progress, RaidResult, TeraType};
use eframe::egui::{Context, Visuals};
use eframe::{App, CreationContext, Frame};
use sysbot_rs::SysBotClient;

pub struct SVRaidReader {
    pub(crate) ip: String,
    pub(crate) port: u16,
    pub(crate) client: Option<SysBotClient>,
    pub(crate) game: Game,
    pub(crate) progress: Progress,
    pub(crate) filter: FilterWrapper,
    pub(crate) error: String,
    pub(crate) results: Vec<RaidResult>,
}

impl Default for SVRaidReader {
    fn default() -> Self {
        Self {
            ip: String::new(),
            port: 6000,
            client: None,
            game: Game::Scarlet,
            progress: Progress::None,
            filter: FilterWrapper {
                tera_type: TeraType::Any,
                ..Default::default()
            },
            error: "".to_string(),
            results: vec![],
        }
    }
}

impl SVRaidReader {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(Visuals::default());
        Default::default()
    }
}

impl App for SVRaidReader {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.draw_left_panel(ctx);
        self.draw_central_panel(ctx);
        _frame.set_window_size(ctx.used_size());
    }
}
