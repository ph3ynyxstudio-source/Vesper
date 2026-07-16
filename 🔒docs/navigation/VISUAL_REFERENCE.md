VISUAL_REFERENCE

Objectif

Permettre de retrouver rapidement les éléments visuels importants du projet.

Chaque entrée doit contenir :

- le fichier ;
- la ligne ;
- le rôle ;
- l'impact visuel ;
- le lien direct.

---

Application

Couleur de fond principale

Description :

Couleur de fond globale de l'application.

Fichier :

src/theme/design_tokens.ts

Lien :

src/theme/design_tokens.ts#L12

Variable :

colors.bg.app

Impact :

Modifie le fond principal de l'application.

---

Couleur des cartes

Description :

Fond principal des cartes.

Fichier :

src/theme/design_tokens.ts

Lien :

src/theme/design_tokens.ts#L18

Variable :

colors.bg.card

Impact :

Modifie toutes les cartes utilisant les design tokens.

---

Largeur maximale du cockpit

Description :

Contrôle la largeur globale du layout.

Fichier :

src/features/cockpit/Cockpit.tsx

Lien :

src/features/cockpit/Cockpit.tsx#L84

Impact :

Modifie la largeur maximale de l'interface.

---

Carte projet

Description :

Composant principal affichant un projet.

Fichier :

src/components/ContextProjectCard/ContextProjectCard.tsx

Lien :

src/components/ContextProjectCard/ContextProjectCard.tsx#L1

Impact :

Toute modification affecte l'affichage des projets.

---

Génération de la structure projet

Description :

Bouton appliquant les dossiers normalisés au projet sélectionné et affichant
le résultat de l'opération.

Fichiers :

src/App.tsx
src/App.css

Impact :

Modifie les actions disponibles dans l'en-tête de la liste des projets.

---

Sélection de la source Chr0nosV3rs

Description :

Bouton du panneau « Fin de session » permettant d'associer un dossier projet
Chr0nosV3rs au projet VespΣr affiché.

Fichiers :

src/components/ContextPanel/ContextPanel.tsx
src/components/ContextPanel/ContextPanel.css

Impact :

Modifie la source Markdown affichée et copiable dans le panneau de session.

---

Lancement de Chr0 et Plum3

Description :

Bouton Chr0 placé dans l'en-tête « Fin de session » et bouton Plum3 placé en
bas à droite de la carte d'arbre généalogique.

Fichiers :

src/components/ContextPanel/ContextPanel.tsx
src/components/ProjectTree/ProjectTree.tsx

Impact :

Ajoute deux raccourcis visuels vers les applications locales associées.

---

Densité du panneau contexte

Description :

Le chemin du projet et la date de dernière session partagent une rangée afin
de réduire la hauteur du bloc de métadonnées.

Fichiers :

src/components/ContextPanel/ContextPanel.tsx
src/components/ContextPanel/ContextPanel.css

Impact :

Compacte la partie supérieure du panneau sans modifier les actions ni le
contenu Markdown.

---

Zone de fin de session

Description :

La carte de session utilise l'espace vertical restant du panneau. Le contenu
Markdown conserve son défilement avec des barres sombres adaptées au thème.

Fichier :

src/components/ContextPanel/ContextPanel.css

Impact :

Allonge l'aperçu de session et remplace l'apparence blanche des barres de
défilement natives.

---

Sélecteur d'icônes projet

Description :

Un clic sur la petite case d'icône d'une carte ouvre une bibliothèque locale
de 34 SVG. Le choix conserve la couleur néon de la carte.

Fichiers :

src/components/ProjectIconPicker/ProjectIconPicker.tsx
src/components/ProjectCard/ProjectCard.tsx

Impact :

Permet de personnaliser visuellement une carte sans ajouter de dépendance ni
charger la bibliothèque SVG externe au démarrage.
