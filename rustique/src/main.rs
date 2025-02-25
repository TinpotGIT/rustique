mod main_menu;

use eframe::egui;
use egui::{Color32, TextureHandle, TextureOptions, Rect, Pos2, Vec2, Stroke};
use image::{ImageBuffer, Rgba};
use std::collections::VecDeque;
use rfd::FileDialog;
use std::time::{Duration, Instant};
use std::path::Path;
use std::fs;
use std::io::Write;
use serde::{Serialize, Deserialize};

use main_menu::MainMenu;

// Constants
const MAX_UNDO_STEPS: usize = 20;
const SAVE_STATE_DELAY: Duration = Duration::from_millis(300);
const CHECKERBOARD_SIZE: usize = 8;
const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const MAX_SAVED_COLORS: usize = 16;

// Enum to represent different tools
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
enum Tool {
    Brush,
    Eraser,
    PaintBucket,
    ColorPicker,
    Line,
}

// Enum to represent the current state of the application
enum AppState {
    MainMenu(MainMenu),
    Canvas(PaintApp),
}

// Layer structure for storing each canvas layer (sans serde)
#[derive(Clone, PartialEq)]
struct Layer {
    name: String,
    data: Vec<Option<Color32>>,
    visible: bool,
}

// Layer structure for serialization
#[derive(Serialize, Deserialize)]
struct LayerData {
    name: String,
    data: Vec<Option<[u8; 4]>>,
    visible: bool,
}

// Structure for saving and loading .rustiq files
#[derive(Serialize, Deserialize)]
struct RustiqueFile {
    width: usize,
    height: usize,
    layers: Vec<LayerData>,
    active_layer_index: usize,
    primary_color: [u8; 4],
    secondary_color: [u8; 4],
    saved_colors: Vec<[u8; 4]>,
    brush_size: i32,
    eraser_size: i32,
}

// Optimized canvas state structure with layers
#[derive(Clone)]
struct CanvasState {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
    active_layer_index: usize,
}

impl CanvasState {
    fn new(width: usize, height: usize) -> Self {
        let default_layer = Layer {
            name: "Background".to_string(),
            data: vec![None; width * height],
            visible: true,
        };
        
        Self {
            width,
            height,
            layers: vec![default_layer],
            active_layer_index: 0,
        }
    }
    
    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<Color32> {
        if x < self.width && y < self.height {
            // Iterate through layers from top to bottom
            for layer_index in (0..self.layers.len()).rev() {
                let layer = &self.layers[layer_index];
                if layer.visible {
                    let idx = y * self.width + x;
                    if let Some(color) = layer.data[idx] {
                        return Some(color);
                    }
                }
            }
        }
        None
    }
    
    #[inline]
    fn get_from_active_layer(&self, x: usize, y: usize) -> Option<Color32> {
        if x < self.width && y < self.height && self.active_layer_index < self.layers.len() {
            let idx = y * self.width + x;
            self.layers[self.active_layer_index].data[idx]
        } else {
            None
        }
    }
    
    #[inline]
    fn set(&mut self, x: usize, y: usize, color: Option<Color32>) {
        if x < self.width && y < self.height && self.active_layer_index < self.layers.len() {
            let idx = y * self.width + x;
            self.layers[self.active_layer_index].data[idx] = color;
        }
    }
    
    #[inline]
    fn is_visible(&self, layer_index: usize) -> bool {
        layer_index < self.layers.len() && self.layers[layer_index].visible
    }
}

// Store changes for efficient undo/redo
#[derive(Clone)]
struct CanvasChange {
    x: usize,
    y: usize,
    layer_index: usize,
    old_color: Option<Color32>,
    new_color: Option<Color32>,
}

// Dialog for asking to save before quitting
enum SaveDialog {
    Hidden,
    AskingSave {
        return_to_menu: bool,
    },
}

// Main struct for the paint application
struct PaintApp {
    current_state: CanvasState,
    undo_stack: Vec<Vec<CanvasChange>>,
    redo_stack: Vec<Vec<CanvasChange>>,
    current_changes: Vec<CanvasChange>,
    current_tool: Tool,
    primary_color: Color32,
    secondary_color: Color32,
    saved_colors: Vec<Color32>,
    brush_size: i32,
    eraser_size: i32,
    last_position: Option<(i32, i32)>,
    is_drawing: bool,
    last_action_time: Instant,
    texture: Option<TextureHandle>,
    texture_dirty: bool,
    zoom: f32,
    pan: Vec2,
    line_start: Option<(i32, i32)>,
    line_end: Option<(i32, i32)>,
    is_drawing_line: bool,
    has_unsaved_changes: bool,
    last_save_path: Option<String>,
    save_dialog: SaveDialog,
}

impl PaintApp {
    // Initialize a new PaintApp
    fn new(width: u32, height: u32) -> Self {
        let initial_state = CanvasState::new(width as usize, height as usize);
        Self {
            current_state: initial_state,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_changes: Vec::new(),
            current_tool: Tool::Brush,
            primary_color: Color32::BLACK,
            secondary_color: Color32::WHITE,
            saved_colors: Vec::new(),
            brush_size: 3,
            eraser_size: 3,
            last_position: None,
            is_drawing: false,
            last_action_time: Instant::now(),
            texture: None,
            texture_dirty: true,
            zoom: 1.0,
            pan: Vec2::ZERO,
            line_start: None,
            line_end: None,
            is_drawing_line: false,
            has_unsaved_changes: false,
            last_save_path: None,
            save_dialog: SaveDialog::Hidden,
        }
    }

    // Create a PaintApp from a .rustiq file
    fn from_rustiq_file(file: RustiqueFile) -> Self {
        let mut canvas = CanvasState {
            width: file.width,
            height: file.height,
            layers: Vec::with_capacity(file.layers.len()),
            active_layer_index: file.active_layer_index,
        };
        
        // Convert the saved layers to Canvas layers
        for layer_data in file.layers {
            let mut layer = Layer {
                name: layer_data.name,
                data: Vec::with_capacity(layer_data.data.len()),
                visible: layer_data.visible,
            };
            
            for pixel_opt in layer_data.data {
                if let Some(rgba) = pixel_opt {
                    layer.data.push(Some(Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])));
                } else {
                    layer.data.push(None);
                }
            }
            
            canvas.layers.push(layer);
        }
        
        let primary_color = Color32::from_rgba_unmultiplied(
            file.primary_color[0],
            file.primary_color[1],
            file.primary_color[2],
            file.primary_color[3]
        );
        
        let secondary_color = Color32::from_rgba_unmultiplied(
            file.secondary_color[0],
            file.secondary_color[1],
            file.secondary_color[2],
            file.secondary_color[3]
        );
        
        let mut saved_colors = Vec::with_capacity(file.saved_colors.len());
        for color_data in file.saved_colors {
            saved_colors.push(Color32::from_rgba_unmultiplied(
                color_data[0],
                color_data[1],
                color_data[2],
                color_data[3]
            ));
        }
        
        Self {
            current_state: canvas,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_changes: Vec::new(),
            current_tool: Tool::Brush,
            primary_color,
            secondary_color,
            saved_colors,
            brush_size: file.brush_size,
            eraser_size: file.eraser_size,
            last_position: None,
            is_drawing: false,
            last_action_time: Instant::now(),
            texture: None,
            texture_dirty: true,
            zoom: 1.0,
            pan: Vec2::ZERO,
            line_start: None,
            line_end: None,
            is_drawing_line: false,
            has_unsaved_changes: false,
            last_save_path: None,
            save_dialog: SaveDialog::Hidden,
        }
    }
    
    // Create a PaintApp from a PNG file
    fn from_png_file(path: &str) -> Option<Self> {
        match image::open(path) {
            Ok(img) => {
                let width = img.width() as usize;
                let height = img.height() as usize;
                let mut canvas = CanvasState::new(width, height);
                
                let rgba_img = img.to_rgba8();
                for y in 0..height {
                    for x in 0..width {
                        let pixel = rgba_img.get_pixel(x as u32, y as u32);
                        if pixel[3] > 0 { // Not fully transparent
                            let color = Color32::from_rgba_unmultiplied(pixel[0], pixel[1], pixel[2], pixel[3]);
                            canvas.set(x, y, Some(color));
                        }
                    }
                }
                
                Some(Self {
                    current_state: canvas,
                    undo_stack: Vec::new(),
                    redo_stack: Vec::new(),
                    current_changes: Vec::new(),
                    current_tool: Tool::Brush,
                    primary_color: Color32::BLACK,
                    secondary_color: Color32::WHITE,
                    saved_colors: Vec::new(),
                    brush_size: 3,
                    eraser_size: 3,
                    last_position: None,
                    is_drawing: false,
                    last_action_time: Instant::now(),
                    texture: None,
                    texture_dirty: true,
                    zoom: 1.0,
                    pan: Vec2::ZERO,
                    line_start: None,
                    line_end: None,
                    is_drawing_line: false,
                    has_unsaved_changes: false,
                    last_save_path: None,
                    save_dialog: SaveDialog::Hidden,
                })
            },
            Err(_) => None
        }
    }

    // Save the current image as a .rustiq file
    fn save_as_rustiq(&mut self, path: &str) -> Result<(), String> {
        let mut layers = Vec::with_capacity(self.current_state.layers.len());
        
        for layer in &self.current_state.layers {
            let mut layer_data = Vec::with_capacity(layer.data.len());
            
            for &pixel_opt in &layer.data {
                match pixel_opt {
                    Some(color) => {
                        layer_data.push(Some([color.r(), color.g(), color.b(), color.a()]));
                    },
                    None => {
                        layer_data.push(None);
                    }
                }
            }
            
            layers.push(LayerData {
                name: layer.name.clone(),
                data: layer_data,
                visible: layer.visible,
            });
        }
        
        let mut saved_colors = Vec::with_capacity(self.saved_colors.len());
        for &color in &self.saved_colors {
            saved_colors.push([color.r(), color.g(), color.b(), color.a()]);
        }
        
        let rustiq_file = RustiqueFile {
            width: self.current_state.width,
            height: self.current_state.height,
            layers,
            active_layer_index: self.current_state.active_layer_index,
            primary_color: [self.primary_color.r(), self.primary_color.g(), self.primary_color.b(), self.primary_color.a()],
            secondary_color: [self.secondary_color.r(), self.secondary_color.g(), self.secondary_color.b(), self.secondary_color.a()],
            saved_colors,
            brush_size: self.brush_size,
            eraser_size: self.eraser_size,
        };
        
        // Sérialiser avec gestion d'erreur
        let json = match serde_json::to_string(&rustiq_file) {
            Ok(json) => json,
            Err(e) => return Err(format!("Erreur de sérialisation: {}", e)),
        };
        
        // Écrire avec gestion d'erreur
        match fs::File::create(path) {
            Ok(mut file) => {
                match file.write_all(json.as_bytes()) {
                    Ok(_) => {
                        self.has_unsaved_changes = false;
                        self.last_save_path = Some(path.to_string());
                        Ok(())
                    },
                    Err(e) => Err(format!("Erreur d'écriture: {}", e)),
                }
            },
            Err(e) => Err(format!("Erreur de création du fichier: {}", e)),
        }
    }
    
    // Save the current canvas using the last save path
    fn quick_save(&mut self) -> Result<(), String> {
        if let Some(_path) = &self.last_save_path {
            let path_clone = self.last_save_path.as_ref().unwrap().clone();
            if path_clone.ends_with(".png") {
                self.save_as_png(&path_clone)
            } else if path_clone.ends_with(".rustiq") {
                self.save_as_rustiq(&path_clone)
            } else {
                Err("Format de fichier non pris en charge".to_string())
            }
        } else {
            Err("Aucun chemin de sauvegarde précédent".to_string())
        }
    }

    // Layer management functions
    fn add_layer(&mut self, name: String) {
        self.current_state.layers.push(Layer {
            name,
            data: vec![None; self.current_state.width * self.current_state.height],
            visible: true,
        });
        self.current_state.active_layer_index = self.current_state.layers.len() - 1;
        self.texture_dirty = true;
        self.has_unsaved_changes = true;
    }
    
    fn remove_layer(&mut self, index: usize) {
        if self.current_state.layers.len() > 1 && index < self.current_state.layers.len() {
            self.current_state.layers.remove(index);
            if self.current_state.active_layer_index >= self.current_state.layers.len() {
                self.current_state.active_layer_index = self.current_state.layers.len() - 1;
            }
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }
    
    fn move_layer_up(&mut self, index: usize) {
        if index > 0 && index < self.current_state.layers.len() {
            self.current_state.layers.swap(index, index - 1);
            if self.current_state.active_layer_index == index {
                self.current_state.active_layer_index -= 1;
            } else if self.current_state.active_layer_index == index - 1 {
                self.current_state.active_layer_index += 1;
            }
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }
    
    fn move_layer_down(&mut self, index: usize) {
        if index < self.current_state.layers.len() - 1 {
            self.current_state.layers.swap(index, index + 1);
            if self.current_state.active_layer_index == index {
                self.current_state.active_layer_index += 1;
            } else if self.current_state.active_layer_index == index + 1 {
                self.current_state.active_layer_index -= 1;
            }
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }
    
    fn toggle_layer_visibility(&mut self, index: usize) {
        if index < self.current_state.layers.len() {
            self.current_state.layers[index].visible = !self.current_state.layers[index].visible;
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }
    
    fn set_active_layer(&mut self, index: usize) {
        if index < self.current_state.layers.len() {
            self.current_state.active_layer_index = index;
        }
    }
    
    fn rename_layer(&mut self, index: usize, name: String) {
        if index < self.current_state.layers.len() {
            self.current_state.layers[index].name = name;
            self.has_unsaved_changes = true;
        }
    }
    
    // Color management functions
    fn add_saved_color(&mut self, color: Color32) {
        // Avoid duplicates
        if !self.saved_colors.contains(&color) {
            if self.saved_colors.len() >= MAX_SAVED_COLORS {
                self.saved_colors.remove(0); // Remove oldest color
            }
            self.saved_colors.push(color);
        }
    }
    
    fn remove_saved_color(&mut self, index: usize) {
        if index < self.saved_colors.len() {
            self.saved_colors.remove(index);
        }
    }
    
    fn set_primary_color_from_saved(&mut self, index: usize) {
        if index < self.saved_colors.len() {
            self.primary_color = self.saved_colors[index];
        }
    }
    
    fn set_secondary_color_from_saved(&mut self, index: usize) {
        if index < self.saved_colors.len() {
            self.secondary_color = self.saved_colors[index];
        }
    }

    // Record a pixel change for undo/redo
    fn record_change(&mut self, x: usize, y: usize, new_color: Option<Color32>) {
        if x < self.current_state.width && y < self.current_state.height {
            let old_color = self.current_state.get_from_active_layer(x, y);
            if old_color != new_color {
                self.current_changes.push(CanvasChange {
                    x, 
                    y, 
                    layer_index: self.current_state.active_layer_index,
                    old_color, 
                    new_color
                });
                self.current_state.set(x, y, new_color);
                self.has_unsaved_changes = true;
            }
        }
    }

    // Save the current state for undo functionality
    fn save_state(&mut self) {
        if !self.current_changes.is_empty() && self.last_action_time.elapsed() >= SAVE_STATE_DELAY {
            self.undo_stack.push(std::mem::take(&mut self.current_changes));
            self.current_changes = Vec::new();
            if self.undo_stack.len() > MAX_UNDO_STEPS {
                self.undo_stack.remove(0);
            }
            self.redo_stack.clear();
            self.is_drawing = false;
        }
    }

    // Undo the last action
    fn undo(&mut self) {
        if let Some(changes) = self.undo_stack.pop() {
            let mut redo_changes = Vec::with_capacity(changes.len());
            
            // Apply changes in reverse
            for change in changes.iter().rev() {
                let layer_index_backup = self.current_state.active_layer_index;
                self.current_state.active_layer_index = change.layer_index;
                
                let current_color = self.current_state.get_from_active_layer(change.x, change.y);
                redo_changes.push(CanvasChange {
                    x: change.x,
                    y: change.y,
                    layer_index: change.layer_index,
                    old_color: current_color,
                    new_color: change.old_color,
                });
                
                self.current_state.set(change.x, change.y, change.old_color);
                self.current_state.active_layer_index = layer_index_backup;
            }
            
            self.redo_stack.push(redo_changes);
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }

    // Redo the last undone action
    fn redo(&mut self) {
        if let Some(changes) = self.redo_stack.pop() {
            let mut undo_changes = Vec::with_capacity(changes.len());
            
            // Apply changes in reverse
            for change in changes.iter().rev() {
                let layer_index_backup = self.current_state.active_layer_index;
                self.current_state.active_layer_index = change.layer_index;
                
                let current_color = self.current_state.get_from_active_layer(change.x, change.y);
                undo_changes.push(CanvasChange {
                    x: change.x,
                    y: change.y,
                    layer_index: change.layer_index,
                    old_color: current_color,
                    new_color: change.new_color,
                });
                
                self.current_state.set(change.x, change.y, change.new_color);
                self.current_state.active_layer_index = layer_index_backup;
            }
            
            self.undo_stack.push(undo_changes);
            self.texture_dirty = true;
            self.has_unsaved_changes = true;
        }
    }

    // Draw a line between two points
    fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), color: Color32) {
        let (x0, y0) = start;
        let (x1, y1) = end;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        // For large brush sizes, collect points
        let _size = if self.current_tool == Tool::Eraser { self.eraser_size } else { self.brush_size };
        let mut points = Vec::new();
        
        loop {
            points.push((x, y));
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
        
        // Draw points with the specified color
        let fill_color = if self.current_tool == Tool::Eraser { None } else { Some(color) };
        for &(px, py) in &points {
            self.draw_point_with_color(px, py, fill_color);
        }
        
        self.last_action_time = Instant::now();
        self.texture_dirty = true;
    }

    // Draw a single point with optimized circular brush
    fn draw_point(&mut self, x: i32, y: i32, use_secondary: bool) {
        let color = if use_secondary { self.secondary_color } else { self.primary_color };
        let fill_color = if self.current_tool == Tool::Eraser { None } else { Some(color) };
        self.draw_point_with_color(x, y, fill_color);
    }
    
    // Helper function for drawing a point with a specific color
    fn draw_point_with_color(&mut self, x: i32, y: i32, fill_color: Option<Color32>) {
        let width = self.current_state.width as i32;
        let height = self.current_state.height as i32;
        let size = if self.current_tool == Tool::Eraser { self.eraser_size } else { self.brush_size };
        let size_squared = size * size;
        
        // Ensure active layer is visible before drawing
        if self.current_state.active_layer_index < self.current_state.layers.len() && 
           !self.current_state.layers[self.current_state.active_layer_index].visible {
            return;
        }
        
        // Collect all points that need to be modified
        let mut pixels = Vec::new();
        for dy in -size..=size {
            for dx in -size..=size {
                // Use circle equation dx²+dy² ≤ r² for circular brush
                if dx*dx + dy*dy <= size_squared {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx >= 0 && nx < width && ny >= 0 && ny < height {
                        pixels.push((nx as usize, ny as usize));
                    }
                }
            }
        }
        
        // Process all pixels sequentially
        for (nx, ny) in pixels {
            self.record_change(nx, ny, fill_color);
        }
        
        self.texture_dirty = true;
    }

    // Optimized paint bucket fill
    fn paint_bucket(&mut self, x: usize, y: usize, use_secondary: bool) {
        if x >= self.current_state.width || y >= self.current_state.height {
            return;
        }
        
        // Ensure active layer is visible before filling
        if self.current_state.active_layer_index < self.current_state.layers.len() && 
           !self.current_state.layers[self.current_state.active_layer_index].visible {
            return;
        }
        
        let target_color = self.current_state.get_from_active_layer(x, y);
        let color = if use_secondary { self.secondary_color } else { self.primary_color };
        let fill_color = if self.current_tool == Tool::Eraser {
            None
        } else {
            Some(color)
        };
        
        if target_color == fill_color {
            return;
        }
        
        // Pre-allocate for better performance
        let mut queue = VecDeque::with_capacity(1024);
        let mut visited = vec![false; self.current_state.width * self.current_state.height];
        queue.push_back((x, y));
        
        while let Some((cx, cy)) = queue.pop_front() {
            let idx = cy * self.current_state.width + cx;
            if visited[idx] || self.current_state.get_from_active_layer(cx, cy) != target_color {
                continue;
            }
            
            visited[idx] = true;
            self.record_change(cx, cy, fill_color);
            
            // Add adjacent pixels to queue
            if cx > 0 { queue.push_back((cx - 1, cy)); }
            if cx + 1 < self.current_state.width { queue.push_back((cx + 1, cy)); }
            if cy > 0 { queue.push_back((cx, cy - 1)); }
            if cy + 1 < self.current_state.height { queue.push_back((cx, cy + 1)); }
        }
        
        self.last_action_time = Instant::now();
        self.texture_dirty = true;
    }

    // Pick a color from the canvas
    fn pick_color(&mut self, x: usize, y: usize, use_secondary: bool) {
        if let Some(color) = self.current_state.get(x, y) {
            if use_secondary {
                self.secondary_color = color;
            } else {
                self.primary_color = color;
            }
        }
    }

    // Optimized PNG saving
    fn save_as_png(&mut self, path: &str) -> Result<(), String> {
        let width = self.current_state.width;
        let height = self.current_state.height;
        let mut img = ImageBuffer::new(width as u32, height as u32);
        
        // Make sure path has .png extension
        let path_with_ext = if !path.to_lowercase().ends_with(".png") {
            format!("{}.png", path)
        } else {
            path.to_string()
        };

        // Process rows one by one
        for y in 0..height {
            for x in 0..width {
                let color = self.current_state.get(x, y).unwrap_or(Color32::TRANSPARENT);
                img.put_pixel(x as u32, y as u32, Rgba([color.r(), color.g(), color.b(), color.a()]));
            }
        }

        match img.save(&path_with_ext) {
            Ok(_) => {
                self.has_unsaved_changes = false;
                self.last_save_path = Some(path_with_ext);
                Ok(())
            },
            Err(e) => Err(format!("Erreur lors de la sauvegarde PNG: {}", e)),
        }
    }

    // Optimized texture update
    fn update_texture(&mut self, ctx: &egui::Context) {
        if self.texture_dirty {
            let width = self.current_state.width;
            let height = self.current_state.height;
            
            // Create the image data
            let mut image_data = vec![0_u8; width * height * 4];
            
            // Process all pixels
            for y in 0..height {
                for x in 0..width {
                    let color = if let Some(pixel) = self.current_state.get(x, y) {
                        pixel
                    } else {
                        let checker_x = x / CHECKERBOARD_SIZE;
                        let checker_y = y / CHECKERBOARD_SIZE;
                        if (checker_x + checker_y) % 2 == 0 {
                            Color32::from_gray(200)
                        } else {
                            Color32::from_gray(160)
                        }
                    };
                    
                    let idx = (y * width + x) * 4;
                    image_data[idx] = color.r();
                    image_data[idx + 1] = color.g();
                    image_data[idx + 2] = color.b();
                    image_data[idx + 3] = color.a();
                }
            }
            
            let color_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &image_data);
            self.texture = Some(ctx.load_texture("canvas", color_image, TextureOptions::NEAREST));
            
            self.texture_dirty = false;
        }
    }
    
    // Show save dialog
    fn show_save_dialog(&mut self, return_to_menu: bool) {
        self.save_dialog = SaveDialog::AskingSave { 
            return_to_menu
        };
    }
}

// Action à effectuer après fermeture de dialogues
enum PendingAction {
    None,
    ReturnToMenu,
    HandleLayerAction(LayerAction),
}

// Action à effectuer sur les calques
enum LayerAction {
    ToggleVisibility(usize),
    SetActive(usize),
    Edit(usize),
}

// Main application struct
struct MyApp {
    state: AppState,
    error_message: Option<String>,
    show_error: bool,
    new_layer_name: String,
    rename_layer_index: Option<usize>,
    rename_layer_name: String,
    pending_action: PendingAction,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu(MainMenu::default()),
            error_message: None,
            show_error: false,
            new_layer_name: "New Layer".to_string(),
            rename_layer_index: None,
            rename_layer_name: String::new(),
            pending_action: PendingAction::None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process keyboard shortcuts
        let ctrl = ctx.input(|i| i.modifiers.ctrl);
        let shift = ctx.input(|i| i.modifiers.shift);
        
        // Show error message dialog if needed
        if self.show_error {
            egui::Window::new("Erreur")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(self.error_message.as_deref().unwrap_or("Une erreur s'est produite"));
                    if ui.button("OK").clicked() {
                        self.show_error = false;
                    }
                });
        }
        
        // Process rename layer dialog
        if let Some(layer_idx) = self.rename_layer_index {
            egui::Window::new("Renommer le calque")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.rename_layer_name);
                    ui.horizontal(|ui| {
                        if ui.button("OK").clicked() && !self.rename_layer_name.is_empty() {
                            if let AppState::Canvas(paint_app) = &mut self.state {
                                paint_app.rename_layer(layer_idx, self.rename_layer_name.clone());
                            }
                            self.rename_layer_index = None;
                        }
                        if ui.button("Annuler").clicked() {
                            self.rename_layer_index = None;
                        }
                    });
                });
        }
        
        // Process pending actions
        match &self.pending_action {
            PendingAction::ReturnToMenu => {
                self.state = AppState::MainMenu(MainMenu::default());
                self.pending_action = PendingAction::None;
            },
            PendingAction::HandleLayerAction(action) => {
                if let AppState::Canvas(paint_app) = &mut self.state {
                    match action {
                        LayerAction::ToggleVisibility(idx) => {
                            paint_app.toggle_layer_visibility(*idx);
                        },
                        LayerAction::SetActive(idx) => {
                            paint_app.set_active_layer(*idx);
                        },
                        LayerAction::Edit(idx) => {
                            if let Some(layer) = paint_app.current_state.layers.get(*idx) {
                                self.rename_layer_index = Some(*idx);
                                self.rename_layer_name = layer.name.clone();
                            }
                        }
                    }
                }
                self.pending_action = PendingAction::None;
            },
            PendingAction::None => {}
        }
        
        match &mut self.state {
            AppState::MainMenu(menu) => {
                if let Some(action) = menu.show(ctx) {
                    match action {
                        main_menu::MenuAction::NewCanvas(width, height) => {
                            self.state = AppState::Canvas(PaintApp::new(width, height));
                        },
                        main_menu::MenuAction::OpenPng => {
                            if let Some(path) = FileDialog::new()
                                .add_filter("PNG Image", &["png"])
                                .set_directory("/")
                                .pick_file() {
                                match PaintApp::from_png_file(path.to_str().unwrap()) {
                                    Some(app) => self.state = AppState::Canvas(app),
                                    None => {
                                        self.error_message = Some("Impossible d'ouvrir l'image PNG".to_string());
                                        self.show_error = true;
                                    }
                                }
                            }
                        },
                        main_menu::MenuAction::OpenRustiq => {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Rustique File", &["rustiq"])
                                .set_directory("/")
                                .pick_file() {
                                match fs::read_to_string(&path) {
                                    Ok(content) => {
                                        match serde_json::from_str::<RustiqueFile>(&content) {
                                            Ok(file) => {
                                                let mut app = PaintApp::from_rustiq_file(file);
                                                app.last_save_path = Some(path.to_string_lossy().to_string());
                                                self.state = AppState::Canvas(app);
                                            },
                                            Err(e) => {
                                                self.error_message = Some(format!("Erreur lors de la lecture du fichier Rustiq: {}", e));
                                                self.show_error = true;
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        self.error_message = Some(format!("Erreur lors de la lecture du fichier: {}", e));
                                        self.show_error = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            AppState::Canvas(paint_app) => {
                // Handle keyboard shortcuts
                if ctrl {
                    if ctx.input(|i| i.key_pressed(egui::Key::Z)) && !shift {
                        paint_app.undo();
                    }
                    if ctx.input(|i| i.key_pressed(egui::Key::Y)) || 
                        shift && ctx.input(|i| i.key_pressed(egui::Key::Z)) {
                        paint_app.redo();
                    }
                    if ctx.input(|i| i.key_pressed(egui::Key::S)) {
                        if let Some(_) = &paint_app.last_save_path {
                            match paint_app.quick_save() {
                                Ok(_) => {},
                                Err(e) => {
                                    self.error_message = Some(e);
                                    self.show_error = true;
                                }
                            }
                        } else {
                            // Show save dialog
                            if let Some(path) = FileDialog::new()
                                .add_filter("PNG Image", &["png"])
                                .add_filter("Rustique File", &["rustiq"])
                                .set_directory("/")
                                .save_file() {
                                let path_str = path.to_str().unwrap();
                                if path_str.ends_with(".png") {
                                    match paint_app.save_as_png(path_str) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            self.error_message = Some(e);
                                            self.show_error = true;
                                        }
                                    }
                                } else if path_str.ends_with(".rustiq") {
                                    match paint_app.save_as_rustiq(path_str) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            self.error_message = Some(e);
                                            self.show_error = true;
                                        }
                                    }
                                } else {
                                    match paint_app.save_as_rustiq(&format!("{}.rustiq", path_str)) {
                                        Ok(_) => {},
                                        Err(e) => {
                                            self.error_message = Some(e);
                                            self.show_error = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Handle save dialog
                match &mut paint_app.save_dialog {
                    SaveDialog::Hidden => {},
                    SaveDialog::AskingSave { return_to_menu } => {
                        let return_to_menu_val = *return_to_menu;
                        egui::Window::new("Sauvegarder les modifications?")
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.label("Voulez-vous sauvegarder les modifications?");
                                ui.horizontal(|ui| {
                                    if ui.button("Oui").clicked() {
                                        // Ouvrir la boîte de dialogue de sauvegarde
                                        let result = if let Some(path) = FileDialog::new()
                                            .add_filter("Rustique File", &["rustiq"])
                                            .add_filter("PNG Image", &["png"])
                                            .set_directory("/")
                                            .save_file() {
                                            let path_str = path.to_str().unwrap();
                                            if path_str.ends_with(".png") {
                                                paint_app.save_as_png(path_str)
                                            } else {
                                                let path_with_ext = if !path_str.ends_with(".rustiq") {
                                                    format!("{}.rustiq", path_str)
                                                } else {
                                                    path_str.to_owned()
                                                };
                                                paint_app.save_as_rustiq(&path_with_ext)
                                            }
                                        } else {
                                            // L'utilisateur a annulé la boîte de dialogue de sauvegarde
                                            Ok(())
                                        };
                                        
                                        match result {
                                            Ok(_) => {
                                                paint_app.save_dialog = SaveDialog::Hidden;
                                                if return_to_menu_val {
                                                    self.pending_action = PendingAction::ReturnToMenu;
                                                }
                                            },
                                            Err(e) => {
                                                self.error_message = Some(e);
                                                self.show_error = true;
                                            }
                                        }
                                    }
                                    if ui.button("Non").clicked() {
                                        paint_app.save_dialog = SaveDialog::Hidden;
                                        if return_to_menu_val {
                                            self.pending_action = PendingAction::ReturnToMenu;
                                        }
                                    }
                                    if ui.button("Annuler").clicked() {
                                        paint_app.save_dialog = SaveDialog::Hidden;
                                    }
                                });
                            });
                    }
                }
                
                paint_app.update_texture(ctx);

                egui::SidePanel::left("layers_panel").show(ctx, |ui| {
                    ui.set_min_width(180.0);
                    ui.vertical(|ui| {
                        ui.heading("Calques");
                        ui.separator();
                        
                        // Layer controls
                        ui.horizontal(|ui| {
                            if ui.button("+").clicked() {
                                paint_app.add_layer(format!("Calque {}", paint_app.current_state.layers.len() + 1));
                            }
                            if ui.button("-").clicked() && paint_app.current_state.layers.len() > 1 {
                                paint_app.remove_layer(paint_app.current_state.active_layer_index);
                            }
                            ui.add_space(5.0);
                            if ui.button("↑").clicked() {
                                paint_app.move_layer_up(paint_app.current_state.active_layer_index);
                            }
                            if ui.button("↓").clicked() {
                                paint_app.move_layer_down(paint_app.current_state.active_layer_index);
                            }
                        });
                        
                        ui.separator();
                        
                        // Layer list - clone the data to avoid borrowing issues
                        // This avoids the conflict between immutable and mutable borrows
                        let layers_info: Vec<(usize, String, bool, bool)> = paint_app.current_state.layers
                            .iter()
                            .enumerate()
                            .map(|(i, layer)| (i, layer.name.clone(), layer.visible, i == paint_app.current_state.active_layer_index))
                            .collect();
                        
                        // Display the layers in reverse order (top layer first visually)
                        for (i, name, visible, is_active) in layers_info.iter().rev() {
                            ui.horizontal(|ui| {
                                let visible_text = if *visible { "👁" } else { "⊘" };
                                if ui.button(visible_text).clicked() {
                                    self.pending_action = PendingAction::HandleLayerAction(
                                        LayerAction::ToggleVisibility(*i)
                                    );
                                }
                                
                                if ui.selectable_label(*is_active, name).clicked() {
                                    self.pending_action = PendingAction::HandleLayerAction(
                                        LayerAction::SetActive(*i)
                                    );
                                }
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("✏").clicked() {
                                        self.pending_action = PendingAction::HandleLayerAction(
                                            LayerAction::Edit(*i)
                                        );
                                    }
                                });
                            });
                        }
                    });
                });

                egui::SidePanel::right("tools_panel").show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Outils");
                        if ui.button("Pinceau").clicked() {
                            paint_app.current_tool = Tool::Brush;
                        }
                        if ui.button("Gomme").clicked() {
                            paint_app.current_tool = Tool::Eraser;
                        }
                        if ui.button("Pot de Peinture").clicked() {
                            paint_app.current_tool = Tool::PaintBucket;
                        }
                        if ui.button("Pipette").clicked() {
                            paint_app.current_tool = Tool::ColorPicker;
                        }
                        if ui.button("Ligne").clicked() {
                            paint_app.current_tool = Tool::Line;
                        }
                        
                        ui.separator();
                        ui.label("Options de sauvegarde");
                        if ui.button("Sauvegarder PNG").clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("PNG Image", &["png"])
                                .set_directory("/")
                                .save_file() {
                                match paint_app.save_as_png(path.to_str().unwrap()) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        self.error_message = Some(e);
                                        self.show_error = true;
                                    }
                                }
                            }
                        }
                        if ui.button("Sauvegarder Rustiq").clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Rustique File", &["rustiq"])
                                .set_directory("/")
                                .save_file() {
                                let path_str = path.to_str().unwrap();
                                let path_with_ext = if Path::new(path_str).extension().is_none() {
                                    format!("{}.rustiq", path_str)
                                } else {
                                    path_str.to_owned()
                                };
                                match paint_app.save_as_rustiq(&path_with_ext) {
                                    Ok(_) => {},
                                    Err(e) => {
                                        self.error_message = Some(e);
                                        self.show_error = true;
                                    }
                                }
                            }
                        }
                        
                        ui.separator();
                        
                        ui.add_space(10.0);
                        ui.label("Taille du pinceau:");
                        ui.add(egui::DragValue::new(&mut paint_app.brush_size).speed(0.1).clamp_range(1..=500));
                        
                        ui.add_space(10.0);
                        ui.label("Taille de la gomme:");
                        ui.add(egui::DragValue::new(&mut paint_app.eraser_size).speed(0.1).clamp_range(1..=500));
                        
                        ui.add_space(10.0);
                        ui.label("Couleurs:");
                        ui.horizontal(|ui| {
                            ui.label("Primaire:");
                            ui.color_edit_button_srgba(&mut paint_app.primary_color);
                            if ui.button("+").clicked() {
                                paint_app.add_saved_color(paint_app.primary_color);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Secondaire:");
                            ui.color_edit_button_srgba(&mut paint_app.secondary_color);
                            if ui.button("+").clicked() {
                                paint_app.add_saved_color(paint_app.secondary_color);
                            }
                        });
                        
                        ui.add_space(10.0);
                        ui.label("Zoom:");
                        ui.add(egui::Slider::new(&mut paint_app.zoom, 0.1..=10.0).logarithmic(true));
                        
                        // Saved colors palette
                        if !paint_app.saved_colors.is_empty() {
                            ui.add_space(10.0);
                            ui.separator();
                            ui.label("Couleurs sauvegardées:");
                            
                            let available_width = ui.available_width();
                            let color_size = 24.0;
                            let colors_per_row = (available_width / color_size).floor() as usize;
                            let colors_count = paint_app.saved_colors.len();
                            
                            for i in 0..(colors_count / colors_per_row + (if colors_count % colors_per_row > 0 { 1 } else { 0 })) {
                                ui.horizontal(|ui| {
                                    for j in 0..colors_per_row {
                                        let idx = i * colors_per_row + j;
                                        if idx < colors_count {
                                            let color = paint_app.saved_colors[idx];
                                            let btn = ui.add(egui::Button::new("").fill(color).min_size(Vec2::new(color_size, color_size)));
                                            
                                            if btn.clicked() {
                                                paint_app.primary_color = color;
                                            }
                                            if btn.clicked_by(egui::PointerButton::Secondary) {
                                                paint_app.secondary_color = color;
                                            }
                                            if btn.clicked_by(egui::PointerButton::Middle) {
                                                paint_app.remove_saved_color(idx);
                                                break;
                                            }
                                            
                                            btn.on_hover_text("Clic gauche: définir comme couleur primaire\nClic droit: définir comme couleur secondaire\nClic molette: supprimer");
                                        }
                                    }
                                });
                            }
                        }
                    });
                });

                // Top panel for buttons
                let return_to_menu_clicked = egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    let mut clicked = false;
                    ui.horizontal(|ui| {
                        // Return to menu button
                        if ui.button("Retour au menu").clicked() {
                            clicked = true;
                        }
                        
                        if ui.button("Undo").clicked() {
                            paint_app.undo();
                        }
                        if ui.button("Redo").clicked() {
                            paint_app.redo();
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label("Clic droit: couleur secondaire | Ctrl+Z: Annuler | Ctrl+Y: Refaire | Ctrl+S: Sauvegarder");
                        });
                    });
                    clicked
                }).inner;
                
                // Handle the return to menu request after all panels to avoid borrowing issues
                if return_to_menu_clicked {
                    if paint_app.has_unsaved_changes {
                        paint_app.show_save_dialog(true);
                    } else {
                        self.pending_action = PendingAction::ReturnToMenu;
                    }
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    let available_size = ui.available_size();
                    let canvas_width = paint_app.current_state.width as f32;
                    let canvas_height = paint_app.current_state.height as f32;
                    let scale = (available_size.x / canvas_width).min(available_size.y / canvas_height);
                    let scaled_size = Vec2::new(canvas_width * scale * paint_app.zoom, canvas_height * scale * paint_app.zoom);
                    let canvas_rect = Rect::from_center_size(
                        ui.available_rect_before_wrap().center() + paint_app.pan,
                        scaled_size,
                    );

                    let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

                    if let Some(texture) = &paint_app.texture {
                        painter.image(texture.id(), canvas_rect, Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)), Color32::WHITE);
                    }

                    let to_canvas = egui::emath::RectTransform::from_to(
                        canvas_rect,
                        Rect::from_min_size(Pos2::ZERO, Vec2::new(canvas_width, canvas_height)),
                    );

                    // Improved panning with middle button
                    if response.dragged_by(egui::PointerButton::Middle) {
                        paint_app.pan += response.drag_delta();
                    }

                    // Handle line tool
                    if paint_app.current_tool == Tool::Line {
                        if response.clicked_by(egui::PointerButton::Primary) || response.clicked_by(egui::PointerButton::Secondary) {
                            let _is_secondary = response.clicked_by(egui::PointerButton::Secondary);
                            if let Some(pos) = response.interact_pointer_pos() {
                                let canvas_pos = to_canvas.transform_pos(pos);
                                paint_app.line_start = Some((canvas_pos.x as i32, canvas_pos.y as i32));
                                paint_app.is_drawing_line = true;
                            }
                        }
                        
                        if paint_app.is_drawing_line {
                            if let Some(pos) = response.hover_pos() {
                                let canvas_pos = to_canvas.transform_pos(pos);
                                paint_app.line_end = Some((canvas_pos.x as i32, canvas_pos.y as i32));
                                
                                // Draw a temporary line preview
                                if let (Some(start), Some(end)) = (paint_app.line_start, paint_app.line_end) {
                                    let start_pos = Pos2::new(
                                        start.0 as f32 * canvas_rect.width() / canvas_width + canvas_rect.min.x,
                                        start.1 as f32 * canvas_rect.height() / canvas_height + canvas_rect.min.y
                                    );
                                    let end_pos = Pos2::new(
                                        end.0 as f32 * canvas_rect.width() / canvas_width + canvas_rect.min.x,
                                        end.1 as f32 * canvas_rect.height() / canvas_height + canvas_rect.min.y
                                    );
                                    
                                    // Draw preview line (corrigé)
                                    let is_secondary = response.dragged_by(egui::PointerButton::Secondary);
                                    let color = if is_secondary { paint_app.secondary_color } else { paint_app.primary_color };
                                    let thickness = (paint_app.brush_size as f32 * paint_app.zoom).max(1.0);
                                    painter.line_segment([start_pos, end_pos], Stroke::new(thickness, color));
                                }
                            }
                            
                            if response.drag_released() {
                                if let (Some(start), Some(end)) = (paint_app.line_start, paint_app.line_end) {
                                    let is_secondary = response.drag_released_by(egui::PointerButton::Secondary);
                                    let color = if is_secondary { paint_app.secondary_color } else { paint_app.primary_color };
                                    paint_app.draw_line(start, end, color);
                                }
                                paint_app.is_drawing_line = false;
                                paint_app.line_start = None;
                                paint_app.line_end = None;
                                paint_app.save_state();
                            }
                        }
                    } else {
                        // Handle other tools with left/right mouse buttons
                        if (response.clicked_by(egui::PointerButton::Primary) || 
                            response.clicked_by(egui::PointerButton::Secondary)) && 
                           !response.clicked_by(egui::PointerButton::Middle) {
                            paint_app.is_drawing = true;
                            paint_app.save_state();
                        }

                        if (response.dragged() || response.clicked()) && 
                           !(response.dragged_by(egui::PointerButton::Middle) || 
                             response.clicked_by(egui::PointerButton::Middle)) {
                            if let Some(pos) = response.interact_pointer_pos() {
                                let canvas_pos = to_canvas.transform_pos(pos);
                                let x = canvas_pos.x as usize;
                                let y = canvas_pos.y as usize;
                                let is_secondary = response.dragged_by(egui::PointerButton::Secondary) || 
                                                 response.clicked_by(egui::PointerButton::Secondary);
                                
                                if x < paint_app.current_state.width && y < paint_app.current_state.height {
                                    match paint_app.current_tool {
                                        Tool::PaintBucket => paint_app.paint_bucket(x, y, is_secondary),
                                        Tool::ColorPicker => paint_app.pick_color(x, y, is_secondary),
                                        _ => {
                                            let (x, y) = (canvas_pos.x as i32, canvas_pos.y as i32);
                                            if let Some(last_pos) = paint_app.last_position {
                                                paint_app.draw_line(last_pos, (x, y), 
                                                                  if is_secondary { paint_app.secondary_color } 
                                                                  else { paint_app.primary_color });
                                            } else {
                                                paint_app.draw_point(x, y, is_secondary);
                                            }
                                            paint_app.last_position = Some((x, y));
                                        }
                                    }
                                    paint_app.is_drawing = true;
                                }
                            }
                        } else {
                            paint_app.save_state();
                            paint_app.last_position = None;
                        }
                    }

                    // Improved zooming with mouse wheel
                    let delta = ui.input(|i| i.scroll_delta.y);
                    if delta != 0.0 {
                        let zoom_speed = 0.001;
                        let old_zoom = paint_app.zoom;
                        paint_app.zoom *= 1.0 + delta * zoom_speed;
                        paint_app.zoom = paint_app.zoom.clamp(0.1, 10.0);
                        
                        // Zoom toward mouse cursor position
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let center = ui.available_rect_before_wrap().center();
                            let mouse_offset = mouse_pos - center - paint_app.pan;
                            let zoom_factor = paint_app.zoom / old_zoom;
                            paint_app.pan += mouse_offset * (1.0 - zoom_factor);
                        }
                    }
                });
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
        ..Default::default()
    };
    eframe::run_native(
        "Rustique Paint",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}