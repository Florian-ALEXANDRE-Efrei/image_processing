# **Image Processing in Rust**

![Rust Image Processing](https://img.shields.io/badge/language-Rust-orange.svg)  

Un projet Rust pour appliquer plusieurs types de filtres d'image, notamment des filtres de détection de contours et des transformations morphologiques.


## **Caractéristiques**

- **Détection de contours** : Kirsch, Sobel, Robinson, Prewitt.
- **Opérateurs morphologiques** : Dilatation et érosion.
- **Outils généraux** : Inversion des couleurs, seuillage classique, etc.


## **Structure du projet**

```bash
image_processing/
├── src/
│   ├── main.rs                 # Point d'entrée principal
│   ├── filters/                # Contient les filtres
│   │   ├── edge_detection.rs   # Détection de contours (Kirsch, Sobel, etc.)
│   │   └── morphological.rs    # Opérateurs morphologiques (dilatation, érosion)
│   ├── utils/
│   │   └── image_utils.rs      # Fonctions utilitaires (chargement d'images_src, etc.)
│   └── lib.rs                  # Déclaration des modules
├── images_src/                 # Images d'entrée
├── images_res/                 # Images générées après traitement
└── Cargo.toml                  # Configuration du projet
```


## **Installation**

### Prérequis
1. [Rust](https://www.rust-lang.org/tools/install) (version stable recommandée).
2. [Cargo](https://doc.rust-lang.org/cargo/), l'outil de gestion de paquets pour Rust.
3. La bibliothèque **image** est utilisée pour la gestion des fichiers image. Assurez-vous qu'elle est incluse dans `Cargo.toml`.

### Étapes d'installation
1. Clonez ce dépôt :
   ```bash
   git clone https://github.com/Florian-ALEXANDRE-Efrei/image_processing.git
   ```
2. Accédez au répertoire du projet :
   ```bash
   cd image_processing
   ```
3. Compilez le projet :
   ```bash
   cargo build --release
   ```


## **Utilisation**

1. Ajoutez vos images sources dans le dossier `images_sources/`.
2. Exécutez le programme en ligne de commande :
   ```bash
   cargo run --release
   ```
   Exemple :
    - L'image source `images_sources/input.png` sera traitée avec les filtres définis dans le code.
    - Les images résultantes seront sauvegardées dans `images_resultat/`.

3. Modifiez le fichier `main.rs` pour expérimenter différents filtres ou traitements :

## **Exemples de filtres appliqués**

| Filtre           | Image Originale            | Image Résultante          |
|--------------------|----------------------------|----------------------------|
| **Prewitt**      | ![Input](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_src/Bureau.png) | ![Output](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_res/bureau_prewitt.png) |
| **Thresholding** | ![Input](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_res/bureau_prewitt.png) | ![Output](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_res/bureau_thresholding.png) | 
| **Dilatation**   | ![Input](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_res/bureau_thresholding.png) | ![Output](https://raw.githubusercontent.com/Florian-ALEXANDRE-Efrei/image_processing/refs/heads/dev/images_res/bureau_dilatee.png) |


## **Détails techniques des filtres et transformations**

### **1. Filtres de détection de contours**

- **Principe** :
  - Ce programme implémente une méthode pour appliquer des masques de convolution, tels que Prewitt, Sobel, et autres, afin de détecter les contours dans une image. Le processus repose sur l'application d'une matrice $3 \times 3$ (le masque de convolution) à chaque pixel de l'image pour calculer une nouvelle valeur d'intensité.
  - La valeur obtenue est mise en valeur absolue pour éviter des intensités négatives, puis normalisée à l'aide d'un facteur de normalisation calculé à l'avance. Ce facteur est basé sur la somme des coefficients positifs (ou négatifs) du masque de convolution. Cela garantit que les valeurs finales restent dans une plage cohérente, facilitant l'affichage et l'analyse.

#### **a) Filtre de Prewitt**
- **Masques de convolution** :
  ```math
  G_x = 1/3
  \begin{bmatrix}
  -1 & 0 & 1 \\
  -1 & 0 & 1 \\
  -1 & 0 & 1
  \end{bmatrix},
  \quad
  G_y = 1/3
  \begin{bmatrix}
  -1 & -1 & -1 \\
  0 &  0 &  0 \\
  1 &  1 &  1
  \end{bmatrix}
  ```
- **Utilisation**
```rust
// main.rs
let img = image_utils::get_image("images_src/Bureau.png"); // Ouverture de l'image

let img_prewitt = edge_detection::prewitt(img.clone()); // Application du filtre de Prewitt
img_prewitt.save("images_res/bureau_prewitt.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```
#### **b) Filtre de Sobel**
- **Masques de convolution** :
  ```math
  G_x = 1/4
  \begin{bmatrix}
  -1 & 0 & 1 \\
  -2 & 0 & 2 \\
  -1 & 0 & 1
  \end{bmatrix},
  \quad
  G_y = 1/4
  \begin{bmatrix}
  -1 & -2 & -1 \\
  0 &  0 &  0 \\
  1 &  2 &  1
  \end{bmatrix}
  ```

- **Utilisation**
```rust
// main.rs
let img = image_utils::get_image("images_src/Bureau.png"); // Ouverture de l'image

let img_sobel = edge_detection::sobel(img.clone()); // Application du filtre de Sobel
img_sobel.save("images_res/bureau_sobel.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```

#### **c) Filtre de Kirsch**
- **Masques de convolution**
  ```math
  G_x = 1/15
  \begin{bmatrix}
  -3 & -3 &  5 \\
  -3 &  0 & 5 \\
  -3 & -3 & 5
  \end{bmatrix},
  \quad
  G_{45°} = 1/15
  \begin{bmatrix}
  -3 & 5 &  5 \\
  -3 &  0 & 5 \\
  -3 & -3 & -3
  \end{bmatrix}
  ```
  ``` math
  G_{-45°} = 1/15
  \begin{bmatrix}
  -3 &  -3 &  -3 \\
  -3 &  0 & 5 \\
  -3 & 5 & 5
  \end{bmatrix}, 
  \quad
  G_y = 1/15
  \begin{bmatrix}
   -3 &  -3 &  -3 \\
  -3 &  0 & -3 \\
  5 & 5 & 5
  \end{bmatrix}
  ```

- **Utilisation**
```rust
// main.rs
let img = image_utils::get_image("images_src/Bureau.png"); // Ouverture de l'image

let img_kirsch = edge_detection::kirsch(img.clone()); // Application du filtre de Kirsch
img_kirsch.save("images_res/bureau_kirsch.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```

#### **d) Filtre de Robinson**
- **Masques de convolution** :
  ```math
  G_{E} = 1/5
  \begin{bmatrix}
  -1 &  1 & 1 \\
  -1 &  2 & 1 \\
  -1 &  1 & 1
  \end{bmatrix},
  \quad
  G_{NE} = 1/5
  \begin{bmatrix}
  1 & 1 &  1 \\
  -1 &  2 & 1 \\
  -1 & -1 & 1
  \end{bmatrix},
  \quad
  G_{N} = 1/5
  \begin{bmatrix}
  1 &  1 &  1 \\
  1 &  -2 & 1 \\
  -1 & -1 & -1
  \end{bmatrix}
  ```
  ```math
  G_{NO} = 1/5
  \begin{bmatrix}
  1 &  1 &  1 \\
  1 &  -2 & -1 \\
  1 & -1 & -1
  \end{bmatrix},
  \quad
  G_{O} = 1/5
  \begin{bmatrix}
  1 &  1 & -1 \\
  1 & -2 & -1 \\
  1 &  1 & -1
  \end{bmatrix},
  \quad
  G_{SO} = 1/5
  \begin{bmatrix}
  1 & -1 &  -1 \\
  1&  -2 & -1 \\
  1& 1 & 1
  \end{bmatrix}
  ```
  ``` math
  G_{S} = 1/5
  \begin{bmatrix}
  -1 &  -1 &  -1 \\
  1  &  0 & 1 \\
  1  & 1 & 1
  \end{bmatrix},
  \quad
  G_{SE} = 1/5
  \begin{bmatrix}
  -1 &  -1 &  1 \\
  -1 &  -2 & 1 \\
  1  & 1 & 1
  \end{bmatrix}
  ```

- **Utilisation**
```rust
// main.rs
let img = image_utils::get_image("images_src/Bureau.png"); // Ouverture de l'image

let img_robinson = edge_detection::robinson(img.clone()); // Application du filtre de Robinson
img_robinson.save("images_res/bureau_robinson.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```

### **2. Transformations morphologiques**

- **Principe** :
  - Les transformations morphologiques, comme la dilatation et l’érosion, sont utilisées pour modifier la structure des objets dans une image binaire. Dans ce projet, les pixels noirs représentent les pixels "objets" et les pixels blancs les pixels "fond".
  - Ces opérations s'appuient sur un élément structurant, une petite matrice (ou motif) qui définit la forme et la taille de l'opération. Cette matrice est appliquée à chaque pixel pour effectuer l'opération en tenant compte de ses voisins.
    - Dilatation : Cette opération agrandit les objets noirs en ajoutant des pixels objets autour d'eux.
    - Érosion : Cette opération réduit les objets noirs en éliminant les pixels objets en bordure.
  - Dans ce projet, seuls des éléments structurants $3 \times 3$ sont utilisés, mais leur contenu est modulaire, permettant de personnaliser la forme et le comportement des transformations.

#### **a) Érosion**
- **Algorithme** :
    1. Choisissez un élément structurant/gabarit (matrice $3 \times 3$).
    2. Pour chaque pixel de l'image :
       - On centre le gabarit et on applique l'érosion que si le pixel courant est un pixel objet.
       - Si **tous** les pixels objets du gabarit correspondent aux pixels objets du pixel courant et de son voisinage alors, le pixel central reste un pixel objet. Sinon, il devient un pixel fond.
    3. Parcourez toute l'image.

- **Utilisation** :
```rust
// main.rs
img_threshold.save("images_res/bureau_thresholding.png").expect("Failed to save image"); // Enregistrement de l'image résultante

let gabarit = [ // création du gabarit pour la dilatation et l'érosion
[image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
[image_utils::BLACKPIXEL, image_utils::BLACKPIXEL, image_utils::BLACKPIXEL],
[image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
];

let img_erodee = morphological::erosion(&gabarit, img_threshold); // Application de l'érosion
img_erodee.save("images_res/bureau_erodee.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```

#### **b) Dilatation**
- **Principe** : Agrandit les objets en remplacant des pixels objets par des pixels fond au bord des objets.
- **Algorithme** :
  1. Choisissez un élément structurant/gabarit (matrice $3 \times 3$).
  2. Pour chaque pixel de l'image :
     -  On centre le gabarit et on applique la dilatation que si le pixel courant est un pixel fond.
     - Si **au moins** un pixel objet du gabarit correspond à un pixel objet du pixel courant et de son voisinage alors, le pixel central devient un pixel objet. Sinon, il reste un pixel fond.
  3. Parcourez toute l'image.

- **Utilisation** :
```rust
// main.rs
img_threshold.save("images_res/bureau_thresholding.png").expect("Failed to save image"); // Enregistrement de l'image résultante

let gabarit = [ // création du gabarit pour la dilatation et l'érosion
[image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
[image_utils::BLACKPIXEL, image_utils::BLACKPIXEL, image_utils::BLACKPIXEL],
[image_utils::WHITEPIXEL, image_utils::BLACKPIXEL, image_utils::WHITEPIXEL],
];

let img_dilatee = morphological::dilatation(&gabarit, img_threshold.clone()); // Application de la dilatation
img_dilatee.save("images_res/bureau_dilatee.png").expect("Failed to save image"); // Enregistrement de l'image résultante
```

#### **c) Gabarit (élément structurant)**
Le choix du gabarit est crucial :
- **Carré $3 \times 3$** : Gabarit classique.
```math
  E =
  \begin{bmatrix}
  1 & 1 & 1 \\
  1 & 1 & 1 \\
  1 & 1 & 1
  \end{bmatrix}
```
- **Croix $3 \times 3$** : Évite les coins.
```math
  E =
  \begin{bmatrix}
  0 & 1 & 0 \\
  1 & 1 & 1 \\
  0 & 1 & 0
  \end{bmatrix}
```
  
## **Contribuer**

Les contributions sont les bienvenues ! Voici comment vous pouvez aider :
1. **Signalez des problèmes** : Utilisez l'onglet "Issues" de GitHub.
2. **Améliorez le code** : Clonez ce dépôt, faites vos modifications, et soumettez une pull request.
3. **Ajoutez des fonctionnalités** : Par exemple, d'autres filtres ou améliorations de performance.

## **Licence**

Ce projet est sous licence MIT. Consultez le fichier [LICENSE](LICENSE) pour plus d'informations.

### Notes supplémentaires

- Ce projet est basé sur les cours de Mr.Patrick J Bonnin, enseignat à l'Efrei Paris pour les cours de "Vision robotique et analyse" et "Vision et Perception de l'Environnement"
- Si vous avez des questions ou des problèmes, n'hésitez pas à ouvrir une issue ou à me contacter directement.
- Ce projet est conçu pour les personnes souhaitant explorer le traitement d'images en Rust et peut servir de base à des projets plus complexes.
