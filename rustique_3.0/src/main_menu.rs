use eframe::egui;
use egui::{Color32, Vec2, RichText, Pos2, Rect, Stroke, TextureHandle, TextureOptions};
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
    background: Option<egui::TextureHandle>,
    language: Language,
}

impl MainMenu {
    pub fn new(language: Language) -> Self {
        Self {
            width: 800,
            height: 600,
            logo: None,
            background: None,
            language,
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context) -> Option<MenuResult> {
        RustiqueTheme::apply_theme(ctx);
        
        let mut result = None;
        
        // Charger le logo et l'arrière-plan si nécessaire
        if self.logo.is_none() {
            self.logo = self.load_logo(ctx);
        }
        if self.background.is_none() {
            self.background = self.load_background(ctx);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(RustiqueTheme::BACKGROUND_PRIMARY))
            .show(ctx, |ui| {
                let screen_rect = ui.max_rect();
                
                // Dessiner l'arrière-plan
                self.draw_background(ui, screen_rect);
                
                // Interface principale
                ui.allocate_ui_at_rect(screen_rect, |ui| {
                    ui.vertical_centered(|ui| {
                        // Espacement du haut
                        ui.add_space(screen_rect.height() * 0.15);
                        
                        // Logo/Titre
                        self.draw_title_section(ui);
                        
                        ui.add_space(screen_rect.height() * 0.08);
                        
                        // Actions principales
                        result = self.draw_action_buttons(ui);
                        
                        ui.add_space(screen_rect.height() * 0.06);
                        
                        // Paramètres de canvas
                        self.draw_canvas_section(ui, &mut result);
                        
                        // Pousser le sélecteur de langue vers le bas
                        let remaining_space = ui.available_height() - 100.0;
                        ui.add_space(remaining_space.max(20.0));
                        
                        // Sélecteur de langue en bas
                        if let Some(lang_result) = self.draw_language_section(ui) {
                            result = Some(lang_result);
                        }
                    });
                });
            });

        result
    }
    
    fn load_background(&self, ctx: &egui::Context) -> Option<TextureHandle> {
        // Essayer de charger une image de fond personnalisée
        if let Ok(image) = image::open("background.png") {
            let image_buffer = image.to_rgba8();
            let size = [image_buffer.width() as _, image_buffer.height() as _];
            let image_data = egui::ColorImage::from_rgba_unmultiplied(
                size, 
                image_buffer.as_flat_samples().as_slice()
            );
            Some(ctx.load_texture("background", image_data, TextureOptions::LINEAR))
        } else {
            None
        }
    }
    
    fn draw_background(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        
        if let Some(bg_texture) = &self.background {
            // Dessiner l'image de fond en couvrant tout l'écran
            painter.image(
                bg_texture.id(),
                rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE
            );
        } else {
            // Fallback : dégradé simple et propre
            painter.rect_filled(rect, 0.0, RustiqueTheme::BACKGROUND_PRIMARY);
            
            // Dégradé subtil
            let gradient_rect = Rect::from_min_size(
                rect.center() - Vec2::new(300.0, 200.0),
                Vec2::new(600.0, 400.0)
            );
            
            for i in 0..50 {
                let t = i as f32 / 50.0;
                let alpha = (30.0 * (1.0 - t * t)) as u8;
                let color = Color32::from_rgba_unmultiplied(138, 101, 255, alpha);
                let radius = 300.0 * t;
                
                painter.circle_filled(gradient_rect.center(), radius, color);
            }
        }
    }
    
    fn draw_title_section(&mut self, ui: &mut egui::Ui) {
        if let Some(logo) = &self.logo {
            // Corriger le ratio d'aspect du logo pour éviter l'écrasement
            let logo_size = Vec2::new(250.0, 125.0); // Ratio 2:1 plus naturel
            ui.image(logo, logo_size);
        } else {
            // Titre principal
            ui.label(
                RichText::new("Rustique")
                    .size(56.0)
                    .color(Color32::WHITE)
                    .strong()
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Modern Paint Application")
                    .size(18.0)
                    .color(Color32::from_gray(200))
            );
        }
    }
    
    fn draw_action_buttons(&mut self, ui: &mut egui::Ui) -> Option<MenuResult> {
        let mut result = None;
        
        ui.horizontal(|ui| {
            let available_width = ui.available_width();
            let button_width = 180.0;
            let spacing = 30.0;
            let total_width = button_width * 2.0 + spacing;
            let padding = (available_width - total_width) / 2.0;
            
            ui.add_space(padding.max(0.0));
            
            // Bouton New File
            if ui.add(
                egui::Button::new(
                    RichText::new("📄 New File")
                        .size(16.0)
                        .color(Color32::WHITE)
                        .strong()
                )
                .fill(RustiqueTheme::ACCENT_PRIMARY)
                .stroke(Stroke::new(2.0, RustiqueTheme::ACCENT_PRIMARY))
                .rounding(RustiqueTheme::rounding_medium())
                .min_size(Vec2::new(button_width, 50.0))
            ).clicked() {
                result = Some(MenuResult::Action(MenuAction::NewCanvas(self.width, self.height)));
            }
            
            ui.add_space(spacing);
            
            // Bouton Open Project
            if ui.add(
                egui::Button::new(
                    RichText::new("📂 Open Project")
                        .size(16.0)
                        .color(RustiqueTheme::TEXT_PRIMARY)
                        .strong()
                )
                .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 30))
                .stroke(Stroke::new(2.0, Color32::from_rgba_unmultiplied(255, 255, 255, 100)))
                .rounding(RustiqueTheme::rounding_medium())
                .min_size(Vec2::new(button_width, 50.0))
            ).clicked() {
                result = Some(MenuResult::Action(MenuAction::OpenFile));
            }
        });
        
        result
    }
    
    fn draw_canvas_section(&mut self, ui: &mut egui::Ui, result: &mut Option<MenuResult>) {
        // Panneau de paramètres de canvas
        ui.allocate_ui_with_layout(
            Vec2::new(450.0, 220.0), // Augmenter légèrement la largeur
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                // Fond semi-transparent
                let panel_rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(
                    panel_rect,
                    RustiqueTheme::rounding_large(),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 120)
                );
                
                ui.add_space(20.0);
                
                ui.label(
                    RichText::new("Canvas Settings")
                        .size(20.0)
                        .color(Color32::WHITE)
                        .strong()
                );
                
                ui.add_space(20.0);
                
                // Contrôles de taille - parfaitement centrés
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Calculer l'espace disponible et centrer manuellement
                    let total_content_width = 250.0; // Estimation de la largeur totale du contenu
                    let available_width = ui.available_width();
                    let padding = (available_width - total_content_width) / 2.0;
                    
                    ui.add_space(padding.max(0.0));
                    
                    ui.label(
                        RichText::new("Size:")
                            .size(14.0)
                            .color(Color32::from_gray(220))
                    );
                    
                    ui.add_space(8.0);
                    
                    ui.add_sized(
                        Vec2::new(80.0, 24.0),
                        egui::DragValue::new(&mut self.width)
                            .speed(1)
                            .clamp_range(100..=4000)
                            .suffix(" px")
                    );
                    
                    ui.add_space(8.0);
                    
                    ui.label(
                        RichText::new("×")
                            .size(16.0)
                            .color(Color32::from_gray(180))
                    );
                    
                    ui.add_space(8.0);
                    
                    ui.add_sized(
                        Vec2::new(80.0, 24.0),
                        egui::DragValue::new(&mut self.height)
                            .speed(1)
                            .clamp_range(100..=4000)
                            .suffix(" px")
                    );
                });
                
                ui.add_space(15.0);
                
                // Préréglages - parfaitement centrés
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // Calculer l'espace pour centrer les boutons de préréglages
                    let preset_width = 65.0;
                    let spacing = 8.0;
                    let num_buttons = 4;
                    let total_buttons_width = (preset_width * num_buttons as f32) + (spacing * (num_buttons - 1) as f32);
                    let available_width = ui.available_width();
                    let padding = (available_width - total_buttons_width) / 2.0;
                    
                    ui.add_space(padding.max(0.0));
                    
                    let preset_size = Vec2::new(preset_width, 28.0);
                    
                    if ui.add(
                        egui::Button::new(
                            RichText::new("HD")
                                .size(13.0)
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 20))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60)))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(preset_size)
                    ).clicked() {
                        self.width = 1280;
                        self.height = 720;
                    }
                    
                    ui.add_space(spacing);
                    
                    if ui.add(
                        egui::Button::new(
                            RichText::new("FHD")
                                .size(13.0)
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 20))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60)))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(preset_size)
                    ).clicked() {
                        self.width = 1920;
                        self.height = 1080;
                    }
                    
                    ui.add_space(spacing);
                    
                    if ui.add(
                        egui::Button::new(
                            RichText::new("4K")
                                .size(13.0)
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 20))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60)))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(preset_size)
                    ).clicked() {
                        self.width = 3840;
                        self.height = 2160;
                    }
                    
                    ui.add_space(spacing);
                    
                    if ui.add(
                        egui::Button::new(
                            RichText::new("Square")
                                .size(13.0)
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 20))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60)))
                        .rounding(RustiqueTheme::rounding_small())
                        .min_size(preset_size)
                    ).clicked() {
                        self.width = 1024;
                        self.height = 1024;
                    }
                });
                
                ui.add_space(20.0);
                
                // Bouton Create Canvas - parfaitement centré
                ui.vertical_centered(|ui| {
                    if ui.add(
                        egui::Button::new(
                            RichText::new("🎨 Create Canvas")
                                .size(18.0)
                                .color(Color32::WHITE)
                                .strong()
                        )
                        .fill(RustiqueTheme::ACCENT_PRIMARY)
                        .stroke(Stroke::new(2.0, RustiqueTheme::ACCENT_HOVER))
                        .rounding(RustiqueTheme::rounding_medium())
                        .min_size(Vec2::new(220.0, 45.0))
                    ).clicked() {
                        *result = Some(MenuResult::Action(MenuAction::NewCanvas(self.width, self.height)));
                    }
                });
                
                ui.add_space(20.0);
            }
        );
    }
    
    fn draw_language_section(&mut self, ui: &mut egui::Ui) -> Option<MenuResult> {
        let mut result = None;
        
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Language:")
                    .size(14.0)
                    .color(Color32::from_gray(180))
            );
            
            ui.add_space(10.0);
            
            let is_french = matches!(self.language, Language::French);
            if ui.add(
                egui::Button::new("🇫🇷 Français")
                    .fill(if is_french { RustiqueTheme::ACCENT_PRIMARY } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(1.0, if is_french { RustiqueTheme::ACCENT_PRIMARY } else { Color32::from_gray(100) }))
                    .min_size(Vec2::new(90.0, 25.0))
            ).clicked() {
                self.language = Language::French;
                result = Some(MenuResult::LanguageChanged(Language::French));
            }
            
            ui.add_space(5.0);
            
            let is_english = matches!(self.language, Language::English);
            if ui.add(
                egui::Button::new("🇺🇸 English")
                    .fill(if is_english { RustiqueTheme::ACCENT_PRIMARY } else { Color32::TRANSPARENT })
                    .stroke(Stroke::new(1.0, if is_english { RustiqueTheme::ACCENT_PRIMARY } else { Color32::from_gray(100) }))
                    .min_size(Vec2::new(90.0, 25.0))
            ).clicked() {
                self.language = Language::English;
                result = Some(MenuResult::LanguageChanged(Language::English));
            }
        });
        
        result
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
                Some(ctx.load_texture("logo", image_data, TextureOptions::LINEAR))
            },
            Err(_) => None
        }
    }
}
