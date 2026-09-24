VespΣr

PH3YNYX.OS Context Cockpit

VespΣr est un cockpit de contexte local-first conçu pour naviguer rapidement dans un écosystème de projets sans remplacer le filesystem.

Son objectif est simple :

- retrouver les projets ;
- retrouver leur documentation ;
- retrouver leur contexte ;
- retrouver leurs ressources ;
- retrouver leurs dernières sessions ;
- accéder rapidement aux outils associés.

VespΣr n'est pas un gestionnaire de projet.

VespΣr est une couche d'orientation construite au-dessus du filesystem.

---

Philosophie

Local First

Les données restent sur la machine de l'utilisateur.

VespΣr ne dépend pas d'un service cloud pour fonctionner.

Filesystem First

Le filesystem est la source de vérité.

Les projets existent parce qu'ils existent sur le disque.

VespΣr observe cette structure et la rend plus facile à explorer.

Écritures confirmées V1

VespΣr reste en lecture seule pour les projets et fichiers existants, sauf pour
les actions de création explicitement déclenchées, prévisualisées et confirmées
par l'utilisateur.

VespΣr peut :

- découvrir les projets ;
- lire les documents ;
- afficher le contexte ;
- ouvrir des outils externes.

VespΣr ne doit pas :

- renommer, déplacer, écraser ou supprimer implicitement un projet ou un fichier ;
- modifier les documents existants ;
- produire automatiquement du contenu sans action explicite.

---

Relation avec chr0

chr0 produit :

- des sessions ;
- des synthèses ;
- de la mémoire projet.

VespΣr consulte cette information.

VespΣr ne la génère pas.

chr0
↓
écrit la mémoire

Filesystem
↓
source de vérité

VespΣr
↓
lit et navigue le contexte

---

V1 officielle

La V1 se concentre sur un objectif principal :

retrouver rapidement le contexte d'un projet.

Inclus

- découverte locale des projets ;
- affichage des métadonnées ;
- lecture des documents Markdown ;
- affichage du contexte ;
- affichage des dernières sessions ;
- navigation entre projets ;
- création confirmée d'un nouveau projet et de son dossier chr0 associé ;
- sélection locale de la source des sessions chr0 ;
- ouverture rapide :
  - VS Code ;
  - Explorateur ;
  - GitHub ;
  - Terminal ;
  - chr0 ;
  - Plum3.

Exclus

- gestionnaire de tâches ;
- base de données ;
- synchronisation cloud ;
- IA conversationnelle ;
- analytics ;
- édition Markdown ;
- génération automatique de contenu.

---

Architecture

Filesystem
     │
     ▼
Rust / Tauri
     │
     ▼
Services React
     │
     ▼
Interface utilisateur

Frontend

- React
- TypeScript
- Vite

Backend

- Rust
- Tauri 2

---

Structure du cockpit

Sidebar
↓
Projets

Centre
↓
Navigation et ressources

Panneau droit
↓
Contexte projet
↓
Sessions chr0
↓
Actions rapides

---

Stack

- React
- TypeScript
- Vite
- Tauri 2
- Rust

---

État actuel

Version :

v1.0.0

Statut :

Version officielle

---

Licence

À définir.
