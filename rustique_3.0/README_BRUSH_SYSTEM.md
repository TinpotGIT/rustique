# Advanced Brush System - Documentation

## 📚 Documentation Complète

Ce fichier donne un aperçu du système. Pour plus de détails :

- **[📖 Documentation Technique Complète](BRUSH_SYSTEM_DOCS.md)** - Architecture, API, développement
- **[👤 Guide Utilisateur](GUIDE_UTILISATEUR_PINCEAUX.md)** - Comment utiliser chaque pinceau  
- **[🔧 Architecture Technique](ARCHITECTURE_PINCEAUX.md)** - Structure du code, algorithmes

## Nouveautés v3.0

Le système de pinceaux avancé remplace le système de pinceau simple par un ensemble complet de pinceaux artistiques avec des formes réalistes et des comportements distincts.

### ✨ Améliorations Majeures
- **Traits fluides** avec espacement optimisé (0.02-0.08)
- **Formes distinctives** pour chaque type de pinceau
- **Bords nets** avec hardness = 1.0 par défaut
- **Performance optimisée** avec masques 7x7 pixels
- **Pinceaux personnalisés** avec détection automatique du type

## Types de pinceaux disponibles

1. **🔵 Round (Rond)** - Pinceau circulaire classique pour des traits précis
2. **▬ Flat (Plat)** - Pinceau rectangulaire pour des traits larges et uniformes  
3. **▬ Bright** - Similaire au plat mais plus court, pour des traits structurés
4. **🥚 Filbert** - Pinceau avec pointe arrondie pour des transitions douces
5. **🌿 Fan (Éventail)** - Pour créer des textures et effets spéciaux
6. **📐 Angle** - Pinceau biseauté pour des traits variés selon l'angle
7. **☁️ Mop** - Pinceau large et doux pour des zones diffuses
8. **| Rigger** - Pinceau très fin pour les détails précis
9. **🎨 Custom (Personnalisé)** - Support pour pinceaux avec textures importées

## Propriétés des pinceaux

Chaque pinceau possède les propriétés suivantes :
- **Taille** : 1-500 pixels, ajustable via l'interface
- **Facteur d'étirement** : Ratio largeur/hauteur du pinceau
- **Sensibilité à l'angle** : Comment le pinceau réagit à la direction du trait
- **Dureté** : Netteté des bords (0.1 = très doux, 1.0 = dur)
- **Espacement** : Distance entre les points lors du tracé (0.02-0.08)
- **Force de texture** : Intensité de la texture appliquée (pinceaux custom)

## Interface utilisateur

- **Palette visuelle** dans le panneau d'outils à droite
- **Aperçus distincts** pour chaque type de pinceau
- **Sélection par clic** sur l'aperçu désiré
- **Taille ajustable** via le curseur existant
- **Pinceaux personnalisés** avec bouton "Charger Texture"
- **Gestion avancée** avec suppression des pinceaux custom

## Architecture technique

Le système utilise :
- **Masques géométriques** générés dynamiquement pour chaque type
- **Calcul d'angle** basé sur la direction du mouvement
- **Formes procédurales** pour des pinceaux nets et distincts
- **Cache de textures** pour les pinceaux personnalisés
- **Optimisations de performance** (masques petits, seuils supprimés)

## 🚀 Utilisation Rapide

1. **Sélectionner** un pinceau dans la palette
2. **Ajuster** la taille avec le curseur
3. **Dessiner** avec clic gauche (couleur primaire) ou droit (secondaire)
4. **Créer** des pinceaux personnalisés avec "Charger Texture"

## 🔧 Pour les Développeurs

Le système est entièrement modulaire et extensible :
- Nouveau type → Ajouter à `BrushType` enum  
- Nouvelle forme → Implémenter dans `generate_brush_mask()`
- Nouvelles propriétés → Étendre `BrushProperties`

Voir **[ARCHITECTURE_PINCEAUX.md](ARCHITECTURE_PINCEAUX.md)** pour plus de détails techniques.

## Améliorations futures possibles

- Sensibilité à la pression (pour tablettes graphiques)
- Modes de fusion supplémentaires  
- Import/export de pinceaux personnalisés
- Préréglages de pinceaux sauvegardables
- Aperçu en temps réel du trait
