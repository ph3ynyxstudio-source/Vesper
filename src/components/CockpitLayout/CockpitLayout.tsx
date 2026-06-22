import type { ReactNode } from "react";
import "./CockpitLayout.css";

type CockpitLayoutProps = {
  navigation: ReactNode;
  projects: ReactNode;
  projectAccess: ReactNode;
  contextPanel: ReactNode;
};

export function CockpitLayout({
  navigation,
  projects,
  projectAccess,
  contextPanel,
}: CockpitLayoutProps) {
  return (
    <div className="cockpit-layout">
      <aside className="cockpit-layout-navigation">{navigation}</aside>

      <section className="cockpit-layout-projects" aria-labelledby="projects-title">
        {projects}
      </section>

      <div className="cockpit-layout-access">{projectAccess}</div>
      <div className="cockpit-layout-context">{contextPanel}</div>
    </div>
  );
}
