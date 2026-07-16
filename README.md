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

VespΣr est une couche de lecture et d'orientation construite au-dessus du filesystem.

---

Philosophie

Local First

Les données restent sur la machine de l'utilisateur.

VespΣr ne dépend pas d'un service cloud pour fonctionner.

Filesystem First

Le filesystem est la source de vérité.

Les projets existent parce qu'ils existent sur le disque.

VespΣr observe cette structure et la rend plus facile à explorer.

Read Only V1

La version 1.0 reste principalement en lecture seule.

VespΣr peut :

- découvrir les projets ;
- lire les documents ;
- afficher le contexte ;
- ouvrir des outils externes.

VespΣr ne doit pas :

- modifier les documents ;
- réécrire des fichiers ;
- supprimer des données ;
- produire automatiquement du contenu.

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
- génération confirmée de la structure normalisée d'un projet existant ;
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
