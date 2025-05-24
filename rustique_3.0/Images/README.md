# Icônes PNG pour Rustique Paint

Placez les fichiers PNG suivants dans ce dossier (`images/`) pour des icônes personnalisées. Si les fichiers PNG ne sont pas trouvés, l'application utilisera automatiquement des emojis de remplacement.

**Taille recommandée : 32x32 pixels (format PNG avec transparence)**

## Image de fond personnalisée

Placez un fichier `background.png` à la racine du projet (à côté de `Cargo.toml`) pour personnaliser l'arrière-plan du menu principal. L'image sera automatiquement redimensionnée pour couvrir tout l'écran.

## Outils principaux (Tools) :
- `brush_tool.png` - Icône pinceau (fallback: 🖌️)
- `eraser_tool.png` - Icône gomme (fallback: 🧹)
- `bucket_tool.png` - Icône seau de peinture (fallback: 🪣)
- `picker_tool.png` - Icône pipette à couleur (fallback: 🎨)
- `line_tool.png` - Icône outil ligne (fallback: 📏)

## Navigation :
- `home_icon.png` - Icône retour au menu (fallback: 🏠)
- `undo_icon.png` - Icône annuler (fallback: ↶)
- `redo_icon.png` - Icône refaire (fallback: ↷)
- `save_icon.png` - Icône sauvegarder (fallback: 💾)

## Layers (Calques) :
- `layer_visible.png` - Icône layer visible (fallback: 👁)
- `layer_hidden.png` - Icône layer caché (fallback: 🚫)
- `layer_add.png` - Icône ajouter layer (fallback: ➕)
- `layer_remove.png` - Icône supprimer layer (fallback: ➖)
- `layer_up.png` - Icône monter layer (fallback: ⬆)
- `layer_down.png` - Icône descendre layer (fallback: ⬇)
- `layer_edit.png` - Icône éditer layer (fallback: ✏️)

## Comment créer les icônes :

### Option 1 : Utiliser un éditeur d'images
1. Créez des images 32x32 pixels en PNG
2. Utilisez un fond transparent
3. Dessinez les icônes avec des couleurs contrastées (blanc/noir recommandé)
4. Sauvegardez avec les noms exacts listés ci-dessus

### Option 2 : Convertir des icônes existantes
1. Téléchargez des icônes libres de droits (ex: Feather Icons, Lucide, etc.)
2. Redimensionnez à 32x32 pixels
3. Convertissez en PNG avec transparence
4. Renommez selon la liste ci-dessus

### Option 3 : Utiliser des sources d'icônes gratuites
- **Feather Icons** : https://feathericons.com/
- **Lucide** : https://lucide.dev/
- **Heroicons** : https://heroicons.com/
- **Tabler Icons** : https://tabler-icons.io/

## Formats supportés :
- PNG (recommandé pour la transparence)
- Taille optimale : 32x32 pixels
- Couleurs : Noir/blanc pour un meilleur contraste
- Transparence supportée et recommandée

## Exemples de nommage :
```
images/
├── brush_tool.png      (icône pinceau)
├── eraser_tool.png     (icône gomme)  
├── bucket_tool.png     (icône seau)
├── picker_tool.png     (icône pipette)
├── line_tool.png       (icône ligne)
├── home_icon.png       (icône maison)
├── undo_icon.png       (icône annuler)
├── redo_icon.png       (icône refaire)
├── save_icon.png       (icône sauvegarder)
├── layer_visible.png   (icône œil ouvert)
├── layer_hidden.png    (icône œil barré)
├── layer_add.png       (icône plus)
├── layer_remove.png    (icône moins)
├── layer_up.png        (icône flèche haut)
├── layer_down.png      (icône flèche bas)
└── layer_edit.png      (icône crayon)
```

**Note :** Si vous ne placez pas d'icônes PNG, l'application fonctionnera parfaitement avec les emojis par défaut. Les icônes PNG sont optionnelles et servent à personnaliser l'apparence de l'interface.
