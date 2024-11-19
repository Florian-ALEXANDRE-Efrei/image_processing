# **Image Processing in Rust**

![Rust Image Processing](https://img.shields.io/badge/language-Rust-orange.svg)  

Un projet Rust pour appliquer plusieurs types de filtres d'image, notamment des filtres de détection de contours et des transformations morphologiques.

## **Caractéristiques**

---

- **Détection de contours** : Kirsch, Sobel, Robinson, Prewitt.
- **Opérateurs morphologiques** : Dilatation et érosion.
- **Outils généraux** : Inversion des couleurs, seuillage classique, etc.

## **Structure du projet**

---

```bash
image_processing/
├── src/
│   ├── main.rs           # Point d'entrée principal
│   ├── filters/          # Contient les filtres
│   │   ├── edge_detection.rs  # Détection de contours (Kirsch, Sobel, etc.)
│   │   └── morphological.rs  # Opérateurs morphologiques (dilatation, érosion)
│   ├── utils/
│   │   └── image_utils.rs    # Fonctions utilitaires (chargement d'images_src, etc.)
│   └── lib.rs           # Déclaration des modules
├── images_src/     # Images d'entrée
├── images_res/     # Images générées après traitement
└── Cargo.toml           # Configuration du projet
```

## **Installation**

---

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

---

1. Ajoutez vos images sources dans le dossier `images_sources/`.
2. Exécutez le programme en ligne de commande :
   ```bash
   cargo run --release
   ```
   Exemple :
    - L'image source `images_sources/input.png` sera traitée avec les filtres définis dans le code.
    - Les images résultantes seront sauvegardées dans `images_resultat/`.

3. Modifiez le fichier `main.rs` pour expérimenter différents filtres ou traitements :
   Exemple :
   ```rust
   use image_processing::filters::edge_detection::sobel;
   use image_processing::utils::image_utils::{get_image, save_image};

   fn main() {
       let image = get_image("images_sources/input.png");
       let processed_image = sobel(&image);
       save_image("images_resultat/output.png", &processed_image);
       println!("Filtre Sobel appliqué !");
   }
   ```

## **Exemples de filtres appliqués**

---

| Filtre            | Image Originale            | Image Résultante          |
|--------------------|----------------------------|----------------------------|
| **Sobel**         | ![Input](images_sources/input.png) | ![Output](images_resultat/sobel.png) |
| **Kirsch**        | ![Input](images_sources/input.png) | ![Output](images_resultat/kirsch.png) |
| **Dilatation**    | ![Input](images_sources/input.png) | ![Output](images_resultat/dilatation.png) |


## **Détails techniques des filtres et transformations**

### **1. Filtres de détection de contours**

---
> ## Précisions requises
> Ces filtres utilisent des **masques de convolution**, qui sont appliqués sur chaque pixel d'une image pour détecter les variations d'intensité dans une direction particulière. Cela permet de détecter les contours. A chaque masque de convolution est appliqué un **facteur de normalisation** Ce facteur est calculé pour correspond à la somme des coefficients positifs ou négatifs du masque.

#### **a) Filtre de Prewitt**
- **Masques de convolution** :
  $$
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
  $$

#### **b) Filtre de Sobel**
- **Masques de convolution** :
  $$
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
  $$


#### **c) Filtre de Kirsch**
- **Masques de convolution**
  $$
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
  \end{bmatrix},
  \quad 
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
  $$

#### **d) Filtre de Robinson**
- **Masques de convolution** :
  $$
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
  \end{bmatrix},
  \quad
  G_{NO} = 1/5
  \begin{bmatrix}
  1 &  1 &  1 \\
  1 &  -2 & -1 \\
  1 & -1 & -1
  \end{bmatrix}
  $$
  $$
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
  \end{bmatrix},
  \quad
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
  $$

### **2. Transformations morphologiques**

---

Les transformations morphologiques comme la **dilatation** et l’**érosion** sont utilisées pour modifier la structure des objets dans une image binaire. Dans le contexte du projet, les pixels noires correspondent à des pixels objets et les pixels blancs à des pixels fond.

Pour la dailatation ou l'érosion, le principe est le même, nous utilisont un élément structurant/gabarit qui va permettre d'effectuer des opérations sur le pixel courant en fonction de ses pixels voisins. 

L'élément structurant/gabarit est une petite matrice (ou motif) utilisée pour définir la forme et la taille des opérations de dilatation et d'érosion. Dans le cas de mon projet, nous ne pouvons utiliser que des matrices 3x3 mais leur remplissage est modulaire.

#### **a) Érosion**
- **Principe** : Réduit les objets en supprimant les pixels objets du bord des objets.
- **Algorithme** :
    1. Choisissez un élément structurant/gabarit (matrice $3 \times 3$).
    
       Exemple (pixel fond = $0$, pixel objet = $1$):
       $
       E =
       \begin{bmatrix}
       0 & 1 & 0 \\
       1 & 1 & 1 \\
       0 & 1 & 0
       \end{bmatrix}
       $
    2. Pour chaque pixel de l'image :
       - On centre le gabarit et on applique l'érosion que si le pixel courant est un pixel objet.
       - Si **tous** les pixels objets du gabarit correspondent aux pixels objets du pixel courant et de son voisinage alors, le pixel central reste un pixel objet. Sinon, il devient un pixel fond.
    3. Parcourez toute l'image.

#### **b) Dilatation**
- **Principe** : Agrandit les objets en remplacant des pixels objets par des pixels fond au bord des objets.
- **Algorithme** :
  1. Choisissez un élément structurant/gabarit (matrice $3 \times 3$).

     Exemple (pixel fond = $0$, pixel objet = $1$):
     $
     E =
     \begin{bmatrix}
     0 & 1 & 0 \\
     1 & 1 & 1 \\
     0 & 1 & 0
     \end{bmatrix}
     $
  2. Pour chaque pixel de l'image :
     -  On centre le gabarit et on applique la dilatation que si le pixel courant est un pixel fond.
     - Si **au moins** un pixel objet du gabarit correspond à un pixel objet du pixel courant et de son voisinage alors, le pixel central devient un pixel objet. Sinon, il reste un pixel fond.
  3. Parcourez toute l'image.
  
#### **c) Gabarit (élément structurant)**
Le choix du gabarit est crucial :
- **Carré \(3 \times 3\)** : Gabarit classique.
  $$
  E =
  \begin{bmatrix}
  1 & 1 & 1 \\
  1 & 1 & 1 \\
  1 & 1 & 1
  \end{bmatrix}
  $$
- **Croix \(3 \times 3\)** : Évite les coins.
  $$
  E =
  \begin{bmatrix}
  0 & 1 & 0 \\
  1 & 1 & 1 \\
  0 & 1 & 0
  \end{bmatrix}
  $$

## **Contribuer**

---

Les contributions sont les bienvenues ! Voici comment vous pouvez aider :
1. **Signalez des problèmes** : Utilisez l'onglet "Issues" de GitHub.
2. **Améliorez le code** : Clonez ce dépôt, faites vos modifications, et soumettez une pull request.
3. **Ajoutez des fonctionnalités** : Par exemple, d'autres filtres ou améliorations de performance.

## **Licence**

---

Ce projet est sous licence MIT. Consultez le fichier [LICENSE](LICENSE) pour plus d'informations.

### Notes supplémentaires

---

Ce projet est basé sur les cours de Mr.Patrick J Bonnin.

- Si vous avez des questions ou des problèmes, n'hésitez pas à ouvrir une issue ou à me contacter directement.
- Ce projet est conçu pour les personnes souhaitant explorer le traitement d'images en Rust et peut servir de base à des projets plus complexes.

---

### Exemple final
- Ajoutez votre propre description ou personnalisez selon votre style ! 😊