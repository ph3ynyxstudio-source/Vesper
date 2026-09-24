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

Écritures confirmées uniquement

Dans le MVP :

VespΣr reste en lecture seule pour les projets et fichiers existants, sauf pour
les actions de création explicitement déclenchées, prévisualisées et confirmées
par l'utilisateur. Aucun renommage, déplacement, écrasement ou suppression
implicite n'est autorisé.

VespΣr ne doit jamais :

- supprimer des fichiers ;
- renommer des fichiers ;
- déplacer des fichiers.

Exception limitée :

VespΣr peut créer, après préparation et confirmation, un nouveau
dossier directement sous la racine VespΣr autorisée, sa structure normalisée,
ainsi que le dossier Chr0 directement sous sa racine autorisée et ses cinq
sous-dossiers. Un dossier Chr0 existant et complet peut être associé sans être
modifié. Un dossier Chr0 incomplet doit bloquer cette partie de l'opération.

VespΣr peut aussi écrire les champs `status` et `icon` dans `vesper.json` après
une action explicite. L'identifiant d'icône doit être validé côté backend et
aucun autre champ ne doit être supprimé.

---

Limitation des chemins

Toute lecture doit être limitée aux racines configurées.

Racines d'écriture du flux Nouveau projet :

- `C:\Ph3yNyx.OS\05_⭐VESPΣR` ;
- `C:\Users\pheyr\AppData\Roaming\com.ph3yn.chronosvers\projects`.

Le backend Rust doit valider les chemins avant toute lecture ou écriture.

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
- Local First · Écritures confirmées
- Permissions minimales
- Aucune télémétrie

---

À préciser plus tard

- Permissions Tauri détaillées
- Gestion des favoris
- Historique local
- Cache éventuel
- Sandbox de lecture
