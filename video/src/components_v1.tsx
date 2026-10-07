import React from 'react';
import {AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig} from 'remotion';
import {C, display, ui} from './theme';

/** Springy scale/translate entrance. `delay` in frames. */
export const Pop: React.FC<{
  delay?: number;
  from?: number;
  rotate?: number;
  style?: React.CSSProperties;
  children: React.ReactNode;
}> = ({delay = 0, from = 0.6, rotate = 0, style, children}) => {
  const frame = useCurrentFrame();
  const {fps} = useVideoConfig();
  const s = spring({frame: frame - delay, fps, config: {damping: 11, stiffness: 160}});
  return (
    <div
      style={{
        opacity: interpolate(s, [0, 0.4], [0, 1], {extrapolateRight: 'clamp'}),
        transform: `scale(${interpolate(s, [0, 1], [from, 1])}) rotate(${interpolate(s, [0, 1], [rotate, 0])}deg)`,
        ...style,
      }}
    >
      {children}
    </div>
  );
};

/** Big display text; each word slams in on its own beat. */
export const Slam: React.FC<{
  words: {t: string; color?: string}[];
  size?: number;
  gap?: number; // frames between words
  delay?: number;
  align?: 'center' | 'left';
}> = ({words, size = 150, gap = 6, delay = 0, align = 'center'}) => (
  <div
    style={{
      display: 'flex',
      flexWrap: 'wrap',
      justifyContent: align === 'center' ? 'center' : 'flex-start',
      columnGap: size * 0.25,
      fontFamily: display,
      fontWeight: 800,
      fontSize: size,
      lineHeight: 1.02,
      letterSpacing: -size * 0.03,
      color: C.text,
    }}
  >
    {words.map((w, i) => (
      <Pop key={i} delay={delay + i * gap} from={1.5} rotate={i % 2 ? 4 : -4} style={{color: w.color ?? C.text}}>
        {w.t}
      </Pop>
    ))}
  </div>
);

/** Moody animated backdrop: drifting blue glow + faint grid. */
export const Backdrop: React.FC<{tint?: string}> = ({tint = C.blue}) => {
  const frame = useCurrentFrame();
  const x = 50 + Math.sin(frame / 40) * 18;
  const y = 45 + Math.cos(frame / 55) * 14;
  return (
    <AbsoluteFill
      style={{
        backgroundColor: C.bg,
        backgroundImage: `radial-gradient(circle at ${x}% ${y}%, ${tint}44 0%, transparent 45%),
          linear-gradient(${C.border}55 1px, transparent 1px), linear-gradient(90deg, ${C.border}55 1px, transparent 1px)`,
        backgroundSize: '100% 100%, 64px 64px, 64px 64px',
        backgroundPosition: `0 0, ${-frame}px ${-frame}px, ${-frame}px ${-frame}px`,
      }}
    />
  );
};

/** Frame wipe flash used on hard cuts. */
export const Flash: React.FC<{at?: number; color?: string}> = ({at = 0, color = '#fff'}) => {
  const frame = useCurrentFrame();
  const o = interpolate(frame - at, [0, 6], [0.9, 0], {extrapolateLeft: 'clamp', extrapolateRight: 'clamp'});
  return <AbsoluteFill style={{background: color, opacity: o, pointerEvents: 'none'}} />;
};

/** Stylized app window (not a screenshot — a recreation of the Tailorbird look). */
export const AppWindow: React.FC<{
  width?: number;
  height?: number;
  title?: string;
  style?: React.CSSProperties;
  children: React.ReactNode;
}> = ({width = 1100, height = 640, title = 'Tailorbird', style, children}) => (
  <div
    style={{
      width,
      height,
      background: C.panel,
      border: `1.5px solid ${C.border}`,
      borderRadius: 18,
      overflow: 'hidden',
      boxShadow: `0 40px 120px #000a, 0 0 0 1px #0006, 0 0 80px ${C.blue}22`,
      fontFamily: ui,
      color: C.text,
      display: 'flex',
      flexDirection: 'column',
      ...style,
    }}
  >
    <div style={{height: 46, background: C.bg, display: 'flex', alignItems: 'center', padding: '0 18px', gap: 8}}>
      {['#f87171', '#fbbf24', '#34d399'].map((c) => (
        <div key={c} style={{width: 12, height: 12, borderRadius: 6, background: c}} />
      ))}
      <div style={{marginLeft: 14, fontSize: 15, color: C.muted, fontWeight: 500}}>{title}</div>
    </div>
    <div style={{flex: 1, position: 'relative'}}>{children}</div>
  </div>
);

export const Cursor: React.FC<{x: number; y: number; press?: boolean}> = ({x, y, press}) => (
  <svg
    width={34}
    height={34}
    viewBox="0 0 24 24"
    style={{
      position: 'absolute',
      left: x,
      top: y,
      transform: `scale(${press ? 0.85 : 1})`,
      filter: 'drop-shadow(0 3px 4px #0008)',
      zIndex: 20,
    }}
  >
    <path d="M3 2l7 19 3-8 8-3z" fill="#fff" stroke="#000" strokeWidth={1.2} strokeLinejoin="round" />
  </svg>
);

export const Chip: React.FC<{color?: string; children: React.ReactNode}> = ({color = C.blue, children}) => (
  <span
    style={{
      fontFamily: ui,
      fontWeight: 600,
      fontSize: 15,
      padding: '5px 12px',
      borderRadius: 999,
      color,
      background: `${color}22`,
      border: `1px solid ${color}55`,
    }}
  >
    {children}
  </span>
);
