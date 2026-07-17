import type { JSX } from "react";

function SvgWrapper({ children }: { children: JSX.Element | JSX.Element[] }) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      {children}
    </svg>
  );
}

export function HourglassIcon() {
  return (
    <SvgWrapper>
      <g transform="scale(0.333333)">
        <path strokeWidth="3" d="m31,34c-7.2725-1.9092-10-9.5454-10-14.3184v-6.6816" />
        <path strokeWidth="3" d="m21,59v-6.6816c0-4.7725,2.7275-12.4092,10-14.3184" />
        <path strokeWidth="3" d="m31,38c1-.2715,2-.8945,2-2,0-1.1045-1-1.7285-2-2" />
        <path strokeWidth="3" d="m41,34c7.2725-1.9092,10-9.5454,10-14.3184v-6.6816" />
        <path strokeWidth="3" d="m51,59v-6.6816c0-4.7725-2.7275-12.4092-10-14.3184" />
        <path strokeWidth="3" d="m41,38c-1-.2715-2-.8945-2-2,0-1.1045,1-1.7285,2-2" />
        <path strokeWidth="3" d="m25,55c6.0742,0,11-4.9258,11-11,0,6.0742,4.9258,11,11,11" />
        <path strokeWidth="3" d="m55,11c0,1.1001-.9004,2-2,2H19c-1.0996,0-2-.8999-2-2v-1c0-1.1001.9004-2,2-2h34c1.0996,0,2,.8999,2,2v1Z" />
        <line x1="47" x2="25" y1="18" y2="18" strokeWidth="3" />
        <path strokeWidth="3" d="m55,62c0,1.0996-.9004,2-2,2H19c-1.0996,0-2-.9004-2-2v-1c0-1.0996.9004-2,2-2h34c1.0996,0,2,.9004,2,2v1Z" />
        <line x1="36" x2="36" y1="39" y2="44" strokeWidth="3" />
      </g>
    </SvgWrapper>
  );
}

export function MoonIcon() {
  return (
    <SvgWrapper>
      <g transform="scale(0.333333)">
        <path strokeWidth="3" d="M7.3634,42.4095c4.5525,6.1703,11.874,10.1726,20.1303,10.1726c13.8071,0,25-11.1929,25-25 c0-8.5226-4.2646-16.0492-10.7763-20.5621c13.0383,2.8385,22.7812,14.4426,22.7812,28.3317c0,16.0163-12.9837,29-29,29 C21.9109,64.3517,10.5097,55.0229,7.3634,42.4095z" />
      </g>
    </SvgWrapper>
  );
}

export function PaintbrushIcon() {
  return (
    <SvgWrapper>
      <g transform="scale(0.333333)">
        <path strokeWidth="3" d="M45.8565,36.851c-7.0488,6.5155-12.4768,10.4313-15.8703,12.6073l-7.6389-7.6751c2.1789-3.3949,6.0918-8.8147,12.5932-15.8482 c4.7694-5.1598,9.7635-9.9137,13.8015-12.4485c2.9438-1.8479,6.597-1.5717,8.8659,0.6971l0,0 c2.2688,2.2688,2.5449,5.9222,0.697,8.8659C55.7702,27.0875,51.0162,32.0816,45.8565,36.851" />
        <path strokeWidth="3" d="M19.6432,44.3631c0,0-4.7911,2.4728-4.2087,6.1331c0.457,2.8713-0.8942,3.7226-3.2026,6.031 c-1.7206,1.7206,6.4887,3.4108,9.5042,1.8095s4.5544-3.1402,5.6852-6.1955" />
        <line x1="27.4304" x2="36.5691" y1="35.2569" y2="44.3958" strokeWidth="3" />
      </g>
    </SvgWrapper>
  );
}

export function VesperionIcon() {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.75"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      <path d="m12 4 2.4 5.1L20 12l-5.6 2.9L12 20l-2.4-5.1L4 12l5.6-2.9L12 4Z" />
      <path d="M15.75 6.75A8.5 8.5 0 0 1 20 12" />
      <path d="M8.25 17.25A8.5 8.5 0 0 1 4 12" />
      <circle cx="18.5" cy="8.5" r="1" />
    </svg>
  );
}
