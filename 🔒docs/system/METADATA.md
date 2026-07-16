metadata

Objectif

Définir les métadonnées optionnelles qu'un projet peut exposer à VespΣr.

Les métadonnées permettent d'améliorer l'affichage sans modifier la structure réelle du projet.

---

Principe

Les métadonnées sont facultatives.

Un projet doit pouvoir être affiché même en l'absence totale de métadonnées.

Le filesystem reste la source de vérité.

---

Fichier utilisé

VespΣr recherche un fichier optionnel :

vesper.json

placé à la racine du projet.

Exemple :

MyProject/
├── vesper.json
├── README.md
├── docs/
├── src/
└── ...

---

Format actuel

{
  "name": "Chr0nosV3rs",
  "icon": "↻",
  "color": "#7B5EA7",
  "description": "Session tracker desktop.",
  "status": "active",
  "repositoryUrl": "https://github.com/example/repo"
}

---

Champs supportés

name

Nom affiché dans l'interface.

{
  "name": "VespΣr"
}

---

icon

Identifiant d'une icône affichée dans les cartes et vues projet. La valeur est
choisie dans la bibliothèque SVG intégrée à VespΣr.

{
  "icon": "crystal-ball"
}

---

color

Couleur principale associée au projet.

{
  "color": "#7B5EA7"
}

---

description

Description courte du projet.

{
  "description": "Cockpit de contexte local-first."
}

---

status

État général du projet.

Valeurs prévues :

active
pause
concept
archived

---

repositoryUrl

Lien vers le dépôt Git.

{
  "repositoryUrl": "https://github.com/..."
}

---

Valeurs par défaut

Si "vesper.json" est absent :

{
  "name": "Nom du dossier",
  "icon": "📁",
  "status": "archived"
}

La couleur reste celle du thème par défaut.

---

Priorité

Ordre de priorité :

vesper.json
↓
détection automatique
↓
valeurs par défaut

---

Validation

Le backend Rust doit :

- vérifier la présence du fichier ;
- vérifier que le JSON est valide ;
- ignorer les champs inconnus ;
- utiliser les valeurs par défaut si nécessaire.

Une erreur de métadonnées ne doit jamais empêcher l'affichage du projet.

---

Version 1.0

VespΣr peut :

- lire "vesper.json"
- interpréter ses valeurs
- afficher les informations
- mettre à jour `status` et `icon` après une action explicite de l'utilisateur
- créer "vesper.json" si cette action l'exige et si le fichier est absent

VespΣr ne doit pas :

- supprimer "vesper.json"
- modifier silencieusement les autres champs

---

Relation avec le filesystem

Les métadonnées enrichissent l'affichage.

Elles ne remplacent jamais :

- la structure du projet ;
- les dossiers détectés ;
- les documents présents ;
- l'état réel du filesystem.

---

Décisions connues

- JSON simple
- Lecture seule
- Champs facultatifs
- Tolérance aux erreurs

---

À préciser plus tard

- Tags de projet
- Catégories
- Actions rapides personnalisées
- Couleurs dynamiques
- Icônes SVG personnalisées
- Métadonnées spécifiques à PH3YNYX.OS
