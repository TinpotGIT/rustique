use eframe::egui;
use egui::{Color32, Vec2, RichText, Pos2, Rect, Stroke};
use crate::localization::Language;
use crate::ui_theme::RustiqueTheme;

pub enum MenuAction {
    NewCanvas(u32, u32),
    OpenFile,
}

pub enum MenuResult {
    Action(MenuAction),
    LanguageChanged(Language),
}

pub struct MainMenu {
    width: u32,
    height: u32,
    logo: Option<egui::TextureHandle>,
    language: Language,
    hover_states: HoverStates,
}

#[derive(Default)]
struct HoverStates {
    new_file: bool,
    open_project: bool,
    language_fr: bool,
    language_en: bool,
}

impl MainMenu {
    pub fn new(language: Language) -> Self {
        Self {
            width: 800,
            height: 600,
            logo: None,
            language,
            hover_states: HoverStates::default(),
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context) -> Option<MenuResult> {
        RustiqueTheme::apply_theme(ctx);
        
        let mut result = None;
        
        if self.logo.is_none() {
            self.logo = self.load_logo(ctx);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(RustiqueTheme::BACKGROUND_PRIMARY))
            .show(ctx, |ui| {
                let screen_rect = ui.max_rect();
                self.draw_background(ui, screen_rect);
                
                ui.vertical_centered(|ui| {
                    ui.add_space(RustiqueTheme::SPACING_XL * 2.0);
                    
                    self.draw_logo_section(ui);
                    
                    ui.add_space(RustiqueTheme::SPACING_XL * 1.5);
                    
                    result = self.draw_main_actions(ui);
                    
                    ui.add_space(RustiqueTheme::SPACING_LG);
                    
                    if let Some(lang_result) = self.draw_language_selector(ui) {
                        result = Some(lang_result);
                    }
                    
                    ui.add_space(RustiqueTheme::SPACING_XL);
                    
                    self.draw_canvas_settings(ui, &mut result);
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(RustiqueTheme::SPACING_MD);
                        ui.label(RustiqueTheme::muted_text("© 2024 Rustique Paint - Modern painting application"));
                    });
                });
            });

        result
    }
    
    fn draw_background(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        
        painter.rect_filled(rect, 0.0, RustiqueTheme::BACKGROUND_PRIMARY);
        
        let gradient_steps = 40;
        let gradient_width = rect.width() * 0.6;
        let gradient_height = rect.height() * 0.8;
        
        for i in 0..gradient_steps {
            let t = i as f32 / gradient_steps as f32;
            let alpha = (25.0 * (1.0 - t * t)) as u8;
            let color = Color32::from_rgba_unmultiplied(138, 101, 255, alpha);
            
            let step_width = gradient_width / gradient_steps as f32;
            let x = rect.width() - gradient_width + (t * gradient_width);
            let y = rect.height() * 0.1 + (t * gradient_height * 0.3);
            
            let step_rect = Rect::from_min_size(
                Pos2::new(x, y),
                Vec2::new(step_width * 2.0, gradient_height - (t * gradient_height * 0.5))
            );
            
            painter.rect_filled(step_rect, RustiqueTheme::rounding_large(), color);
        }
    }
    
    fn draw_logo_section(&mut self, ui: &mut egui::Ui) {
        if let Some(logo) = &self.logo {
            let logo_size = Vec2::new(200.0, 100.0);
            ui.add_space(RustiqueTheme::SPACING_SM);
            ui.image(logo, logo_size);
        } else {
            ui.add_space(RustiqueTheme::SPACING_MD);
            let title = RustiqueTheme::heading_text("Rustique", 48.0);
            ui.label(title);
            
            let subtitle = RustiqueTheme::accent_text("Modern Paint Application");
            ui.label(subtitle);
            
            let painter = ui.painter();
            let text_rect = ui.min_rect();
            let line_start = Pos2::new(text_rect.center().x - 100.0, text_rect.bottom() + 8.0);
            let line_end = Pos2::new(text_rect.center().x + 100.0, text_rect.bottom() + 8.0);
            painter.line_segment([line_start, line_end], Stroke::new(2.0, RustiqueTheme::ACCENT_PRIMARY));
        }
    }
    
    fn draw_main_actions(&mut self, ui: &mut egui::Ui) -> Option<MenuResult> {
        let mut result = None;
        let available_width = ui.available_width();
        let card_width = (available_width * 0.35).max(140.0).min(180.0);
        let card_height = (available_width * 0.15).max(100.0).min(140.0);
        let spacing = (available_width * 0.02).max(8.0).min(24.0);
        
        ui.horizontal(|ui| {
            ui.add_space(spacing);
            
            RustiqueTheme::card_frame().show(ui, |ui| {
                ui.set_min_size(Vec2::new(card_width, card_height));
                ui.vertical_centered(|ui| {
                    ui.add_space(RustiqueTheme::SPACING_MD);
                    
                    let icon_size = (card_width * 0.25).max(24.0).min(36.0);
                    let new_file_icon = "📄";
                    ui.label(RichText::new(new_file_icon).size(icon_size));
                    
                    ui.add_space(RustiqueTheme::SPACING_SM);
                    
                    let btn_width = (card_width * 0.8).max(100.0).min(140.0);
                    let btn_height = (card_height * 0.25).max(30.0).min(40.0);
                    
                    let btn_text = RustiqueTheme::body_text("New File");
                    let btn = ui.add(
                        egui::Button::new(btn_text)
                            .fill(RustiqueTheme::ACCENT_PRIMARY)
                            .stroke(egui::Stroke::new(1.0, RustiqueTheme::ACCENT_PRIMARY))
                            .rounding(RustiqueTheme::rounding_medium())
                            .min_size(Vec2::new(btn_width, btn_height))
                    );
                    
                    if btn.clicked() {
                        result = Some(MenuResult::Action(MenuAction::NewCanvas(self.width, self.height)));
                    }
                    
                    self.hover_states.new_file = btn.hovered();
                    ui.add_space(RustiqueTheme::SPACING_MD);
                });
            });
            
            ui.add_space(spacing);
            
            RustiqueTheme::card_frame().show(ui, |ui| {
                ui.set_min_size(Vec2::new(card_width, card_height));
                ui.vertical_centered(|ui| {
                    ui.add_space(RustiqueTheme::SPACING_MD);
                    
                    let icon_size = (card_width * 0.25).max(24.0).min(36.0);
                    let open_icon = "📂";
                    ui.label(RichText::new(open_icon).size(icon_size));
                    
                    ui.add_space(RustiqueTheme::SPACING_SM);
                    
                    let btn_width = (card_width * 0.8).max(100.0).min(140.0);
                    let btn_height = (card_height * 0.25).max(30.0).min(40.0);
                    
                    let btn_text = RustiqueTheme::body_text("Open Project");
                    let btn = ui.add(
                        egui::Button::new(btn_text)
                            .fill(RustiqueTheme::SURFACE_PRIMARY)
                            .stroke(egui::Stroke::new(1.0, RustiqueTheme::BORDER_LIGHT))
                            .rounding(RustiqueTheme::rounding_medium())
                            .min_size(Vec2::new(btn_width, btn_height))
                    );
                    
                    if btn.clicked() {
                        result = Some(MenuResult::Action(MenuAction::OpenFile));
                    }
                    
                    self.hover_states.open_project = btn.hovered();
                    ui.add_space(RustiqueTheme::SPACING_MD);
                });
            });
        });
        
        result
    }
    
    fn draw_language_selector(&mut self, ui: &mut egui::Ui) -> Option<MenuResult> {
        let mut result = None;
        
        RustiqueTheme::panel_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RustiqueTheme::muted_text("Language:"));
                ui.add_space(RustiqueTheme::SPACING_SM);
                
                let is_french = matches!(self.language, Language::French);
                let fr_btn = ui.add(
                    egui::Button::new(RustiqueTheme::body_text("🇫🇷 Français"))
                        .fill(if is_french { RustiqueTheme::ACCENT_PRIMARY } else { Color32::TRANSPARENT })
                        .stroke(egui::Stroke::new(
                            if is_french { 2.0 } else { 1.0 }, 
                            if is_french { RustiqueTheme::ACCENT_PRIMARY } else { RustiqueTheme::BORDER_LIGHT }
                        ))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(Vec2::new(90.0, 28.0))
                );
                
                if fr_btn.clicked() {
                    self.language = Language::French;
                    result = Some(MenuResult::LanguageChanged(Language::French));
                }
                
                ui.add_space(RustiqueTheme::SPACING_XS);
                
                let is_english = matches!(self.language, Language::English);
                let en_btn = ui.add(
                    egui::Button::new(RustiqueTheme::body_text("🇺🇸 English"))
                        .fill(if is_english { RustiqueTheme::ACCENT_PRIMARY } else { Color32::TRANSPARENT })
                        .stroke(egui::Stroke::new(
                            if is_english { 2.0 } else { 1.0 }, 
                            if is_english { RustiqueTheme::ACCENT_PRIMARY } else { RustiqueTheme::BORDER_LIGHT }
                        ))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(Vec2::new(90.0, 28.0))
                );
                
                if en_btn.clicked() {
                    self.language = Language::English;
                    result = Some(MenuResult::LanguageChanged(Language::English));
                }
            });
        });
        
        result
    }
    
    fn draw_canvas_settings(&mut self, ui: &mut egui::Ui, result: &mut Option<MenuResult>) {
        let available_width = ui.available_width();
        let panel_width = (available_width * 0.6).max(300.0).min(500.0);
        
        RustiqueTheme::card_frame().show(ui, |ui| {
            ui.set_min_width(panel_width);
            ui.vertical_centered(|ui| {
                ui.label(RustiqueTheme::heading_text("Canvas Settings", 18.0));
                ui.add_space(RustiqueTheme::SPACING_MD);
                
                let input_width = (panel_width * 0.4).max(80.0).min(120.0);
                
                ui.horizontal(|ui| {
                    ui.label(RustiqueTheme::body_text("Width:"));
                    ui.add_space(RustiqueTheme::SPACING_SM);
                    let width_drag = egui::DragValue::new(&mut self.width)
                        .speed(1)
                        .clamp_range(100..=4000)
                        .suffix(" px");
                    ui.add_sized(Vec2::new(input_width, 24.0), width_drag);
                });
                
                ui.add_space(RustiqueTheme::SPACING_SM);
                
                ui.horizontal(|ui| {
                    ui.label(RustiqueTheme::body_text("Height:"));
                    ui.add_space(RustiqueTheme::SPACING_SM);
                    let height_drag = egui::DragValue::new(&mut self.height)
                        .speed(1)
                        .clamp_range(100..=4000)
                        .suffix(" px");
                    ui.add_sized(Vec2::new(input_width, 24.0), height_drag);
                });
                
                ui.add_space(RustiqueTheme::SPACING_MD);
                
                let btn_width = (panel_width * 0.6).max(180.0).min(250.0);
                let create_btn = ui.add(
                    egui::Button::new(RustiqueTheme::heading_text("Create Canvas", 16.0))
                        .fill(RustiqueTheme::ACCENT_PRIMARY)
                        .stroke(egui::Stroke::new(1.0, RustiqueTheme::ACCENT_PRIMARY))
                        .rounding(RustiqueTheme::rounding_medium())
                        .min_size(Vec2::new(btn_width, 42.0))
                );
                
                if create_btn.clicked() {
                    *result = Some(MenuResult::Action(MenuAction::NewCanvas(self.width, self.height)));
                }
            });
        });
    }
    
    fn load_logo(&self, ctx: &egui::Context) -> Option<egui::TextureHandle> {
        match image::open("rustique.png") {
            Ok(image) => {
                let image_buffer = image.to_rgba8();
                let size = [image_buffer.width() as _, image_buffer.height() as _];
                let image_data = egui::ColorImage::from_rgba_unmultiplied(
                    size, 
                    image_buffer.as_flat_samples().as_slice()
                );
                Some(ctx.load_texture("logo", image_data, egui::TextureOptions::LINEAR))
            },
            Err(_) => {
                let width = 200;
                let height = 100;
                let mut pixels = vec![0; width * height * 4];
                
                for y in 0..height {
                    for x in 0..width {
                        let t = (x as f32 / width as f32) * 2.0 * std::f32::consts::PI;
                        let intensity = ((t.sin() + 1.0) * 0.5 * 255.0) as u8;
                        let idx = (y * width + x) * 4;
                        pixels[idx] = 138;
                        pixels[idx + 1] = 101;
                        pixels[idx + 2] = 255;
                        pixels[idx + 3] = intensity;
                    }
                }
                
                let image_data = egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels);
                Some(ctx.load_texture("default_logo", image_data, egui::TextureOptions::LINEAR))
            }
        }
    }
}
