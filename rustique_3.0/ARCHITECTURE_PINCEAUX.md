# 🔧 Architecture Technique - Système de Pinceaux

## Structure du Code

```
src/brush_system/
├── mod.rs                 # Module principal
│   ├── BrushType          # Enum des types
│   ├── BrushProperties    # Propriétés d'un pinceau
│   ├── BrushManager       # Gestionnaire principal
│   └── generate_brush_mask # Génération des formes
```

## Flux d'Exécution

### 1. Sélection d'un Pinceau
```
Utilisateur clique → set_active_brush(index) → Propriétés chargées
```

### 2. Génération du Masque
```
Mouvement souris → update_angle() → generate_brush_mask() → Masque 2D
```

### 3. Application du Trait
```
Masque → draw_point() → record_change() → Pixels modifiés
```

## Types de Données Clés

### BrushType (Enum)
```rust
Round, Flat, Bright, Filbert, Fan, Angle, Mop, Rigger, Custom
```

### BrushProperties (Struct)
```rust
pub struct BrushProperties {
    pub size: f32,           // 1.0 - 500.0
    pub hardness: f32,       // 0.1 - 1.0  
    pub spacing: f32,        // 0.02 - 0.5
    pub angle_sensitivity: f32, // 0.0 - 1.0
    // ...
}
```

### Masque de Pinceau
```rust
Vec<f32> // Valeurs 0.0-1.0 représentant l'opacité
```

## Algorithmes Principaux

### Génération de Forme Ronde
```rust
let dist = (rx * rx + ry * ry).sqrt();
value = (1.0 - dist).max(0.0);
```

### Rotation selon l'Angle
```rust
let rx_rot = rx * cos_a - ry * sin_a;
let ry_rot = rx * sin_a + ry * cos_a;
```

### Application du Masque
```rust
for pixel in mask {
    if mask_value > 0.0 {
        alpha = color.a() * mask_value;
        draw_pixel(x, y, color_with_alpha);
    }
}
```

## Optimisations Appliquées

1. **Masques petits** : 7x7 max au lieu de 20x20
2. **Seuil à 0.0** : Tous les pixels sont rendus
3. **Espacement réduit** : 0.05 au lieu de 0.2
4. **Hardness = 1.0** : Bords nets par défaut
5. **Cache de textures** : Images chargées une seule fois

## Points d'Extension

### Ajouter un Nouveau Type
1. Ajouter à `BrushType` enum
2. Implémenter dans `generate_brush_mask()`
3. Ajouter l'aperçu dans `draw_brush_preview()`
4. Configurer les propriétés par défaut

### Modifier une Forme
Modifier le calcul dans le `match` de `generate_brush_mask()` :
```rust
BrushType::NouveauType => {
    // Votre calcul géométrique ici
    if condition_geometrique {
        value = 1.0;
    }
},
```

---
*Documentation technique pour développeurs*
