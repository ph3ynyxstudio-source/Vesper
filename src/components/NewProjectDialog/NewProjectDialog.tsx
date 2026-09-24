import { useEffect, useState } from "react";
import { createPortal } from "react-dom";
import "./NewProjectDialog.css";

export type NewProjectTargetReport = {
  path: string;
  operation: "create" | "associate" | "verify" | "skip";
  result:
    | "planned"
    | "created"
    | "already_existing_valid"
    | "ignored"
    | "blocked"
    | "failed"
    | "verified";
  message?: string;
};

export type NewProjectPreparationReport = {
  valid: boolean;
  can_create: boolean;
  project_name: string;
  project_path: string;
  project_exists: boolean;
  create_vesper_structure: boolean;
  project_structure_paths: string[];
  project_structure_collisions: string[];
  create_or_associate_chronos: boolean;
  chronos_path: string;
  chronos_directory_paths: string[];
  chronos_state: "absent" | "complete" | "incomplete" | "not_directory";
  missing_chronos_directories: string[];
  targets: NewProjectTargetReport[];
  blockers: string[];
};

export type NewProjectCreationReport = {
  success: boolean;
  partial: boolean;
  project_name: string;
  project_path: string;
  chronos_path: string;
  operations: NewProjectTargetReport[];
  errors: string[];
};

type RequestState = "idle" | "loading" | "ready" | "error";

type NewProjectDialogProps = {
  projectName: string;
  createVesperStructure: boolean;
  createOrAssociateChronos: boolean;
  projectsRoot: string;
  chronosRoot: string;
  preparationState: RequestState;
  preparation?: NewProjectPreparationReport;
  creationState: RequestState;
  creation?: NewProjectCreationReport;
  frontendError?: string;
  canCreate: boolean;
  onProjectNameChange: (value: string) => void;
  onCreateVesperStructureChange: (value: boolean) => void;
  onCreateOrAssociateChronosChange: (value: boolean) => void;
  onVerify: () => void;
  onCreate: () => void;
  onClose: () => void;
};

const operationLabels: Record<NewProjectTargetReport["operation"], string> = {
  create: "Créer",
  associate: "Associer",
  verify: "Vérifier",
  skip: "Ignorer",
};

const resultLabels: Record<NewProjectTargetReport["result"], string> = {
  planned: "Prévu",
  created: "Créé",
  already_existing_valid: "Déjà existant et valide",
  ignored: "Ignoré",
  blocked: "Bloqué",
  failed: "Échoué",
  verified: "Vérifié",
};

function readableBlocker(blocker: string) {
  if (blocker === "project_name_empty") return "Le nom du projet est requis.";
  if (blocker === "project_name_too_long") return "Le nom du projet est trop long.";
  if (blocker === "project_name_invalid_ending") {
    return "Le nom ne peut pas se terminer par un point ou un espace.";
  }
  if (blocker === "project_name_reserved_for_technical_directory") {
    return "Les noms commençant par un point sont réservés aux dossiers techniques.";
  }
  if (blocker === "project_name_invalid_character") {
    return "Le nom contient un caractère interdit par Windows.";
  }
  if (blocker === "project_name_not_single_component") {
    return "Le nom doit être un seul nom de dossier Windows.";
  }
  if (blocker === "project_name_reserved_by_windows") {
    return "Ce nom est réservé par Windows.";
  }
  if (blocker === "vesper_project_exists") return "Le projet VespΣr existe déjà.";
  if (blocker === "chronos_project_incomplete") {
    return "Le dossier Chr0 existe mais sa structure est incomplète.";
  }
  if (blocker === "chronos_project_not_directory") {
    return "La cible Chr0 existe mais n’est pas un dossier.";
  }
  if (blocker.startsWith("vesper_structure_collision:")) {
    return `Sous-dossier VespΣr déjà existant : ${blocker.slice("vesper_structure_collision:".length)}.`;
  }
  if (blocker.startsWith("invalid_structure_directory:")) {
    return "Un nom de sous-dossier VespΣr généré est invalide.";
  }
  if (blocker.startsWith("vesper_path_invalid:")) {
    return "Le chemin VespΣr calculé n’est pas autorisé.";
  }
  if (blocker.startsWith("chronos_path_invalid:")) {
    return "Le chemin Chr0 calculé n’est pas autorisé.";
  }
  if (blocker === "confirmation_required") return "La confirmation finale est requise.";

  return blocker;
}

function TargetList({ targets }: { targets: NewProjectTargetReport[] }) {
  if (targets.length === 0) {
    return <p className="new-project-dialog-empty">Aucune cible disponible.</p>;
  }

  return (
    <ul className="new-project-target-list">
      {targets.map((target, index) => (
        <li key={`${target.operation}-${target.path}-${index}`} data-result={target.result}>
          <div>
            <strong>{resultLabels[target.result]}</strong>
            <span>{operationLabels[target.operation]}</span>
          </div>
          <code>{target.path}</code>
          {target.message ? <p>{target.message}</p> : null}
        </li>
      ))}
    </ul>
  );
}

export function NewProjectDialog({
  projectName,
  createVesperStructure,
  createOrAssociateChronos,
  projectsRoot,
  chronosRoot,
  preparationState,
  preparation,
  creationState,
  creation,
  frontendError,
  canCreate,
  onProjectNameChange,
  onCreateVesperStructureChange,
  onCreateOrAssociateChronosChange,
  onVerify,
  onCreate,
  onClose,
}: NewProjectDialogProps) {
  const [isConfirmationOpen, setIsConfirmationOpen] = useState(false);
  const [isConfirmationReady, setIsConfirmationReady] = useState(false);
  const isCreating = creationState === "loading";
  const isVerifying = preparationState === "loading";
  const formLocked = isCreating || Boolean(creation) || isConfirmationOpen;
  const canCloseFromBackdrop = creationState === "idle" && !creation;
  const previewName = projectName || "NomDuProjet";
  const vesperPath = preparation?.project_path || `${projectsRoot}\\${previewName}`;
  const chronosPath = preparation?.chronos_path || `${chronosRoot}\\${previewName}`;

  useEffect(() => {
    function closeOnEscape(event: KeyboardEvent) {
      if (event.key === "Escape" && !isCreating) onClose();
    }

    document.addEventListener("keydown", closeOnEscape);
    return () => document.removeEventListener("keydown", closeOnEscape);
  }, [isCreating, onClose]);

  useEffect(() => {
    if (!canCreate) setIsConfirmationOpen(false);
  }, [canCreate]);

  useEffect(() => {
    if (!isConfirmationOpen) {
      setIsConfirmationReady(false);
      return;
    }

    const timer = window.setTimeout(() => setIsConfirmationReady(true), 400);
    return () => window.clearTimeout(timer);
  }, [isConfirmationOpen]);

  const creationSummary = creation
    ? creation.success
      ? "Réussite complète"
      : creation.partial
        ? "Réussite partielle"
        : "Échec"
    : undefined;

  return createPortal(
    <div
      className="new-project-dialog-backdrop"
      onPointerDown={(event) => {
        if (event.target === event.currentTarget && canCloseFromBackdrop) onClose();
      }}
    >
      <section
        className="new-project-dialog"
        role="dialog"
        aria-modal="true"
        aria-labelledby="new-project-dialog-title"
        aria-describedby="new-project-dialog-description"
      >
        <header>
          <div>
            <span>Initialisation locale</span>
            <h2 id="new-project-dialog-title">Nouveau projet</h2>
            <p id="new-project-dialog-description">
              Vérifie les chemins et les cibles avant toute écriture.
            </p>
          </div>
          <button
            className="new-project-dialog-close"
            type="button"
            onClick={onClose}
            disabled={isCreating}
            aria-label="Fermer le dialogue Nouveau projet"
          >
            ×
          </button>
        </header>

        <div className="new-project-dialog-body">
          <label className="new-project-name-field">
            <span>Nom du projet</span>
            <input
              type="text"
              value={projectName}
              onChange={(event) => onProjectNameChange(event.currentTarget.value)}
              disabled={formLocked}
              autoFocus
              autoComplete="off"
              spellCheck={false}
              placeholder="🎮MonJeu"
            />
          </label>

          <div className="new-project-options">
            <label>
              <input
                type="checkbox"
                checked={createVesperStructure}
                onChange={(event) =>
                  onCreateVesperStructureChange(event.currentTarget.checked)
                }
                disabled={formLocked}
              />
              <span>Créer la structure 01-02-05-99</span>
            </label>
            <label>
              <input
                type="checkbox"
                checked={createOrAssociateChronos}
                onChange={(event) =>
                  onCreateOrAssociateChronosChange(event.currentTarget.checked)
                }
                disabled={formLocked}
              />
              <span>Créer ou associer le dossier Chr0</span>
            </label>
          </div>

          <section className="new-project-paths" aria-labelledby="new-project-paths-title">
            <h3 id="new-project-paths-title">Aperçu des chemins</h3>
            <div>
              <span>VespΣr</span>
              <code>{vesperPath}</code>
            </div>
            <div>
              <span>Chr0</span>
              <code>{chronosPath}</code>
            </div>
          </section>

          <section aria-labelledby="new-project-targets-title">
            <div className="new-project-section-heading">
              <h3 id="new-project-targets-title">Cibles prévues</h3>
              <span>
                Création autorisée : {preparation ? (preparation.can_create ? "Oui" : "Non") : "Non vérifiée"}
              </span>
            </div>
            {preparation ? (
              <TargetList targets={preparation.targets} />
            ) : (
              <p className="new-project-dialog-empty">
                Utilise Vérifier pour obtenir la liste réelle des cibles.
              </p>
            )}
          </section>

          <section aria-labelledby="new-project-blockers-title">
            <h3 id="new-project-blockers-title">Blocages</h3>
            {preparation?.blockers.length ? (
              <ul className="new-project-blocker-list" role="alert">
                {preparation.blockers.map((blocker) => (
                  <li key={blocker}>{readableBlocker(blocker)}</li>
                ))}
              </ul>
            ) : (
              <p className="new-project-dialog-empty">
                {preparation ? "Aucun blocage détecté." : "Aucune vérification effectuée."}
              </p>
            )}
            {preparation?.missing_chronos_directories.length ? (
              <p className="new-project-missing-directories">
                Dossiers Chr0 manquants : {preparation.missing_chronos_directories.join(", ")}.
              </p>
            ) : null}
          </section>

          {creation ? (
            <section
              className="new-project-creation-report"
              data-outcome={creation.success ? "success" : creation.partial ? "partial" : "failure"}
              aria-labelledby="new-project-creation-title"
            >
              <div className="new-project-section-heading">
                <h3 id="new-project-creation-title">Rapport de création</h3>
                <strong>{creationSummary}</strong>
              </div>
              <TargetList targets={creation.operations} />
              {creation.errors.length ? (
                <ul className="new-project-creation-errors" role="alert">
                  {creation.errors.map((error, index) => (
                    <li key={`${error}-${index}`}>{readableBlocker(error)}</li>
                  ))}
                </ul>
              ) : null}
            </section>
          ) : null}

          {isConfirmationOpen && preparation ? (
            <section
              className="new-project-confirmation"
              aria-labelledby="new-project-confirmation-title"
            >
              <h3 id="new-project-confirmation-title">Confirmation finale</h3>
              <p>Confirme la création ou l’association aux chemins suivants :</p>
              <code>VespΣr : {preparation.project_path}</code>
              <code>Chr0 : {preparation.chronos_path}</code>
              <p>Aucune cible existante ne sera écrasée.</p>
            </section>
          ) : null}

          {isVerifying ? <p role="status">Vérification en cours…</p> : null}
          {isCreating ? <p role="status">Création et vérification en cours…</p> : null}
          {frontendError ? (
            <p className="new-project-frontend-error" role="alert">
              {frontendError}
            </p>
          ) : null}
        </div>

        <footer>
          <button type="button" className="secondary" onClick={onClose} disabled={isCreating}>
            Annuler
          </button>
          <button
            type="button"
            className="secondary"
            onClick={onVerify}
            disabled={
              isVerifying || isCreating || formLocked || projectName.length === 0
            }
          >
            {isVerifying ? "Vérification…" : "Vérifier"}
          </button>
          <button
            type="button"
            className="primary"
            onClick={() => {
              if (isConfirmationOpen) onCreate();
              else setIsConfirmationOpen(true);
            }}
            disabled={
              !canCreate ||
              isCreating ||
              (isConfirmationOpen && !isConfirmationReady)
            }
          >
            {isCreating
              ? "Création…"
              : isConfirmationOpen
                ? "Confirmer la création"
                : "Créer"}
          </button>
        </footer>
      </section>
    </div>,
    document.body,
  );
}
