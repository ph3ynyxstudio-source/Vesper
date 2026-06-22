export const DESIGN_TOKENS = {
  colors: {
    bg: {
      app: "#030611",
      surface: "#060B18",
      card: "rgba(8, 14, 28, 0.72)",
      cardStrong: "rgba(10, 18, 36, 0.92)",
      cardGlow: "rgba(84, 214, 255, 0.12)",
      overlay: "rgba(3, 6, 17, 0.78)",
    },

    border: {
      soft: "rgba(84, 214, 255, 0.18)",
      strong: "rgba(84, 214, 255, 0.42)",
      purple: "rgba(168, 85, 247, 0.36)",
    },

    text: {
      primary: "#f4f7fb",
      secondary: "#9aa8c2",
      muted: "#66738c",
    },

    accent: {
      blue: "rgb(84, 214, 255)",
      blueSoft: "rgba(84, 214, 255, 0.16)",
      purple: "#a855f7",
      purpleSoft: "rgba(168, 85, 247, 0.16)",
    },

    status: {
      local: "#54d6ff",
      readOnly: "#a855f7",
      active: "#7dd3fc",
      paused: "#c084fc",
      archived: "#64748b",
    },
  },

  spacing: {
    xs: "4px",
    sm: "8px",
    md: "12px",
    lg: "16px",
    xl: "24px",
    "2xl": "32px",
  },

  radius: {
    sm: "8px",
    md: "12px",
    lg: "18px",
    xl: "24px",
    pill: "999px",
  },

  shadow: {
    card: "0 18px 50px rgba(0, 0, 0, 0.36)",
    glowBlue: "0 0 28px rgba(84, 214, 255, 0.18)",
    glowPurple: "0 0 28px rgba(168, 85, 247, 0.16)",
  },

  layout: {
    appMaxWidth: "1440px",
    sidebarWidth: "260px",
    contextPanelWidth: "360px",
    cockpitGap: "20px",
  },
} as const;

export type DesignTokens = typeof DESIGN_TOKENS;