architecture

Objectif

Décrire l'architecture générale de VespΣr.

Ce document définit les grandes responsabilités du système sans entrer dans les détails d'implémentation.

---

Vision

VespΣr est un cockpit de contexte.

Son objectif est de permettre à l'utilisateur de retrouver rapidement :

- ses projets ;
- leur documentation ;
- leurs ressources ;
- leurs dernières sessions ;
- leurs accès rapides.

VespΣr organise l'information existante.

Il ne devient pas le gestionnaire principal de cette information.

---

Architecture générale

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

---

Frontend

Stack :

React
TypeScript
Vite

Responsabilités :

- affichage des projets ;
- affichage des documents ;
- navigation ;
- recherche ;
- overlays de lecture ;
- actions rapides.

Le frontend ne lit jamais directement le filesystem.

Toute lecture passe par Tauri.

---

Backend

Stack :

Rust
Tauri

Responsabilités :

- découverte des projets ;
- validation des chemins ;
- lecture sécurisée des fichiers ;
- lecture des métadonnées ;
- ouverture d'applications externes ;
- exposition des commandes au frontend.

---

Communication

Communication via :

invoke("command_name", payload)

Le frontend ne connaît pas les détails du filesystem.

Le backend ne connaît pas les détails de l'interface.

---

Domaine principal

Concept central :

type ContextProject = {
  id: string;
  displayName: string;
  rootPath: string;
  description?: string;
  repositoryUrl?: string;
  status?: "active" | "pause" | "concept" | "archived";
};

Tous les écrans tournent autour de ce concept.

---

Documents

Concept secondaire :

type ContextDocument = {
  id: string;
  title: string;
  relativePath: string;
  kind: "readme" | "context" | "documentation" | "summary" | "session";
};

Les documents sont affichés en lecture seule.

---

Découverte

Le backend détecte les projets à partir du filesystem.

La découverte doit rester :

- simple ;
- rapide ;
- prévisible ;
- locale.

---

Sécurité

Toute lecture doit être limitée aux racines autorisées.

Aucune écriture automatique dans le MVP.

Aucune suppression dans le MVP.

Aucune modification de document dans le MVP.

---

Relation avec chr0

chr0 produit :

- sessions ;
- synthèses ;
- mémoire projet.

VespΣr lit ces informations.

VespΣr ne les génère pas.

---

Décisions connues

- React + TypeScript
- Tauri 2
- Rust
- Local First
- Filesystem First
- Read Only MVP

---

Hors scope actuel

- Synchronisation cloud
- Base de données
- IA conversationnelle
- Gestionnaire de tâches
- Analytics de productivité
- Éditeur Markdown

---

À préciser plus tard

- Structure exacte des commandes Rust
- Gestion du cache local
- Recherche globale
- Favoris
- Historique de navigation
- Permissions Tauri détaillées