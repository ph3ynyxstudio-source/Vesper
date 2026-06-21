governance


Toute propriété visuelle majeure doit être documentée
dans VISUAL_REFERENCE.md.

Les liens doivent pointer directement vers le fichier
et idéalement vers la ligne concernée.

L'objectif est de permettre à un humain ou à une IA
de retrouver rapidement les points d'entrée visuels
du projet.


Objectif


Définir comment les décisions sont prises dans le projet VespΣr.

VespΣr doit rester simple, lisible et contrôlé.

---

Principe central

Codex aide à construire le projet.

Codex ne décide pas du projet.

Les décisions finales appartiennent au concepteur.

---

Sources de vérité

Ordre de priorité :

1. Instructions explicites du concepteur
2. README du projet
3. Documentation dans "docs/"
4. Code existant
5. Suggestions de Codex

En cas de contradiction, Codex doit signaler l'incohérence au lieu de corriger automatiquement.

---

Changements interdits sans validation

Ne pas faire sans demande explicite :

- renommer des concepts ;
- déplacer des dossiers ;
- supprimer des fichiers ;
- fusionner des documents ;
- modifier la vision produit ;
- ajouter une dépendance ;
- créer une fonctionnalité hors MVP.

---

Réponse attendue de Codex

Chaque réponse doit préciser :

- ce qui a été compris ;
- ce qui a été modifié ;
- les fichiers concernés ;
- la validation effectuée ;
- les risques ;
- un commit FR ;
- un commit EN.

---

Décision actuelle

VespΣr v0.1 avance une étape à la fois.

La documentation précède l'implémentation.

---

À préciser plus tard

- Processus de validation des versions
- Structure des issues GitHub
- Règles de branches
- Format des releases