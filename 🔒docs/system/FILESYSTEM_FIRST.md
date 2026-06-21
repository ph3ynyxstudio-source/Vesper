FILESYSTEM_FIRST

Objectif

Définir le rôle du filesystem dans VespΣr.

Le filesystem est la source de vérité.

VespΣr est une couche de lecture, d'organisation visuelle et de navigation.

---

Principe

Un projet existe parce qu'il existe dans le filesystem.

VespΣr ne crée pas la structure du projet dans le MVP.

Il détecte ce qui existe déjà.

---

Dossiers clés détectés

VespΣr peut détecter :

Docs/
Assets/
App/
Data/
Exports/

Ces dossiers indiquent ce qui est présent dans un projet.

Ils ne sont pas un score de qualité.

---

Métadonnées optionnelles

Un projet peut contenir :

vesper.json

Ce fichier peut fournir :

- nom affiché ;
- icône ;
- couleur ;
- description ;
- statut ;
- lien repository.

Si le fichier est absent, VespΣr utilise des valeurs par défaut.

---

Règle importante

VespΣr ne doit pas modifier automatiquement :

- les dossiers du projet ;
- les documents ;
- les fichiers de configuration ;
- les fichiers produits par chr0.

---

À préciser plus tard

- Racine exacte des projets PH3YNYX.OS
- Nommage officiel des dossiers
- Gestion des projets absents ou déplacés
- Tolérance aux variantes de noms