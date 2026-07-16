security

Objectif

Définir les limites de sécurité du MVP VespΣr.

VespΣr est une application de lecture et de navigation.

Le principe de sécurité principal est de limiter l'accès au strict nécessaire.

---

Principe

VespΣr doit avoir accès uniquement aux informations nécessaires à son fonctionnement.

Le MVP privilégie :

- simplicité ;
- prévisibilité ;
- contrôle utilisateur ;
- permissions minimales.

---

Lecture autorisée

VespΣr peut lire :

- les projets détectés ;
- les fichiers Markdown ;
- les fichiers JSON de configuration ;
- les dossiers nécessaires à la découverte des projets.

---

Écriture interdite

Dans le MVP :

VespΣr ne doit pas :

- créer des fichiers ;
- modifier des fichiers ;
- supprimer des fichiers ;
- renommer des fichiers ;
- déplacer des fichiers.

Exception limitée :

VespΣr peut créer les sous-dossiers normalisés `01`, `02`, `05` et `99` dans
un projet existant, uniquement après une action et une confirmation explicites
de l'utilisateur. Le backend doit valider que le projet appartient à la racine
autorisée et refuser toute cible déjà existante avant la première création.

VespΣr peut aussi écrire les champs `status` et `icon` dans `vesper.json` après
une action explicite. L'identifiant d'icône doit être validé côté backend et
aucun autre champ ne doit être supprimé.

---

Limitation des chemins

Toute lecture doit être limitée aux racines configurées.

Exemple :

C:\Ph3yNyx.OS\Devs

Le backend Rust doit valider les chemins avant toute lecture.

---

Documents

Les documents sont affichés en lecture seule.

Aucun éditeur intégré n'est prévu dans le MVP.

---

Ouverture externe

VespΣr peut ouvrir :

- VS Code ;
- Terminal ;
- Explorer ;
- GitHub ;
- URLs externes.

Ces actions doivent être déclenchées explicitement par l'utilisateur.

Les boutons applicatifs peuvent ouvrir uniquement les destinations locales
explicitement validées pour Chr0nosV3rs et Plum3. Aucun exécutable alternatif
ou chemin de développement ne doit être utilisé comme fallback silencieux.

Sélection des sessions Chr0nosV3rs

Le sélecteur peut démarrer dans la racine locale de Chr0nosV3rs. Le backend
n'accepte comme source qu'un dossier projet directement enfant de cette racine.
Une source sélectionnée invalide ne doit déclencher aucun fallback silencieux.

---

Données utilisateur

Le MVP ne collecte pas :

- statistiques d'utilisation ;
- télémétrie ;
- analytics ;
- données cloud.

---

Relation avec chr0

VespΣr peut consulter les documents produits par chr0.

VespΣr ne doit pas les modifier.

---

Décisions connues

- Local First
- Read Only MVP
- Permissions minimales
- Aucune télémétrie

---

À préciser plus tard

- Permissions Tauri détaillées
- Gestion des favoris
- Historique local
- Cache éventuel
- Sandbox de lecture
