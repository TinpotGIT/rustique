# 🎨 Système de Pinceaux Avancé - Documentation Technique

## Vue d'ensemble

Le système de pinceaux de Rustique Paint propose 8 types de pinceaux distincts, chacun avec ses propres caractéristiques géométriques et comportements. Ce système remplace les pinceaux circulaires basiques par des formes réalistes qui simulent de vrais outils artistiques.

## 📋 Table des Matières

1. [Types de Pinceaux](#types-de-pinceaux)
2. [Propriétés des Pinceaux](#propriétés-des-pinceaux)
3. [Architecture Technique](#architecture-technique)
4. [Pinceaux Personnalisés](#pinceaux-personnalisés)
5. [Guide d'Utilisation](#guide-dutilisation)
6. [Configuration Avancée](#configuration-avancée)

---

## Types de Pinceaux

### 🔵 Round (Rond)
**Utilisation :** Trait polyvalent, détails précis
- **Forme :** Cercle parfait
- **Caractéristiques :** 
  - Bords nets et uniformes
  - Pas de sensibilité à l'angle
  - Espacement très serré (0.05)
  - Hardness = 1.0 (bords francs)

```rust
// Calcul de la forme ronde
let dist = (rx * rx + ry * ry).sqrt();
value = (1.0 - dist).max(0.0);
```

### ▬ Flat (Plat)
**Utilisation :** Traits larges, remplissage de zones
- **Forme :** Rectangle très plat (ratio 4:1)
- **Caractéristiques :**
  - S'oriente selon la direction du mouvement
  - Idéal pour les aplats de couleur
  - Sensibilité à l'angle = 0.8

```rust
// Rectangle orienté selon l'angle de mouvement
if rx_rot.abs() <= 0.2 && ry_rot.abs() <= 1.0 {
    value = 1.0;
}
```

### ▬ Bright
**Utilisation :** Détails structurés, contours
- **Forme :** Rectangle moyennement plat (ratio 3:1)
- **Caractéristiques :**
  - Plus court que Flat mais plus structuré
  - Bords très nets (hardness = 1.0)
  - Bon compromis précision/couverture

### 🥚 Filbert
**Utilisation :** Transitions douces, modelé
- **Forme :** Ellipse arrondie
- **Caractéristiques :**
  - Bords légèrement adoucis (hardness = 0.8)
  - Forme ovale qui s'adapte à l'angle
  - Idéal pour les dégradés

```rust
// Forme elliptique
let ellipse_dist = (rx_rot * rx_rot) / (0.5 * 0.5) + (ry_rot * ry_rot) / (1.0 * 1.0);
if ellipse_dist <= 1.0 {
    value = (1.0 - ellipse_dist.sqrt()).max(0.0);
}
```

### 🌿 Fan (Éventail)
**Utilisation :** Textures, feuillages, effets spéciaux
- **Forme :** 5 "dents" distinctes en éventail
- **Caractéristiques :**
  - Crée des traits séparés naturellement
  - Espacement plus large (0.08) pour l'effet
  - Parfait pour simuler des branches, cheveux

```rust
// 5 segments d'éventail
for i in 0..5 {
    let segment_center = i as f32 * segment_width;
    let distance_from_segment = (normalized_angle - segment_center).abs();
    if distance_from_segment < segment_width * 0.4 {
        value = 1.0;
        break;
    }
}
```

### 📐 Angle (Biseauté)
**Utilisation :** Traits variés, calligraphie
- **Forme :** Biseautée asymétrique
- **Caractéristiques :**
  - Rotation de base de 45°
  - Crée des traits d'épaisseur variable
  - Sensibilité à l'angle = 0.8

### ☁️ Mop
**Utilisation :** Zones diffuses, arrière-plans
- **Forme :** Large et douce
- **Caractéristiques :**
  - Bords très doux (hardness = 0.5)
  - Espacement minimal (0.03) pour la douceur
  - Faible sensibilité à l'angle

### | Rigger
**Utilisation :** Détails fins, lignes précises
- **Forme :** Très fine et haute
- **Caractéristiques :**
  - Ratio inversé (0.5:1 - plus haut que large)
  - Espacement minimal (0.02) pour précision maximale
  - Bords ultra-nets (hardness = 1.0)

---

## Propriétés des Pinceaux

### 📏 Size (Taille)
- **Valeur :** 1-500 pixels
- **Effet :** Détermine la taille de base du pinceau
- **Note :** Se combine avec le `stretch_factor` pour la forme finale

### ↔️ Stretch Factor (Facteur d'étirement)
- **Valeur :** 0.5 à 4.0
- **Effet :** Ratio largeur/hauteur
  - `1.0` = Rond
  - `> 1.0` = Plus large que haut
  - `< 1.0` = Plus haut que large

### 🔄 Angle Sensitivity (Sensibilité à l'angle)
- **Valeur :** 0.0 à 1.0
- **Effet :** À quel point le pinceau s'oriente selon le mouvement
  - `0.0` = Pas d'orientation
  - `1.0` = Orientation maximale

### ⚪ Hardness (Dureté)
- **Valeur :** 0.1 à 1.0
- **Effet :** Netteté des bords
  - `1.0` = Bords francs
  - `0.5` = Bords doux
  - `0.1` = Très flou

### 📐 Spacing (Espacement)
- **Valeur :** 0.02 à 0.5
- **Effet :** Distance entre points lors du tracé
  - Plus petit = Trait plus fluide
  - Plus grand = Effet pointillé

### 🎨 Texture Strength (Force de texture)
- **Valeur :** 0.0 à 1.0
- **Effet :** Intensité des effets de texture
- **Note :** Principalement pour pinceaux personnalisés

---

## Architecture Technique

### Génération des Masques

Chaque pinceau génère un **masque de valeurs de 0.0 à 1.0** représentant l'opacité de chaque pixel :

```rust
pub fn generate_brush_mask(&self, size: usize) -> Vec<f32>
```

**Processus :**
1. **Calcul des coordonnées** relatives au centre (-1.0 à 1.0)
2. **Application de la rotation** selon l'angle de mouvement
3. **Test géométrique** selon le type de pinceau
4. **Application de la dureté** (optionnel)

### Système de Coordonnées

```
    (-1, -1) ---- (1, -1)
       |            |
       |   (0,0)    |     <- Centre du masque
       |            |
    (-1, 1) ----- (1, 1)
```

### Pipeline de Rendu

1. **Détection du mouvement** → Calcul de l'angle
2. **Génération du masque** → Selon le type de pinceau
3. **Application pixel par pixel** → Avec alpha blending
4. **Gestion de l'espacement** → Pour la fluidité

```rust
// Exemple d'application du masque
for dy in 0..mask_size as i32 {
    for dx in 0..mask_size as i32 {
        let mask_value = mask[(dy as usize) * mask_size + (dx as usize)];
        if mask_value > 0.0 {
            let alpha = (color.a() as f32 * mask_value) as u8;
            // Application du pixel...
        }
    }
}
```

---

## Pinceaux Personnalisés

### Création Automatique

Le système détecte automatiquement le type de texture selon le nom du fichier :

| Nom de fichier | Type généré | Caractéristiques |
|---|---|---|
| `*splatter*` | Éclaboussure | Motif aléatoire, spacing 0.4 |
| `*grunge*` | Rugueux | Texture rugueuse, hardness 0.4 |
| `*watercolor*` | Aquarelle | Effet fluide, hardness 0.2 |
| `*stipple*` | Pointillé | Motif en points, hardness 0.9 |

### Motifs Procéduraux

```rust
// Exemple : Motif d'éclaboussure
if texture_path.contains("splatter") {
    let noise = ((rx * 6.0).sin() * (ry * 6.0).cos()).abs();
    value = if noise > 0.3 { 1.0 } else { 0.0 };
}
```

### Interface Utilisateur

1. **Bouton "Charger Texture"** → Ouvre dialogue de fichier
2. **Détection automatique** → Configure les propriétés
3. **Gestion des pinceaux** → Liste avec suppression

---

## Guide d'Utilisation

### Sélection d'un Pinceau

1. **Panneau de droite** → Section "Sélection du pinceau"
2. **Grille visuelle** → Clic sur l'aperçu désiré
3. **Ajustement** → Taille avec le curseur

### Comportements par Type

| Pinceau | Meilleur pour | Technique |
|---------|---------------|-----------|
| **Round** | Détails, contours | Pression constante |
| **Flat** | Aplats, zones larges | Mouvements horizontaux |
| **Bright** | Détails structurés | Traits courts et nets |
| **Filbert** | Modelé, volumes | Mouvements courbes |
| **Fan** | Textures, feuillages | Tapotements légers |
| **Angle** | Calligraphie, variation | Rotation du poignet |
| **Mop** | Arrière-plans, flou | Grands mouvements |
| **Rigger** | Lignes fines, détails | Mouvements précis |

### Raccourcis

- **Clic gauche** : Couleur primaire
- **Clic droit** : Couleur secondaire
- **Taille** : Ajustable en temps réel
- **Angle** : Automatique selon le mouvement

---

## Configuration Avancée

### Propriétés par Défaut

```rust
impl Default for BrushProperties {
    fn default() -> Self {
        Self {
            spacing: 0.05,        // Fluidité maximale
            texture_strength: 0.0, // Pas de texture
            hardness: 1.0,        // Bords nets
            angle_sensitivity: 0.0, // Pas d'orientation
            // ...
        }
    }
}
```

### Optimisations

- **Masques optimisés** : 7x7 pixels maximum pour la performance
- **Seuils supprimés** : Rendu de tous les pixels (> 0.0)
- **Cache de textures** : Les images chargées sont mises en cache
- **Espacement intelligent** : Adapté à chaque type de pinceau

### Extensibilité

Le système est conçu pour être facilement extensible :

1. **Nouveau type** → Ajouter à l'enum `BrushType`
2. **Nouvelle forme** → Implémenter dans `generate_brush_mask`
3. **Nouvelles propriétés** → Ajouter à `BrushProperties`
4. **Nouveaux aperçus** → Modifier `draw_brush_preview`

---

## 🔧 Dépannage

### Problèmes Courants

**Traits hachurés/pointillés :**
- Vérifier `spacing` < 0.1
- S'assurer que `hardness` = 1.0 pour bords nets

**Pinceaux identiques :**
- Vérifier l'implémentation dans `generate_brush_mask`
- Contrôler les propriétés spécifiques au type

**Performance lente :**
- Réduire la taille des masques
- Optimiser les calculs dans les boucles

### Debug

```rust
// Afficher les propriétés du pinceau actif
println!("Brush: {:?}", brush_manager.active_brush());
```

---

## 📝 Notes de Développement

- **Coordinées normalisées** : Toujours entre -1.0 et 1.0
- **Masques carrés** : Taille toujours impaire pour centrage
- **Alpha blending** : Multiplicatif avec la couleur
- **Anti-aliasing** : Désactivé pour performance et netteté

---

*Documentation générée pour Rustique Paint v3.0*
*Système de pinceaux avancé - Décembre 2024*
