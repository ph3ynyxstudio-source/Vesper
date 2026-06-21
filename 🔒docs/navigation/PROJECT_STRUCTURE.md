PROJECT_STRUCTURE

Objectif

Permettre à un humain ou à une IA de retrouver rapidement les éléments importants du projet.

Ce document sert de carte d'orientation.

---

Vue d'ensemble

Vesper
│
├── docs/
├── src/
├── src-tauri/
├── public/
├── package.json
└── README.md

---

Documentation

docs/

Source principale de contexte.

Contient :

- règles ;
- architecture ;
- navigation ;
- documentation Codex ;
- décisions importantes.

Avant toute modification importante, consulter la documentation.

---

Frontend

src/

Contient l'interface React.

Responsabilités :

- affichage ;
- navigation ;
- composants ;
- interactions utilisateur.

Aucune lecture directe du filesystem.

---

Backend

src-tauri/

Contient le backend Rust.

Responsabilités :

- découverte des projets ;
- lecture des documents ;
- validation des chemins ;
- ouverture des outils externes.

---

Ressources publiques

public/

Contient les ressources statiques.

Exemples :

- icônes ;
- images ;
- assets publics.

---

Configuration

package.json

Configuration principale du projet frontend.

---

Documentation principale

README.md

Point d'entrée rapide du projet.

Présente :

- la vision ;
- le rôle ;
- le MVP ;
- la stack.

---

Où chercher selon le besoin

Comprendre la vision

README.md
docs/rules/
docs/system/

---

Comprendre l'architecture

docs/system/
src/
src-tauri/

---

Comprendre les règles

docs/rules/

---

Comprendre les conventions Codex

docs/codex/

---

Comprendre l'interface

docs/ui/
src/

---

Principe

Si une information existe déjà dans la documentation, elle doit être considérée avant de faire une hypothèse.

La documentation a priorité sur les suppositions.

---

À maintenir

Mettre à jour ce document lorsque :

- un dossier majeur est ajouté ;
- un dossier majeur est supprimé ;
- l'architecture change significativement.