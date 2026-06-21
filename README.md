# Vesper
FR : Cockpit de contexte local-first pour explorer les projets, la documentation et les connaissances de l’écosystème 
PH3YNYX.  EN : Local-first context cockpit for exploring projects, documentation and knowledge across the PH3YNYX ecosystem.

# ✧ VespΣr v0.1


### Cockpit visuel de PH3YNYX.OS

VespΣr est le point d'entrée rapide vers l'écosystème de projets locaux.

Son objectif est de répondre à trois questions en moins de cinq secondes :

* Où est mon projet ?
* Qu'est-ce qui est disponible ?
* Comment j'y accède ?

---

# Philosophie

| Aspect     | VespΣr                   | chr0                  |
| ---------- | ------------------------ | --------------------- |
| Rôle       | Naviguer et consulter    | Écrire et synthétiser |
| Opération  | Lecture                  | Création et édition   |
| Scope      | Contexte et accès rapide | Mémoire et évolution  |
| Principe   | Local First              | Local First           |
| Validation | Humaine                  | Humaine               |

---

## Ce que VespΣr n'est pas

VespΣr ne remplace pas :

* L'explorateur Windows
* Obsidian
* GitHub
* VS Code
* Un gestionnaire de tâches
* Une IA

---

## Ce que VespΣr fait

VespΣr se contente de :

* Lire le filesystem
* Découvrir les projets disponibles
* Détecter les dossiers clés
* Afficher le contexte d'un projet
* Afficher les documents importants
* Afficher les dernières sessions disponibles
* Fournir des accès rapides vers les outils associés

Le filesystem demeure toujours la source de vérité.

---

# Vision

VespΣr n'est pas un gestionnaire de projet.

VespΣr est un cockpit de contexte.

Il sert à rendre immédiatement visibles :

* la vision d'un projet
* sa documentation
* ses contextes
* ses ressources
* ses dernières sessions
* ses liens utiles

sans avoir à parcourir manuellement plusieurs dossiers.

---

# Relation avec chr0

Les deux projets sont complémentaires.

## chr0

Responsable de :

* Capturer les sessions
* Produire les synthèses
* Conserver la mémoire
* Structurer l'évolution

## VespΣr

Responsable de :

* Lire les contextes
* Afficher les synthèses existantes
* Explorer les projets
* Faciliter la navigation

```text
chr0 writes memory.
VespΣr reads memory.
```

---

# Stack

### Frontend

* React 19
* TypeScript
* Vite

### Desktop

* Tauri 2
* Rust

### Style

* Design Tokens partagés avec chr0

### État

* React Hooks
* Pas de Redux
* Pas de base de données

### Stockage

* Filesystem local
* Markdown
* JSON

---

# MVP v0.1

Objectifs :

* Découverte automatique des projets
* Navigation entre projets
* Lecture des contextes
* Lecture des documents importants
* Lecture des dernières sessions
* Détection de l'intégrité des dossiers
* Accès rapides vers VS Code, Terminal, Explorer et GitHub
* Interface cohérente avec PH3YNYX.OS

---

# Hors Scope

VespΣr ne vise pas à inclure :

* Chat IA
* Gestion de tâches
* Édition de fichiers
* Synchronisation cloud
* Score de productivité
* Analytics utilisateurs

---

# Structure d'un projet

```text
Project
│
├── Docs
├── Assets
├── App
├── Data
├── Exports
├── README.md
├── CONTEXT.md
└── vesper.json
```

---

# Métadonnées (vesper.json)

Chaque projet peut fournir un fichier optionnel :

```json
{
  "name": "Chr0nosV3rs",
  "icon": "↻",
  "color": "#7B5EA7",
  "description": "Session tracker desktop.",
  "status": "active"
}
```

Valeurs possibles :

```text
active
pause
concept
archived
```

Si aucun fichier n'est présent, VespΣr utilise automatiquement des valeurs par défaut.

---

# Principes de développement

* Local First
* Human First
* Read-Only MVP
* Filesystem First
* Simplicité avant automatisation
* Réutilisation des Design Tokens de chr0

---

# Architecture

```text
src/
 ├─ features/
 ├─ components/
 ├─ services/
 ├─ theme/
 └─ assets/

src-tauri/
 ├─ commands/
 ├─ domain/
 ├─ filesystem/
 └─ security/
```

---

# Roadmap

### v0.1

* Découverte des projets
* Lecture des contextes
* Détection d'intégrité
* Navigation rapide

### v0.5

* Recherche
* Dernières sessions chr0
* Résumés d'activité
* Raccourcis clavier

### v1.0

* Commandes personnalisées
* Couleurs dynamiques par projet
* Navigation avancée
* Intégration légère avec chr0

---

# Manifeste

```text
⌛ chr0 comprend

◑ Lun△rMood observe

Σ BΣTA explore

✦ Astr4lForge crée

✧ VespΣr navigue
```

---

### Une page. Une question. Une action.

VespΣr est conçu pour retrouver rapidement le bon contexte, au bon moment, sans détour.
