# Advanced Brush System - Documentation

## Nouveautés

Le système de pinceaux avancé remplace le système de pinceau simple par un ensemble complet de pinceaux artistiques avec des propriétés uniques.

## Types de pinceaux disponibles

1. **Round (Rond)** - Pinceau circulaire classique pour des traits précis
2. **Flat (Plat)** - Pinceau rectangulaire pour des traits larges et uniformes
3. **Bright** - Similaire au plat mais plus court, pour des traits structurés
4. **Filbert** - Pinceau avec pointe arrondie pour des transitions douces
5. **Fan (Éventail)** - Pour créer des textures et effets spéciaux
6. **Angle** - Pinceau biseauté pour des traits variés selon l'angle
7. **Mop** - Pinceau large et doux pour des zones diffuses
8. **Rigger** - Pinceau très fin pour les détails précis
9. **Custom (Personnalisé)** - Support pour pinceaux avec textures importées

## Propriétés des pinceaux

Chaque pinceau possède les propriétés suivantes :
- **Taille** : Ajustable via l'interface
- **Facteur d'étirement** : Ratio largeur/hauteur du pinceau
- **Sensibilité à l'angle** : Comment le pinceau réagit à la direction du trait
- **Dureté** : Dureté des bords (0.0 = très doux, 1.0 = dur)
- **Espacement** : Distance entre les points lors du tracé
- **Force de texture** : Intensité de la texture appliquée

## Interface utilisateur

- Le sélecteur de pinceaux apparaît dans le panneau d'outils à droite
- Cliquez sur un pinceau pour le sélectionner
- La taille du pinceau reste ajustable via le curseur existant
- Le pinceau actif est mis en évidence visuellement

## Architecture technique

Le système utilise :
- Un masque généré dynamiquement pour chaque type de pinceau
- Un calcul d'angle basé sur la direction du mouvement
- Une interpolation intelligente pour des lignes fluides
- Un système de cache pour les textures personnalisées

## Améliorations futures possibles

- Sensibilité à la pression (pour tablettes graphiques)
- Modes de fusion supplémentaires
- Import/export de pinceaux personnalisés
- Préréglages de pinceaux sauvegardables
- Aperçu en temps réel du trait
