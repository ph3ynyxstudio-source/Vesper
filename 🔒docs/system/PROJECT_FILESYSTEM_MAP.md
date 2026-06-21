PROJECT_FILESYSTEM_MAP

Objectif

Définir comment VespΣr comprend la structure d'un projet local.

Ce document sert à cadrer la découverte des projets, des dossiers clés et des documents importants.

---

Principe

Le filesystem est la source de vérité.

VespΣr ne crée pas la structure d'un projet.

Il observe ce qui existe déjà et l'affiche clairement.

---

Racine des projets

La racine principale prévue est :

C:\Ph3yNyx.OS\Devs

VespΣr doit pouvoir découvrir les projets présents dans cette racine.

La racine exacte pourra être configurable plus tard.

---

Projet détectable

Un projet peut être détecté si un dossier contient au moins un des éléments suivants :

README.md
docs/
src/
src-tauri/
package.json
vesper.json
.git/

Le MVP peut commencer par une détection simple basée sur les dossiers présents dans la racine définie.

---

Dossiers clés

VespΣr détecte les dossiers suivants :

Docs/
Assets/
App/
Data/
Exports/

ou leurs variantes techniques :

docs/
assets/
src/
data/
dist/

---

Rôle des dossiers

Dossier| Rôle
Docs / docs| Documentation
Assets / assets| Ressources visuelles ou fichiers associés
App / src| Code applicatif
Data / data| Données locales ou exemples
Exports / dist| Sorties, builds ou fichiers exportés

---

Badges d'intégrité

La présence ou l'absence de ces dossiers peut être affichée sous forme de badges.

Ces badges indiquent uniquement ce qui existe.

Ils ne doivent pas devenir :

- un score ;
- une note ;
- un jugement ;
- une mesure de productivité.

---

Métadonnées

Si un fichier existe :

vesper.json

VespΣr peut l'utiliser pour enrichir l'affichage du projet.

Si ce fichier est absent, VespΣr utilise des valeurs par défaut.

---

Documents importants

VespΣr peut détecter :

README.md
CONTEXT.md
VISION.md
PRODUCT_SPEC.md
ARCHITECTURE.md
ROADMAP.md

Ces documents peuvent être affichés dans la vue contexte du projet.

---

Relation avec chr0

Si un projet contient des sessions ou synthèses produites par chr0, VespΣr peut les afficher.

VespΣr ne doit pas les modifier ni les générer.

---

Décisions connues

- Détection locale uniquement
- Lecture seule dans le MVP
- Aucun déplacement automatique
- Aucun renommage automatique
- Aucun nettoyage automatique

---

À préciser plus tard

- Racines multiples
- Projets favoris
- Projets archivés
- Détection des projets hors dossier principal
- Gestion des chemins cassés
- Priorité entre "vesper.json" et les valeurs détectées