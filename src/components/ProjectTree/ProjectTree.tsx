import "./ProjectTree.css";

export type ProjectTreeSection = {
  name: string;
  items: string[];
};

type ProjectTreeProps = {
  projectName: string;
  sections: ProjectTreeSection[];
};

export function ProjectTree({ projectName, sections }: ProjectTreeProps) {
  return (
    <section className="project-tree" aria-labelledby="project-tree-title">
      <div className="project-tree-heading">
        <span>Carte du projet</span>
        <strong>Lecture seule</strong>
      </div>

      <div className="project-tree-root">
        <span className="project-tree-root-icon" aria-hidden="true">
          ◇
        </span>
        <h2 id="project-tree-title">{projectName}</h2>
      </div>

      <ul className="project-tree-sections">
        {sections.map((section) => (
          <li className="project-tree-section" key={section.name}>
            <div className="project-tree-folder">
              <span aria-hidden="true">▾</span>
              <strong>{section.name}</strong>
              <small>{section.items.length}</small>
            </div>

            <ul className="project-tree-items">
              {section.items.map((item) => (
                <li key={item}>{item}</li>
              ))}
            </ul>
          </li>
        ))}
      </ul>
    </section>
  );
}
