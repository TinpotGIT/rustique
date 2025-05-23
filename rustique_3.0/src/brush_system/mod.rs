use eframe::egui;
use egui::{Color32, Vec2, Pos2, Rect, TextureHandle, Stroke};
use std::f32::consts::PI;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Les différents types de pinceaux selon l'image fournie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrushType {
    Round,   // Pinceau rond pour des traits précis et fins
    Flat,    // Pinceau plat pour des traits larges et uniformes
    Bright,  // Similaire au plat mais plus court, pour des traits plus structurés
    Filbert, // Pinceau avec une pointe arrondie, pour des transitions douces
    Fan,     // Pinceau en éventail pour créer des textures et effets spéciaux
    Angle,   // Pinceau biseauté pour des traits variés selon l'angle
    Mop,     // Pinceau large et doux pour des zones diffuses
    Rigger,  // Pinceau très fin pour les détails précis
    Custom,  // Pinceau personnalisé avec texture importée
}

impl BrushType {
    // Obtenir le nom localisé du pinceau
    pub fn get_name(&self, language: crate::localization::Language) -> String {
        use crate::localization::get_text;
        match self {
            BrushType::Round => get_text("brush_round", language),
            BrushType::Flat => get_text("brush_flat", language),
            BrushType::Bright => get_text("brush_bright", language),
            BrushType::Filbert => get_text("brush_filbert", language),
            BrushType::Fan => get_text("brush_fan", language),
            BrushType::Angle => get_text("brush_angle", language),
            BrushType::Mop => get_text("brush_mop", language),
            BrushType::Rigger => get_text("brush_rigger", language),
            BrushType::Custom => get_text("brush_custom", language),
        }
    }
    
    // Obtenir l'index par défaut du pinceau
    pub fn get_default_index(&self) -> usize {
        match self {
            BrushType::Round => 0,
            BrushType::Flat => 1,
            BrushType::Bright => 2,
            BrushType::Filbert => 3,
            BrushType::Fan => 4,
            BrushType::Angle => 5,
            BrushType::Mop => 6,
            BrushType::Rigger => 7,
            BrushType::Custom => 8,
        }
    }
    
    // Liste de tous les types de pinceaux standard (hors Custom)
    pub fn all_types() -> Vec<BrushType> {
        vec![
            BrushType::Round,
            BrushType::Flat,
            BrushType::Bright,
            BrushType::Filbert,
            BrushType::Fan,
            BrushType::Angle,
            BrushType::Mop,
            BrushType::Rigger,
        ]
    }
}

// Propriétés d'un pinceau spécifique
#[derive(Clone, Serialize, Deserialize)]
pub struct BrushProperties {
    // Type de pinceau
    pub brush_type: BrushType,
    
    // Taille de base du pinceau
    pub size: f32,
    
    // Facteur d'étirement (ratio largeur/hauteur)
    pub stretch_factor: f32,
    
    // Sensibilité à l'angle (0 = pas d'effet, 1 = effet maximal)
    pub angle_sensitivity: f32,
    
    // Sensibilité à la pression (simulation)
    pub pressure_sensitivity: f32,
    
    // Texture personnalisée (chemin de l'image)
    pub texture_path: Option<String>,
    
    // Modes de fusion (normal, additif, soustractif, etc.)
    pub blend_mode: BlendMode,
    
    // Espacement entre les points lors du tracé
    pub spacing: f32,
    
    // Paramètres de bruit/texture
    pub texture_strength: f32,
    
    // Dureté des bords (0.0 = très doux, 1.0 = dur)
    pub hardness: f32,
    
    // Rotation de base du pinceau (en radians)
    pub base_rotation: f32,
}

// Mode de fusion pour le pinceau
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BlendMode {
    Normal,
    Add,
    Multiply,
    Screen,
    Overlay,
}

impl Default for BrushProperties {
    fn default() -> Self {
        Self {
            brush_type: BrushType::Round,
            size: 10.0,
            stretch_factor: 1.0,
            angle_sensitivity: 0.5,
            pressure_sensitivity: 0.5,
            texture_path: None,
            blend_mode: BlendMode::Normal,
            spacing: 0.25,
            texture_strength: 0.0,
            hardness: 0.8,
            base_rotation: 0.0,
        }
    }
}

impl BrushProperties {
    // Créer un pinceau avec des paramètres prédéfinis pour un type spécifique
    pub fn from_type(brush_type: BrushType) -> Self {
        let mut properties = Self::default();
        properties.brush_type = brush_type;
        
        // Définir les propriétés par défaut selon le type
        match brush_type {
            BrushType::Round => {
                properties.stretch_factor = 1.0;  // Complètement rond
                properties.angle_sensitivity = 0.1; // Peu sensible à l'angle
                properties.hardness = 0.8;
                properties.spacing = 0.2;
            },
            BrushType::Flat => {
                properties.stretch_factor = 4.0;  // 4 fois plus large que haut
                properties.angle_sensitivity = 0.9; // Très sensible à l'angle
                properties.hardness = 0.7;
                properties.spacing = 0.25;
            },
            BrushType::Bright => {
                properties.stretch_factor = 3.0;  // 3 fois plus large que haut
                properties.angle_sensitivity = 0.8;
                properties.hardness = 0.9;
                properties.spacing = 0.3;
            },
            BrushType::Filbert => {
                properties.stretch_factor = 2.5;
                properties.angle_sensitivity = 0.7;
                properties.hardness = 0.6;
                properties.spacing = 0.2;
            },
            BrushType::Fan => {
                properties.stretch_factor = 3.0;
                properties.angle_sensitivity = 0.95; // Très forte sensibilité à l'angle
                properties.texture_strength = 0.5;   // Plus de texture
                properties.hardness = 0.5;
                properties.spacing = 0.3;
            },
            BrushType::Angle => {
                properties.stretch_factor = 2.0;
                properties.angle_sensitivity = 0.9;
                properties.base_rotation = PI / 4.0; // 45 degrés
                properties.hardness = 0.7;
                properties.spacing = 0.25;
            },
            BrushType::Mop => {
                properties.stretch_factor = 1.2;
                properties.angle_sensitivity = 0.2; // Peu sensible à l'angle
                properties.hardness = 0.3;          // Bords très doux
                properties.spacing = 0.15;
            },
            BrushType::Rigger => {
                properties.stretch_factor = 0.5;    // Plus haut que large
                properties.angle_sensitivity = 0.3;
                properties.hardness = 0.95;         // Bords durs
                properties.spacing = 0.1;
            },
            BrushType::Custom => {
                // Les valeurs par défaut sont bonnes pour Custom
            },
        }
        
        properties
    }
    
    // Obtenir un ensemble de préréglages pour un type de pinceau spécifique
    pub fn get_presets(brush_type: BrushType) -> Vec<Self> {
        let base = Self::from_type(brush_type);
        
        match brush_type {
            BrushType::Round => {
                // Préréglages pour pinceaux ronds
                vec![
                    base.clone(),  // Pinceau standard
                    {
                        // Pinceau rond doux
                        let mut p = base.clone();
                        p.hardness = 0.4;
                        p.texture_strength = 0.1;
                        p
                    },
                    {
                        // Pinceau rond texturé
                        let mut p = base.clone();
                        p.texture_strength = 0.3;
                        p
                    },
                ]
            },
            BrushType::Flat => {
                // Préréglages pour pinceaux plats
                vec![
                    base.clone(),  // Pinceau standard
                    {
                        // Pinceau très plat
                        let mut p = base.clone();
                        p.stretch_factor = 6.0;
                        p
                    },
                    {
                        // Pinceau plat doux
                        let mut p = base.clone();
                        p.hardness = 0.5;
                        p
                    },
                ]
            },
            _ => vec![base], // Préréglage unique pour les autres types
        }
    }
    
    // Calculer la forme du pinceau en fonction de l'angle
    pub fn calculate_shape(&self, angle: f32, size: f32) -> (f32, f32) {
        // Angle effectif avec la rotation de base
        let effective_angle = angle + self.base_rotation;
        
        // Calculer le facteur d'étirement basé sur la sensibilité à l'angle
        let stretch_effect = 1.0 + (self.stretch_factor - 1.0) * self.angle_sensitivity;
        
        // Calculer largeur et hauteur en fonction de l'angle
        let angle_factor = effective_angle.sin().abs() * effective_angle.cos().abs() * 2.0;
        let width_factor = 1.0 + (stretch_effect - 1.0) * angle_factor;
        let height_factor = 1.0 / width_factor;
        
        // Appliquer à la taille de base
        let base_size = size * self.size;
        let width = base_size * width_factor;
        let height = base_size * height_factor * self.stretch_factor;
        
        (width, height)
    }
}

// Gestionnaire de pinceaux qui gère tous les pinceaux disponibles
pub struct BrushManager {
    // Liste des pinceaux disponibles
    pub brushes: Vec<BrushProperties>,
    
    // Index du pinceau actif
    pub active_brush_index: usize,
    
    // Textures chargées pour les pinceaux personnalisés (path -> texture)
    texture_cache: HashMap<String, TextureHandle>,
    
    // Angle actuel du pinceau (en radians)
    pub current_angle: f32,
    
    // Dernière position de dessin pour calculer l'angle
    pub last_position: Option<(f32, f32)>,
    
    // Taille du pinceau actuellement sélectionné
    pub current_size: f32,
}

impl Default for BrushManager {
    fn default() -> Self {
        // Créer des pinceaux par défaut
        let mut brushes = Vec::new();
        
        // Ajouter tous les types de pinceaux standards
        for brush_type in BrushType::all_types() {
            brushes.push(BrushProperties::from_type(brush_type));
        }
        
        Self {
            brushes,
            active_brush_index: 0,
            texture_cache: HashMap::new(),
            current_angle: 0.0,
            last_position: None,
            current_size: 3.0,
        }
    }
}

impl BrushManager {
    // Créer un nouveau gestionnaire de pinceaux
    pub fn new() -> Self {
        Self::default()
    }
    
    // Obtenir les propriétés du pinceau actif
    pub fn active_brush(&self) -> &BrushProperties {
        &self.brushes[self.active_brush_index]
    }
    
    // Obtenir les propriétés du pinceau actif de manière mutable
    pub fn active_brush_mut(&mut self) -> &mut BrushProperties {
        &mut self.brushes[self.active_brush_index]
    }
    
    // Changer le pinceau actif
    pub fn set_active_brush(&mut self, index: usize) {
        if index < self.brushes.len() {
            self.active_brush_index = index;
        }
    }
    
    // Ajouter un nouveau pinceau personnalisé
    pub fn add_custom_brush(&mut self, properties: BrushProperties) -> usize {
        self.brushes.push(properties);
        self.brushes.len() - 1
    }
    
    // Supprimer un pinceau
    pub fn remove_brush(&mut self, index: usize) {
        if index < self.brushes.len() && self.brushes.len() > 1 {
            self.brushes.remove(index);
            if self.active_brush_index >= self.brushes.len() {
                self.active_brush_index = self.brushes.len() - 1;
            }
        }
    }
    
    // Charger une texture pour un pinceau depuis un chemin
    pub fn load_brush_texture(&mut self, path: &str, ctx: &egui::Context) -> Option<TextureHandle> {
        // Vérifier si la texture est déjà en cache
        if let Some(texture) = self.texture_cache.get(path) {
            return Some(texture.clone());
        }
        
        // Essayer de charger l'image
        if let Ok(image) = image::open(path) {
            // Convertir en RGBA
            let image_rgba = image.to_rgba8();
            // Créer une texture egui
            let size = [image_rgba.width() as _, image_rgba.height() as _];
            let image_data = egui::ColorImage::from_rgba_unmultiplied(
                size,
                image_rgba.as_flat_samples().as_slice()
            );
            
            // Charger dans le contexte egui
            let texture = ctx.load_texture(
                path,
                image_data,
                egui::TextureOptions::LINEAR
            );
            
            // Ajouter au cache
            self.texture_cache.insert(path.to_string(), texture.clone());
            
            Some(texture)
        } else {
            None
        }
    }
    
    // Mettre à jour l'angle du pinceau en fonction du mouvement
    pub fn update_angle(&mut self, x: f32, y: f32) {
        if let Some((prev_x, prev_y)) = self.last_position {
            let dx = x - prev_x;
            let dy = y - prev_y;
            
            // Calculer l'angle uniquement si le déplacement est significatif
            if dx * dx + dy * dy > 1.0 {
                // Calculer l'angle du mouvement
                self.current_angle = dy.atan2(dx);
            }
        }
        
        // Mettre à jour la dernière position
        self.last_position = Some((x, y));
    }
    
    // Réinitialiser la dernière position (fin d'un trait)
    pub fn reset_position(&mut self) {
        self.last_position = None;
    }
    
    // Générer un masque de pinceau en fonction des propriétés actuelles et de l'angle
    pub fn generate_brush_mask(&self, size: usize) -> Vec<f32> {
        let active = self.active_brush();
        
        // Taille de base du masque
        let mut mask = vec![0.0; size * size];
        let center = size as f32 / 2.0;
        let radius = center;
        
        // Angle effectif avec la rotation de base
        let effective_angle = self.current_angle + active.base_rotation;
        let cos_a = effective_angle.cos();
        let sin_a = effective_angle.sin();
        
        // Calculer le masque pour chaque pixel selon le type de pinceau
        for y in 0..size {
            for x in 0..size {
                // Position relative au centre
                let rx = (x as f32 - center) / radius;
                let ry = (y as f32 - center) / radius;
                
                let mut value = 0.0;
                
                match active.brush_type {
                    BrushType::Round => {
                        // Pinceau rond classique
                        let dist = (rx * rx + ry * ry).sqrt();
                        value = (1.0 - dist).max(0.0);
                    },
                    
                    BrushType::Flat | BrushType::Bright => {
                        // Pinceau rectangulaire avec rotation
                        let rx_rot = rx * cos_a - ry * sin_a;
                        let ry_rot = rx * sin_a + ry * cos_a;
                        
                        let width_scale = if active.brush_type == BrushType::Flat { 0.25 } else { 0.35 };
                        let height_scale = 1.0;
                        
                        if rx_rot.abs() <= width_scale && ry_rot.abs() <= height_scale {
                            let edge_x = 1.0 - (rx_rot.abs() / width_scale);
                            let edge_y = 1.0 - (ry_rot.abs() / height_scale);
                            value = edge_x.min(edge_y);
                        }
                    },
                    
                    BrushType::Filbert => {
                        // Forme ovale arrondie avec rotation
                        let rx_rot = rx * cos_a - ry * sin_a;
                        let ry_rot = rx * sin_a + ry * cos_a;
                        
                        let ellipse_dist = (rx_rot * rx_rot) / (0.6 * 0.6) + (ry_rot * ry_rot) / (1.0 * 1.0);
                        value = (1.0 - ellipse_dist.sqrt()).max(0.0);
                    },
                    
                    BrushType::Fan => {
                        // Forme en éventail
                        let angle_from_center = ry.atan2(rx);
                        let dist = (rx * rx + ry * ry).sqrt();
                        
                        // Créer des "rayons" en éventail
                        let fan_angle = (angle_from_center + PI) * 5.0; // 5 rayons
                        let ray_intensity = (fan_angle.sin().abs() * 2.0).min(1.0);
                        
                        if dist <= 1.0 && ray_intensity > 0.3 {
                            value = (1.0 - dist) * ray_intensity;
                        }
                    },
                    
                    BrushType::Angle => {
                        // Pinceau biseauté avec angle
                        let rx_rot = rx * cos_a - ry * sin_a;
                        let ry_rot = rx * sin_a + ry * cos_a;
                        
                        // Forme triangulaire/biseautée
                        if rx_rot >= -0.8 && rx_rot <= 0.4 && ry_rot.abs() <= 0.6 {
                            let triangle_factor = (0.4 - rx_rot) / 1.2;
                            let edge_y = 1.0 - (ry_rot.abs() / 0.6);
                            value = triangle_factor.min(edge_y).max(0.0);
                        }
                    },
                    
                    BrushType::Mop => {
                        // Large pinceau diffus
                        let dist = (rx * rx + ry * ry).sqrt();
                        value = (1.2 - dist).max(0.0);
                        // Adoucir les bords plus que les autres pinceaux
                        value = value.powf(0.5);
                    },
                    
                    BrushType::Rigger => {
                        // Pinceau très fin
                        let rx_rot = rx * cos_a - ry * sin_a;
                        let ry_rot = rx * sin_a + ry * cos_a;
                        
                        if rx_rot.abs() <= 0.1 && ry_rot.abs() <= 0.8 {
                            let edge_x = 1.0 - (rx_rot.abs() / 0.1);
                            let edge_y = 1.0 - (ry_rot.abs() / 0.8);
                            value = edge_x.min(edge_y);
                        }
                    },
                    
                    BrushType::Custom => {
                        // Comportement personnalisable - pour l'instant comme un rond modifié
                        let dist = (rx * rx + ry * ry).sqrt();
                        value = (1.0 - dist).max(0.0);
                    },
                }
                
                // Appliquer la dureté pour tous les types
                if value > 0.0 {
                    value = value.powf(1.0 / (active.hardness.max(0.1)));
                }
                
                // Appliquer de la texture/bruit si nécessaire
                if active.texture_strength > 0.0 && value > 0.0 {
                    let noise = ((x as f32 * 0.3).sin() * (y as f32 * 0.3).cos() + 
                                (x as f32 * 0.7).cos() * (y as f32 * 0.5).sin()) * 0.5;
                    let noise_factor = 1.0 - active.texture_strength * noise.abs();
                    value *= noise_factor.max(0.2); // Garder au moins 20% de la valeur
                }
                
                // Appliquer la sensibilité à l'angle pour moduler l'intensité
                if active.angle_sensitivity > 0.0 {
                    let angle_factor = 1.0 - active.angle_sensitivity * 0.3 * (effective_angle.sin().abs());
                    value *= angle_factor.max(0.3);
                }
                
                // Stocker la valeur dans le masque
                mask[y * size + x] = value.clamp(0.0, 1.0);
            }
        }
        
        mask
    }
    
    // Dessiner un point avec le pinceau actif
    pub fn draw_point(&mut self, x: i32, y: i32, color: Color32, record_change: &mut dyn FnMut(usize, usize, Option<Color32>)) {
        let active = self.active_brush();
        let size = self.current_size as usize * 2 + 1; // S'assurer que la taille est impaire
        
        // Mettre à jour l'angle si nécessaire
        self.update_angle(x as f32, y as f32);
        
        // Générer le masque du pinceau
        let mask = self.generate_brush_mask(size);
        
        // Appliquer le masque centré sur (x, y)
        let center = size as i32 / 2;
        for dy in 0..size as i32 {
            for dx in 0..size as i32 {
                let nx = x + dx - center;
                let ny = y + dy - center;
                
                // Récupérer la valeur du masque
                let mask_value = mask[(dy as usize) * size + (dx as usize)];
                
                if mask_value > 0.0 && nx >= 0 && ny >= 0 {
                    // Calculer la couleur avec transparence basée sur le masque
                    let alpha = (color.a() as f32 * mask_value) as u8;
                    let new_color = if alpha > 0 {
                        Some(Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha))
                    } else {
                        None
                    };
                    
                    // Appliquer le changement
                    record_change(nx as usize, ny as usize, new_color);
                }
            }
        }
    }
    
    // Dessiner une ligne avec le pinceau actif en utilisant une interpolation entre points
    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), color: Color32, record_change: &mut dyn FnMut(usize, usize, Option<Color32>)) {
        // Calculer les points intermédiaires avec l'algorithme de Bresenham
        let (x0, y0) = start;
        let (x1, y1) = end;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        
        let mut x = x0;
        let mut y = y0;
        
        // Calculer la longueur totale (pour information)
        let _total_length = ((x1 - x0).pow(2) + (y1 - y0).pow(2)) as f32;
        let spacing = (self.active_brush().spacing * self.current_size).max(1.0);
        
        // Points à dessiner
        let mut points = Vec::new();
        points.push((x, y));
        
        // Accumulateur de distance
        let mut accumulated_distance = 0.0;
        
        loop {
            let old_x = x;
            let old_y = y;
            
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x1 { break; }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y1 { break; }
                err += dx;
                y += sy;
            }
            
            // Calculer la distance parcourue
            let segment_length = ((x - old_x).pow(2) + (y - old_y).pow(2)) as f32;
            accumulated_distance += segment_length;
            
            // Ajouter un point si on a dépassé l'espacement
            if accumulated_distance >= spacing {
                points.push((x, y));
                accumulated_distance = 0.0;
            }
        }
        
        // Ajouter le point final s'il n'est pas déjà ajouté
        if points.last() != Some(&(x1, y1)) {
            points.push((x1, y1));
        }
        
        // Dessiner les points
        for &(px, py) in &points {
            self.draw_point(px, py, color, record_change);
        }
    }
    
    // Afficher une grille de sélection visuelle des pinceaux
    pub fn brush_selector_grid(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context, language: crate::localization::Language) -> bool {
        use crate::localization::get_text;
        let mut changed = false;
        
        ui.heading(get_text("select_brush", language));
        ui.separator();
        
        // Définir la taille des cellules et le nombre de colonnes
        let cell_size = Vec2::new(64.0, 64.0);
        let margin = 8.0;
        let total_size = cell_size + Vec2::splat(margin * 2.0);
        let available_width = ui.available_width();
        let columns = (available_width / total_size.x).floor().max(1.0) as usize;
        
        // Créer une grille pour les pinceaux standard
        egui::Grid::new("brush_selector_grid")
            .spacing([margin, margin])
            .min_col_width(cell_size.x)
            .min_row_height(cell_size.y)
            .show(ui, |ui| {
                let mut col = 0;
                
                // Créer une copie des types de pinceaux pour éviter les problèmes de borrowing
                let brush_types: Vec<(usize, BrushType)> = self.brushes
                    .iter()
                    .enumerate()
                    .map(|(i, brush)| (i, brush.brush_type))
                    .collect();
                let active_index = self.active_brush_index;
                
                for (i, brush_type) in brush_types {
                    // Allouer l'espace pour le bouton
                    let (rect, response) = ui.allocate_exact_size(cell_size, egui::Sense::click());
                    
                    // Dessiner le cadre
                    let is_active = i == active_index;
                    ui.painter().rect(
                        rect,
                        4.0,
                        if is_active {
                            ui.style().visuals.selection.bg_fill
                        } else if response.hovered() {
                            ui.style().visuals.widgets.hovered.bg_fill
                        } else {
                            ui.style().visuals.widgets.inactive.bg_fill
                        },
                        if is_active {
                            egui::Stroke::new(2.0, ui.style().visuals.selection.stroke.color)
                        } else {
                            egui::Stroke::NONE
                        }
                    );
                    
                    // Dessiner l'aperçu du pinceau
                    let preview_size = Vec2::new(48.0, 48.0);
                    let preview_rect = Rect::from_center_size(
                        Pos2::new(rect.center().x, rect.min.y + preview_size.y * 0.5 + 8.0),
                        preview_size
                    );
                    
                    self.draw_brush_preview(ui, preview_rect, &brush_type);
                    
                    // Afficher le nom du pinceau
                    let text_pos = Pos2::new(rect.center().x, rect.max.y - 12.0);
                    ui.painter().text(
                        text_pos,
                        egui::Align2::CENTER_CENTER,
                        brush_type.get_name(language),
                        egui::FontId::proportional(11.0),
                        ui.style().visuals.text_color(),
                    );
                    
                    // Gérer le clic
                    if response.clicked() {
                        self.active_brush_index = i;
                        changed = true;
                    }
                    
                    // Passer à la ligne suivante si nécessaire
                    col += 1;
                    if col >= columns {
                        col = 0;
                        ui.end_row();
                    }
                }
                
                // Assurer que la dernière ligne est complète pour l'alignement
                if col > 0 && col < columns {
                    for _ in col..columns {
                        ui.add_space(cell_size.x);
                    }
                }
            });
        
        // Bouton pour ajouter un pinceau personnalisé
        ui.add_space(10.0);
        if ui.button(format!("+ {}", get_text("brush_custom", language))).clicked() {
            let new_brush = BrushProperties::from_type(BrushType::Custom);
            let index = self.add_custom_brush(new_brush);
            self.set_active_brush(index);
            changed = true;
        }
        
        changed
    }
    
    // Dessiner un aperçu du pinceau dans un rectangle donné
    fn draw_brush_preview(&self, ui: &mut egui::Ui, rect: egui::Rect, brush_type: &BrushType) {
        let painter = ui.painter();
        
        // Fond blanc
        painter.rect_filled(rect, 4.0, Color32::from_gray(240));
        
        match brush_type {
            BrushType::Round => {
                // Cercle pour pinceau rond
                let center = rect.center();
                let radius = rect.width() * 0.25;
                painter.circle_filled(center, radius, Color32::BLACK);
            },
            BrushType::Flat => {
                // Rectangle horizontal très plat pour pinceau plat
                let center = rect.center();
                let width = rect.width() * 0.7;
                let height = rect.height() * 0.1;
                let rect = Rect::from_center_size(center, Vec2::new(width, height));
                painter.rect_filled(rect, 1.0, Color32::BLACK);
            },
            BrushType::Bright => {
                // Rectangle horizontal plus court et plus épais que Flat
                let center = rect.center();
                let width = rect.width() * 0.5;
                let height = rect.height() * 0.15;
                let rect = Rect::from_center_size(center, Vec2::new(width, height));
                painter.rect_filled(rect, 1.0, Color32::BLACK);
            },
            BrushType::Filbert => {
                // Forme ovale arrondie
                let center = rect.center();
                let width = rect.width() * 0.5;
                let height = rect.height() * 0.3;
                let rect = Rect::from_center_size(center, Vec2::new(width, height));
                painter.rect_filled(rect, 12.0, Color32::BLACK);
            },
            BrushType::Fan => {
                // Lignes en éventail plus réalistes
                let center = rect.center();
                let radius = rect.width() * 0.3;
                let angles = [-25.0, -12.0, 0.0, 12.0, 25.0]; // en degrés
                
                for (i, angle_deg) in angles.iter().enumerate() {
                    let angle = (*angle_deg as f32).to_radians();
                    let length = radius * (0.8 + 0.2 * (2.0 - i as f32).abs() / 2.0); // Longueurs variables
                    let dir_x = angle.sin() * length;
                    let dir_y = -angle.cos() * length;
                    
                    let stroke_width = if i == 2 { 2.5 } else { 1.5 }; // Centre plus épais
                    
                    painter.line_segment(
                        [center, Pos2::new(center.x + dir_x, center.y + dir_y)],
                        Stroke::new(stroke_width, Color32::BLACK)
                    );
                }
            },
            BrushType::Angle => {
                // Forme biseautée plus réaliste
                let center = rect.center();
                let size = rect.width() * 0.15;
                
                // Créer une forme triangulaire inclinée
                let points = vec![
                    Pos2::new(center.x - size * 2.0, center.y),
                    Pos2::new(center.x + size * 0.8, center.y - size * 1.5),
                    Pos2::new(center.x + size * 0.8, center.y + size * 1.5),
                ];
                
                painter.add(egui::Shape::convex_polygon(
                    points,
                    Color32::BLACK,
                    Stroke::NONE,
                ));
            },
            BrushType::Mop => {
                // Large forme diffuse avec dégradé
                let center = rect.center();
                let radius = rect.width() * 0.35;
                
                // Dessiner plusieurs cercles concentriques pour l'effet flou
                for r in (1..=8).rev() {
                    let alpha = (r as f32 / 8.0 * 180.0) as u8;
                    let color = Color32::from_rgba_unmultiplied(0, 0, 0, alpha);
                    let r_scaled = radius * (r as f32 / 8.0);
                    painter.circle_filled(center, r_scaled, color);
                }
            },
            BrushType::Rigger => {
                // Ligne très fine et longue
                let center = rect.center();
                let height = rect.height() * 0.8;
                
                painter.line_segment(
                    [
                        Pos2::new(center.x, center.y - height * 0.5),
                        Pos2::new(center.x, center.y + height * 0.5)
                    ],
                    Stroke::new(1.5, Color32::BLACK)
                );
            },
            BrushType::Custom => {
                // Motif de texture personnalisée
                let center = rect.center();
                let size = rect.width() * 0.12;
                
                // Dessiner un motif en damier pour représenter la texture
                for i in -1..=1 {
                    for j in -1..=1 {
                        if (i + j) % 2 == 0 {
                            let x = center.x + i as f32 * size * 1.2;
                            let y = center.y + j as f32 * size * 1.2;
                            let small_rect = Rect::from_center_size(
                                Pos2::new(x, y),
                                Vec2::splat(size)
                            );
                            painter.rect_filled(small_rect, 2.0, Color32::from_gray(60));
                        }
                    }
                }
                
                // Ajouter un petit symbole au centre
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    "T",
                    egui::FontId::proportional(12.0),
                    Color32::BLACK,
                );
            }
        }
    }
}
